import type { QContext } from "./core/state/context";

/**
 * Set up event listening for browser.
 *
 * Determine all of the browser events and set up global listeners for them.
 * If browser triggers event search for the lazy load URL and `import()` it.
 *
 */

export const qwikLoader = (hasInitialized?: number) => {
  const Q_CONTEXT = "__q_context__";
  const events = new Set();

  const broadcast = (infix: string, event: Event, type = event.type) => {
    document
      .querySelectorAll(`[on${infix}\\:${type}]`)
      .forEach((target) => dispatch(target, infix, event, type));
  };

  const resolveContainer = (containerEl: Element) => {
    if ((containerEl as any)["_qwikjson_"] === undefined) {
      const parentJSON =
        containerEl === document.documentElement ? document.body : containerEl;
      let script = parentJSON.lastElementChild;
      while (script) {
        if (
          script.tagName === "SCRIPT" &&
          script.getAttribute("type") === "qwik/json"
        ) {
          (containerEl as any)["_qwikjson_"] = JSON.parse(
            script.textContent!.replace(/\\x3C(\/?script)/g, "<$1")
          );
          break;
        }
        script = script.previousElementSibling;
      }
    }
  };

  const createEvent = (eventName: string, detail?: any) =>
    new CustomEvent(eventName, {
      detail,
    });

  const dispatch = async (
    element: Element,
    onPrefix: string,
    event: Event,
    eventName = event.type
  ) => {
    const attrName = "on" + onPrefix + ":" + eventName;
    if (element.hasAttribute("preventdefault:" + eventName)) {
      event.preventDefault();
    }
    const context = (element as any)["_qc_"] as QContext | undefined;
    const qrls = context?.li.filter((li) => li[0] === attrName);
    if (qrls && qrls.length > 0) {
      for (const q of qrls) {
        await q[1].getFn([element, event], () => element.isConnected)(
          event,
          element
        );
      }
      return;
    }
    const attrValue = element.getAttribute(attrName);
    if (attrValue) {
      const container = element.closest("[q\\:container]")!;
      const base = new URL(
        container.getAttribute("q:base")!,
        document.baseURI
      );

      for (const qrl of attrValue.split("\n")) {
        const url = new URL(qrl, base);
        const symbolName =
          url.hash.replace(/^#?([^?[|]*).*$/, "$1") || "default";
        const reqTime = performance.now();
        const module = import(url.href.split("#")[0]);
        resolveContainer(container);
        const handler = findSymbol(await module, symbolName);
        const previousContext = document[Q_CONTEXT];
        if (element.isConnected) {
          try {
            document[Q_CONTEXT] = [element, event, url];
            emitEvent("qsymbol", {
              symbol: symbolName,
              element: element,
              reqTime,
            });
            await handler(event, element);
          } finally {
            document[Q_CONTEXT] = previousContext;
          }
        }
      }
    }
  };

  const emitEvent = (eventName: string, detail?: any) => {
    document.dispatchEvent(createEvent(eventName, detail));
  };

  const findSymbol = (module: any, symbol: string) => {
    if (symbol in module) {
      return module[symbol];
    }

    for (const value of Object.values(module)) {
      if (typeof value === "object" && value && symbol in value) {
        return (value as any)[symbol];
      }
    }
  };

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
  const processDocumentEvent = async (event: Event) => {
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
  };

  const processWindowEvent = (ev: Event) => {
    broadcast("-window", ev, camelToKebab(ev.type));
  };

  const processReadyStateChange = () => {
    const readyState = document.readyState;
    if (
      !hasInitialized &&
      (readyState == "interactive" || readyState == "complete")
    ) {
      // document is ready
      hasInitialized = 1;

      emitEvent("qinit");
      const riC = window.requestIdleCallback ?? window.setTimeout;
      riC.bind(window)(() => emitEvent("qidle"));

      if (events.has("qvisible")) {
        const results = document.querySelectorAll("[on\\:qvisible]");
        const observer = new IntersectionObserver((entries) => {
          for (const entry of entries) {
            if (entry.isIntersecting) {
              observer.unobserve(entry.target);
              dispatch(entry.target, "", createEvent("qvisible", entry));
            }
          }
        });
        results.forEach((el) => observer.observe(el));
      }
    }
  };

  const push = (eventNames: string[]) => {
    for (const eventName of eventNames) {
      if (!events.has(eventName)) {
        document.addEventListener(eventName, processDocumentEvent, { capture: true })
        window.addEventListener(eventName, processWindowEvent);
        events.add(eventName);
      }
    }
  };

  if (!(document as any).qR) {
    const qwikevents = (window as any).qwikevents;
    if (Array.isArray(qwikevents)) {
      push(qwikevents);
    }
    (window as any).qwikevents = {
      push: (...e: string[]) => push(e),
    };
    document.addEventListener("readystatechange", processReadyStateChange);
    processReadyStateChange();
  }
};

export interface QwikLoaderMessage extends MessageEvent {
  data: string[];
}
