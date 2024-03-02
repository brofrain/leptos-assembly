/// <reference lib="WebWorker" />

import {
  RouteHandlerCallbackOptions,
  cacheNames,
  clientsClaim,
} from "workbox-core";
import { precacheAndRoute } from "workbox-precaching";
import { registerRoute } from "workbox-routing";
import { CacheFirst, NetworkFirst } from "workbox-strategies";

declare const self: ServiceWorkerGlobalScope;
declare const __BUILD_PIPELINE_ID__: string;

self.skipWaiting();
clientsClaim();

const manifest = self.__WB_MANIFEST;
precacheAndRoute([
  ...manifest,
  { url: "pwa", revision: __BUILD_PIPELINE_ID__ },
]);

async function getCachedResponse(
  options: RouteHandlerCallbackOptions,
  url: string,
) {
  const request = new Request(url);
  const cache = await caches.open(cacheNames.precache);
  const response = await cache.match(request);
  return response ?? new CacheFirst().handle({ ...options, request });
}

function stripFilenameHash(fileUrl: string) {
  return fileUrl.replace(/\.[-\w]+(?=\.[a-zA-Z\d]+$)/, "");
}

const pwaSubstituteUrlPerHashlessFilename = new Map(
  manifest
    .map((entry) => (typeof entry === "string" ? entry : entry.url))
    .filter((url) => url.startsWith("pwa") && url !== "pwa")
    .map((url) => [stripFilenameHash(url).replace(/^pwa\//, ""), url]),
);

function findFilePwaSubstitute(fileUrl: string) {
  const filename = fileUrl
    .match(/(?<=hydrate\/).*\.[-\w]+\.[a-zA-Z\d]+$/)
    ?.at(0);

  if (!filename) {
    return null;
  }

  const hashlessFilename = stripFilenameHash(filename);
  return pwaSubstituteUrlPerHashlessFilename.get(hashlessFilename) ?? null;
}

registerRoute(
  ({ url }) => url.pathname.startsWith("/hydrate"),
  (options) => {
    const pwaSubstituteUrl = findFilePwaSubstitute(options.url.pathname);

    if (!pwaSubstituteUrl) {
      return new NetworkFirst().handle(options);
    }

    return getCachedResponse(options, pwaSubstituteUrl);
  },
);

registerRoute(
  ({ url }) => url.pathname.startsWith("/favicon"),
  (options) =>
    getCachedResponse(
      options,
      `/assets${options.url.pathname}?__WB_REVISION__=${__BUILD_PIPELINE_ID__}`,
    ),
);

registerRoute(
  ({ url }) =>
    !url.pathname.startsWith("/api") &&
    !url.pathname.startsWith("/assets") &&
    !url.pathname.startsWith("/pwa"),
  async (options) =>
    getCachedResponse(options, `/pwa?__WB_REVISION__=${__BUILD_PIPELINE_ID__}`),
);
