/// <reference lib="WebWorker" />

import { clientsClaim } from "workbox-core";
import { precacheAndRoute } from "workbox-precaching";
import { registerRoute } from "workbox-routing";
import { NetworkFirst } from "workbox-strategies";

declare const self: ServiceWorkerGlobalScope;
declare const __BUILD_PIPELINE_ID__: string;

self.skipWaiting();
clientsClaim();

const manifest = self.__WB_MANIFEST;
precacheAndRoute([
  ...manifest,
  { url: "/pwa/index.html", revision: __BUILD_PIPELINE_ID__ },
]);

registerRoute(({ url }) => url.pathname.startsWith("/"), new NetworkFirst());

registerRoute("/", (options) => {
  const request = new Request("/pwa/index.html");
  return new NetworkFirst().handle({ ...options, request });
});

registerRoute("/index.html", (options) => {
  const request = new Request("/pwa/index.html");
  return new NetworkFirst().handle({ ...options, request });
});

registerRoute("/pkg/app.js", (options) => {
  const request = new Request("/pwa/app.js");
  return new NetworkFirst().handle({ ...options, request });
});

registerRoute("/pkg/app.wasm", (options) => {
  const request = new Request("/pwa/app.wasm");
  return new NetworkFirst().handle({ ...options, request });
});
