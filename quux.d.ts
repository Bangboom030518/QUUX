interface VirtualElement {
  readonly open: Comment;
  readonly close: Comment;
  readonly insertBefore: <T extends Node>(node: T, child: Node | null) => T;
  readonly appendChild: <T extends Node>(node: T) => T;
  readonly insertBeforeTo: (newParent: QwikElement, child: Node | null) => void;
  readonly appendTo: (newParent: QwikElement) => void;
  readonly ownerDocument: Document;
  readonly namespaceURI: string;
  readonly nodeType: 111;
  readonly childNodes: Node[];
  readonly firstChild: Node | null;
  readonly previousSibling: Node | null;
  readonly nextSibling: Node | null;
  readonly remove: () => void;
  readonly closest: (query: string) => Element | null;
  readonly hasAttribute: (prop: string) => boolean;
  readonly getAttribute: (prop: string) => string | null;
  readonly removeAttribute: (prop: string) => void;
  readonly querySelector: (query: string) => QwikElement | null;
  readonly querySelectorAll: (query: string) => QwikElement[];
  readonly compareDocumentPosition: (other: Node) => number;
  readonly matches: (query: string) => boolean;
  readonly setAttribute: (prop: string, value: string) => void;
  readonly removeChild: (node: Node) => void;
  readonly localName: string;
  readonly nodeName: string;
  readonly isConnected: boolean;
  readonly parentElement: Element | null;
}

type QwikElement = Element | VirtualElement;

interface QContext {
  $element$: QwikElement;
  $refMap$: any[];
  $flags$: number;
  $id$: string;
  $props$: Record<string, any> | null;
  $componentQrl$: QRLInternal<OnRenderFn<any>> | null;
  li: Listener[];
  $seq$: any[] | null;
  $watches$: SubscriberEffect[] | null;
  $contexts$: Map<string, any> | null;
  $appendStyles$: StyleAppend[] | null;
  $scopeIds$: string[] | null;
  $vdom$: ProcessedJSXNode | null;
  $slots$: ProcessedJSXNode[] | null;
  $dynamicSlots$: QContext[] | null;
  $parent$: QContext | null;
  $slotParent$: QContext | null;
}
