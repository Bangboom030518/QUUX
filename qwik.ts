/**
 * Set up event listening for browser.
 *
 * Determine all of the browser events and set up global listeners for them.
 * If browser triggers event search for the lazy load URL and `import()` it.
 *
 */

function qwikLoader(hasInitialized?: number) {
  const Q_CONTEXT = "__q_context__";
  const events = new Set();

  function broadcast(infix: string, event: Event, type = event.type) {
    document
      .querySelectorAll(`[on${infix}\\:${type}]`)
      .forEach((target) => dispatch(target, infix, event, type));
  }

  function resolveContainer(container: Element) {
    if ((container as any)["_qwikjson_"] !== undefined) return;

    const parentJSON =
      container === document.documentElement ? document.body : container;
    let script = parentJSON.lastElementChild;
    while (script) {
      if (
        script.tagName === "SCRIPT" &&
        script.getAttribute("type") === "qwik/json"
      ) {
        (container as any)["_qwikjson_"] = JSON.parse(
          script.textContent!.replace(/\\x3C(\/?script)/g, "<$1")
        );
        break;
      }
      script = script.previousElementSibling;
    }
  }

  const createEvent = (eventName: string, detail?: any) =>
    new CustomEvent(eventName, {
      detail,
    });

  async function dispatch(
    element: Element,
    onPrefix: string,
    event: Event,
    eventName = event.type
  ) {
    const attributeName = `on${onPrefix}:${eventName}`;
    if (element.hasAttribute("preventdefault:" + eventName))
      event.preventDefault();

    const context = (element as any)["_qc_"] as QContext | undefined;
    const listeners = context?.li.filter((listener) => listener[0] === attributeName);
    // There are already listeners to apply
    if (listeners && listeners.length > 0) {
      for (const listener of listeners) {
        await listener[1].getFn([element, event], () => element.isConnected)(
          event,
          element
        );
        // this => Element
        // (event) => { ... }
      }
      return;
    }
    // we need to add the listeners
    const attributeValue = element.getAttribute(attributeName);
    if (!attributeValue) return;
    const container = element.closest("[q\\:container]")!;
    const base = new URL(container.getAttribute("q:base")!, document.baseURI);

    for (const eventModuleUrl of attributeValue.split("\n")) {
      if (!element.isConnected) return;
      const url = new URL(eventModuleUrl, base);
      const symbolName = url.hash.replace(/^#?([^?[|]*).*$/, "$1") || "default";
      const requestTime = performance.now();
      resolveContainer(container);
      const handler = findSymbol(await import(url.href.split("#")[0]), symbolName);
      const previousContext = document[Q_CONTEXT];
      try {
        document[Q_CONTEXT] = [element, event, url];
        emitEvent("qsymbol", {
          symbol: symbolName,
          element: element,
          reqTime: requestTime,
        });
        await handler(event, element);
      } finally {
        document[Q_CONTEXT] = previousContext;
      }
    }
  }

  function emitEvent(eventName: string, detail?: any) {
    document.dispatchEvent(createEvent(eventName, detail));
  }

  function findSymbol(module: any, symbol: string) {
    if (symbol in module) {
      return module[symbol];
    }

    for (const value of Object.values(module)) {
      if (typeof value === "object" && value && symbol in value) {
        return (value as any)[symbol];
      }
    }
  }

  const camelToKebab = (str: string) =>
    str.replace(/([A-Z])/g, (a) => "-" + a.toLowerCase());

  /**
   * Event handler responsible for processing browser events.
   *
   * If browser emits an event, the `eventProcessor` walks the DOM tree
   * looking for corresponding `(${event.type})`. If found the event's URL
   * is parsed and `import()`ed.
   *
   * @param event - Browser event.
   */
  async function processDocumentEvent(event: Event) {
    // eslint-disable-next-line prefer-const
    let type = camelToKebab(event.type);
    let element = event.target as Element | null;
    broadcast("-document", event, type);

    while (element && element.getAttribute) {
      await dispatch(element, "", event, type);
      element =
        event.bubbles && event.cancelBubble !== true
          ? element.parentElement
          : null;
    }
  }

  function processWindowEvent(event: Event) {
    broadcast("-window", event, camelToKebab(event.type));
  }

  function push(eventNames: string[]) {
    for (const eventName of eventNames) {
      if (events.has(eventName)) continue;
      document.addEventListener(eventName, processDocumentEvent, {
        capture: true,
      });
      window.addEventListener(eventName, processWindowEvent);
      events.add(eventName);
    }
  }

  function processReadyStateChange() {
    if (
      hasInitialized ||
      ["interactive", "complete"].includes(document.readyState)
    )
      return;
    // document is ready
    hasInitialized = 1;
    emitEvent("qinit");
    const requestIdleCallback = window.requestIdleCallback ?? window.setTimeout;
    requestIdleCallback.call(window, () => emitEvent("qidle"));
    
    // Adds a `qvisible` browser event for when elements become visible, for the developer

    if (!events.has("qvisible")) return; // The rest of the code in this function is pointless, as there are no `qvisible` events
      
    const results = document.querySelectorAll("[on\\:qvisible]");
    
    const observer = new IntersectionObserver((entries) => {
      for (const entry of entries) {
        if (!entry.isIntersecting) continue;
        observer.unobserve(entry.target);
        dispatch(entry.target, "", createEvent("qvisible", entry));
      }
    });
    results.forEach(observer.observe);
  }

  if ((document as any).qR) return;

  const qwikevents = (window as any).qwikevents;
  if (Array.isArray(qwikevents)) {
    push(qwikevents);
  }
  (window as any).qwikevents = {
    push: (...events: string[]) => push(events),
  };
  document.addEventListener("readystatechange", processReadyStateChange);
  processReadyStateChange();
}
