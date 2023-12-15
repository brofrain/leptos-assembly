/// <reference lib="WebWorker" />

import { precacheAndRoute, type PrecacheEntry } from "workbox-precaching";

declare let self: ServiceWorkerGlobalScope & {
  __WB_MANIFEST: PrecacheEntry[];
};

const manifest = self.__WB_MANIFEST;
precacheAndRoute(manifest);
