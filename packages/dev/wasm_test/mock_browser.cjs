const { GlobalRegistrator } = require("@happy-dom/global-registrator");

exports.mock_browser = () => {
  if (global.document) {
    global.document.body.innerHTML = "";
    GlobalRegistrator.unregister();
  }
  GlobalRegistrator.register();
  Window = Object;
};
