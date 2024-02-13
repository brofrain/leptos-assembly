const { GlobalRegistrator } = require("@happy-dom/global-registrator");

exports.mock_browser = () => {
  GlobalRegistrator.register();
  Window = Object;
};
