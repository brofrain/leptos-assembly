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
  { url: "/pwa", revision: __BUILD_PIPELINE_ID__ },
]);

registerRoute(
  ({ url }) => url.pathname.startsWith("/hydrate"),
  (options) => {
    const request = new Request(
      options.request.url.replace("/hydrate", "/pwa"),
    );
    return new NetworkFirst().handle({ ...options, request });
  },
);

registerRoute(
  ({ url }) =>
    !url.pathname.startsWith("/api") &&
    !url.pathname.startsWith("/assets") &&
    !url.pathname.startsWith("/pwa"),
  (options) => {
    const request = new Request("/pwa");
    return new NetworkFirst().handle({ ...options, request });
  },
);
