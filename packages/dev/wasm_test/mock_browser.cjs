const { GlobalRegistrator } = require("@happy-dom/global-registrator");

exports.mock_browser = () => {
  if (global.document) {
    global.document.body.innerHTML = "";
    GlobalRegistrator.unregister();
  }
  GlobalRegistrator.register();

  // biome-ignore lint/suspicious/noGlobalAssign: Since the browser global context is mocked,
  // we need to make web_sys's internal `window instanceof Window` check pass.
  // Overriding `Window` is much easier than applying it as the fake `window`'s constructor.
  Window = Object;
};
