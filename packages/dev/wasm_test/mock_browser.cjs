const LIVING_KEYS = [
  "DOMException",
  "URL",
  "URLSearchParams",
  "EventTarget",
  "NamedNodeMap",
  "Node",
  "Attr",
  "Element",
  "DocumentFragment",
  "DOMImplementation",
  "Document",
  "XMLDocument",
  "CharacterData",
  "Text",
  "CDATASection",
  "ProcessingInstruction",
  "Comment",
  "DocumentType",
  "NodeList",
  "RadioNodeList",
  "HTMLCollection",
  "HTMLOptionsCollection",
  "DOMStringMap",
  "DOMTokenList",
  "StyleSheetList",
  "HTMLElement",
  "HTMLHeadElement",
  "HTMLTitleElement",
  "HTMLBaseElement",
  "HTMLLinkElement",
  "HTMLMetaElement",
  "HTMLStyleElement",
  "HTMLBodyElement",
  "HTMLHeadingElement",
  "HTMLParagraphElement",
  "HTMLHRElement",
  "HTMLPreElement",
  "HTMLUListElement",
  "HTMLOListElement",
  "HTMLLIElement",
  "HTMLMenuElement",
  "HTMLDListElement",
  "HTMLDivElement",
  "HTMLAnchorElement",
  "HTMLAreaElement",
  "HTMLBRElement",
  "HTMLButtonElement",
  "HTMLCanvasElement",
  "HTMLDataElement",
  "HTMLDataListElement",
  "HTMLDetailsElement",
  "HTMLDialogElement",
  "HTMLDirectoryElement",
  "HTMLFieldSetElement",
  "HTMLFontElement",
  "HTMLFormElement",
  "HTMLHtmlElement",
  "HTMLImageElement",
  "HTMLInputElement",
  "HTMLLabelElement",
  "HTMLLegendElement",
  "HTMLMapElement",
  "HTMLMarqueeElement",
  "HTMLMediaElement",
  "HTMLMeterElement",
  "HTMLModElement",
  "HTMLOptGroupElement",
  "HTMLOptionElement",
  "HTMLOutputElement",
  "HTMLPictureElement",
  "HTMLProgressElement",
  "HTMLQuoteElement",
  "HTMLScriptElement",
  "HTMLSelectElement",
  "HTMLSlotElement",
  "HTMLSourceElement",
  "HTMLSpanElement",
  "HTMLTableCaptionElement",
  "HTMLTableCellElement",
  "HTMLTableColElement",
  "HTMLTableElement",
  "HTMLTimeElement",
  "HTMLTableRowElement",
  "HTMLTableSectionElement",
  "HTMLTemplateElement",
  "HTMLTextAreaElement",
  "HTMLUnknownElement",
  "HTMLFrameElement",
  "HTMLFrameSetElement",
  "HTMLIFrameElement",
  "HTMLEmbedElement",
  "HTMLObjectElement",
  "HTMLParamElement",
  "HTMLVideoElement",
  "HTMLAudioElement",
  "HTMLTrackElement",
  "HTMLFormControlsCollection",
  "SVGElement",
  "SVGGraphicsElement",
  "SVGSVGElement",
  "SVGTitleElement",
  "SVGAnimatedString",
  "SVGNumber",
  "SVGStringList",
  "Event",
  "CloseEvent",
  "CustomEvent",
  "MessageEvent",
  "ErrorEvent",
  "HashChangeEvent",
  "PopStateEvent",
  "StorageEvent",
  "ProgressEvent",
  "PageTransitionEvent",
  "SubmitEvent",
  "UIEvent",
  "FocusEvent",
  "InputEvent",
  "MouseEvent",
  "KeyboardEvent",
  "TouchEvent",
  "CompositionEvent",
  "WheelEvent",
  "BarProp",
  "External",
  "Location",
  "History",
  "Screen",
  "Crypto",
  "Performance",
  "Navigator",
  "PluginArray",
  "MimeTypeArray",
  "Plugin",
  "MimeType",
  "FileReader",
  "Blob",
  "File",
  "FileList",
  "ValidityState",
  "DOMParser",
  "XMLSerializer",
  "FormData",
  "XMLHttpRequestEventTarget",
  "XMLHttpRequestUpload",
  "XMLHttpRequest",
  "WebSocket",
  "NodeFilter",
  "NodeIterator",
  "TreeWalker",
  "AbstractRange",
  "Range",
  "StaticRange",
  "Selection",
  "Storage",
  "CustomElementRegistry",
  "ShadowRoot",
  "MutationObserver",
  "MutationRecord",
  "Headers",
  "AbortController",
  "AbortSignal",
  "Uint8Array",
  "Uint16Array",
  "Uint32Array",
  "Uint8ClampedArray",
  "Int8Array",
  "Int16Array",
  "Int32Array",
  "Float32Array",
  "Float64Array",
  "ArrayBuffer",
  "DOMRectReadOnly",
  "DOMRect",
  "Image",
  "Audio",
  "Option",
  "CSS",
];

