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
};
