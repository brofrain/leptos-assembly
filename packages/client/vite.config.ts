import Unocss from "unocss/vite";
import { v4 as uuid } from "uuid";
import { defineConfig } from "vite";
import { VitePWA } from "vite-plugin-pwa";
import WebfontDownload from "vite-plugin-webfont-dl";

const unocssWithFonts = (
  fonts: Record<
    string,
    {
      name: string;
      weights?: (string | number)[];
      italic?: boolean;
    }
  >,
) => {
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
};

const buildPipelineId = uuid();

const pwa = () =>
  VitePWA({
    strategies: "injectManifest",
    srcDir: "js",
    filename: "sw.ts",
    base: "/",
    outDir: "../../target/client-prebuild",
    injectManifest: {
      globPatterns: [
        "assets/**/*.{js,css,ico,svg,woff2,webmanifest}",
        "pwa/*.{js,wasm}",
      ],
      manifestTransforms: [
        (entries) => ({
          manifest: entries.map(({ url, size }) => ({
            url,
            size,
            // fonts rarely change and their filenames are already hashed
            revision: url.endsWith(".woff2") ? null : buildPipelineId,
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

const releaseMode = process.env.PROFILE !== "debug";
const pwaEnabled = typeof process.env.CARGO_FEATURE_PWA === "string";

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
      entry: "js/bindings.ts",
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

    pwaEnabled ? pwa() : null,
  ],
});
