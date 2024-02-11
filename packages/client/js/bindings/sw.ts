export const sw = {
  register: async () => {
    const { registerSW } = await import("virtual:pwa-register");

    registerSW({
      onRegistered: (r) =>
        r &&
        setInterval(
          async () => {
            try {
              await r.update();
            } catch {}
          },
          1000 * 60 * 2, // 2 minutes
        ),
    });
  },

  // By default `window.navigator.serviceWorker` is typed as `ServiceWorkerContainer`
  // and can be accessed without `Option<>` in `web_sys` bindings.
  // However, it may actually be `undefined` in some cases - e.g. Firefox's private mode.
  get: () =>
    window.navigator.serviceWorker as ServiceWorkerContainer | undefined,
};