const OTHER_KEYS = [
  "addEventListener",
  "alert",
  "blur",
  "cancelAnimationFrame",
  "close",
  "confirm",
  "createPopup",
  "dispatchEvent",
  "document",
  "focus",
  "frames",
  "getComputedStyle",
  "history",
  "innerHeight",
  "innerWidth",
  "length",
  "location",
  "matchMedia",
  "moveBy",
  "moveTo",
  "name",
  "navigator",
  "open",
  "outerHeight",
  "outerWidth",
  "pageXOffset",
  "pageYOffset",
  "parent",
  "postMessage",
  "print",
  "prompt",
  "removeEventListener",
  "requestAnimationFrame",
  "resizeBy",
  "resizeTo",
  "screen",
  "screenLeft",
  "screenTop",
  "screenX",
  "screenY",
  "scroll",
  "scrollBy",
  "scrollLeft",
  "scrollTo",
  "scrollTop",
  "scrollX",
  "scrollY",
  "self",
  "stop",
  "top",
  "Window",
  "window",
];

const KEYS = LIVING_KEYS.concat(OTHER_KEYS);

const skipKeys = ["window", "self", "top", "parent"];

function getWindowKeys(global, win, additionalKeys = []) {
  const keysArray = [...additionalKeys, ...KEYS];
  const keys = new Set(
    keysArray.concat(Object.getOwnPropertyNames(win)).filter((k) => {
      if (skipKeys.includes(k)) return false;
      if (k in global) return keysArray.includes(k);

      return true;
    }),
  );

  return keys;
}

function isClassLikeName(name) {
  return name[0] === name[0].toUpperCase();
}

function populateGlobal(global, win, options = { bindFunctions: true }) {
  const { bindFunctions = false } = options;
  const keys = getWindowKeys(global, win, options.additionalKeys);

  const originals = new Map();

  const overrideObject = new Map();
  for (const key of keys) {
    const boundFunction =
      bindFunctions &&
      typeof win[key] === "function" &&
      !isClassLikeName(key) &&
      win[key].bind(win);

    if (KEYS.includes(key) && key in global) originals.set(key, global[key]);

    Object.defineProperty(global, key, {
      get() {
        if (overrideObject.has(key)) return overrideObject.get(key);
        if (boundFunction) return boundFunction;
        return win[key];
      },
      set(v) {
        overrideObject.set(key, v);
      },
      configurable: true,
    });
  }

  global.window = global;
  global.self = global;
  global.top = global;
  global.parent = global;

  if (global.global) global.global = global;

  if (global.document && global.document.defaultView) {
    Object.defineProperty(global.document, "defaultView", {
      get: () => global,
      enumerable: true,
      configurable: true,
    });
  }

  skipKeys.forEach((k) => keys.add(k));
}

const { JSDOM } = require("jsdom");

exports.mock_browser = () => {
  const { window } = new JSDOM(
    "<!DOCTYPE html> <html> <head></head><body></body> </html>",
    {
      runScripts: "dangerously",
      pretendToBeVisual: true,
      includeNodeLocations: false,
      console: false,
      cookieJar: false,
      contentType: "text/html",
      url: "http://localhost:3000",
    },
  );

  populateGlobal(global, window);

  Window = Object;
};
