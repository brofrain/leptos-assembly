/// <reference lib="WebWorker" />

import { cacheNames, clientsClaim } from "workbox-core";
import { precacheAndRoute } from "workbox-precaching";
import { registerRoute } from "workbox-routing";
import { CacheFirst } from "workbox-strategies";

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
  async (options) => {
    const request = new Request(
      `${options.url.pathname.replace(
        "/hydrate",
        "/pwa",
      )}?__WB_REVISION__=${__BUILD_PIPELINE_ID__}`,
    );

    const cache = await caches.open(cacheNames.precache);
    const response = await cache.match(request);

    return response || new CacheFirst().handle({ ...options, request });
  },
);

registerRoute(
  ({ url }) =>
    !url.pathname.startsWith("/api") &&
    !url.pathname.startsWith("/assets") &&
    !url.pathname.startsWith("/pwa"),
  async (options) => {
    const request = new Request(
      `/pwa?__WB_REVISION__=${__BUILD_PIPELINE_ID__}`,
    );

    const cache = await caches.open(cacheNames.precache);
    const response = await cache.match(request);

    return response || new CacheFirst().handle({ ...options, request });
  },
);
