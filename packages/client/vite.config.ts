/// <reference types="vite/client" />
/// <reference types="vite-plugin-pwa/client" />

import Unocss from "unocss/vite";
import { v4 as uuid } from "uuid";
import { defineConfig } from "vite";
import { VitePWA } from "vite-plugin-pwa";
import WebfontDownload from "vite-plugin-webfont-dl";

function unocssWithFonts(
  fonts: Record<
    string,
    {
      name: string;
      weights?: (string | number)[];
      italic?: boolean;
    }
  >,
) {
  const urls: string[] = [];
  const fontFamily: Record<string, string> = {};

  for (const [key, font] of Object.entries(fonts)) {
    fontFamily[key] = font.name;

    let url = `https://fonts.googleapis.com/css2?family=${font.name.replaceAll(
      " ",
      "+",
    )}`;

    if (font.weights?.length) {
      url += `:${font.italic ? "ital," : ""}wght@`;

      if (font.italic) {
        url += [
          font.weights.map((v) => `0,${v}`).join(";"),
          font.weights.map((v) => `1,${v}`).join(";"),
        ].join(";");
      } else {
        url += font.weights.join(";");
      }
    }

    url += "&display=swap";

    urls.push(url);
  }

  return [Unocss({ theme: { fontFamily } }), WebfontDownload(urls)];
}

const buildPipelineId = uuid();
const pwaEnabled = typeof process.env.CARGO_FEATURE_PWA === "string";

const ALREADY_HASHED_FILENAME_REGEXES = [
  // cargo-leptos output
  /^pwa\/.*\.js$/,
  /^pwa\/.*\.wasm$/,
  // fonts downloaded by vite-plugin-webfont-dl
  /\.woff2$/,
  // SW stuff from vite-plugin-pwa
  /^assets\/workbox-window\.prod\..*\.js$/,
  /^assets\/virtual_pwa-register-.*\.js$/,
];

function isAlreadyHashed(url: string) {
  return ALREADY_HASHED_FILENAME_REGEXES.some((r) => r.test(url));
}

function pwa() {
  return VitePWA({
    disable: !pwaEnabled,
    strategies: "injectManifest",
    srcDir: "js",
    filename: "sw.ts",
    base: "/",
    outDir: "../../target/client-prebuild",
    injectManifest: {
      globPatterns: [
        "assets/**/*.{js,css,ico,png,svg,woff2,webmanifest}",
        "pwa/*.{js,wasm}",
      ],
      manifestTransforms: [
        (entries) => ({
          manifest: entries.map(({ url, size }) => ({
            url,
            size,
            revision: isAlreadyHashed(url) ? null : buildPipelineId,
          })),
        }),
      ],
      maximumFileSizeToCacheInBytes: 1024 * 1024 * 10, // 10MB
    },
    manifest: {
      name: "App",
      short_name: "App",
      description: "App",
      theme_color: "#ffffff",
      icons: [
        {
          src: "/assets/pwa-192x192.png",
          sizes: "192x192",
          type: "image/png",
        },
        {
          src: "/assets/pwa-512x512.png",
          sizes: "512x512",
          type: "image/png",
        },
        {
          src: "/assets/pwa-512x512.png",
          sizes: "512x512",
          type: "image/png",
          purpose: "any maskable",
        },
      ],
    },
  });
}

const releaseMode = process.env.PROFILE !== "debug";

export default defineConfig({
  define: {
    __BUILD_PIPELINE_ID__: JSON.stringify(buildPipelineId),
  },
  base: "/assets",
  build: {
    outDir: "../../target/client-prebuild/assets",
    emptyOutDir: true,
    minify: releaseMode,
    cssMinify: releaseMode && "lightningcss",
    lib: {
      formats: ["es"],
      entry: "js/bindings/index.ts",

      // The file is being imported by wasm-bindgen using absolute path `/assets/bindings.js`.
      // Therefore its filename should not be hashed.
      fileName: "bindings",
    },
  },
  plugins: [
    ...unocssWithFonts({
      sans: {
        name: "B612",
        weights: [400, 700],
        italic: true,
      },
      mono: {
        name: "B612 Mono",
        weights: [400, 700],
        italic: true,
      },
    }),
    pwa(),
  ],
});
