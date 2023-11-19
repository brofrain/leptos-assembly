import { execSync } from "node:child_process";
import Unocss from "unocss/vite";
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

const pwa = () => {
  const revision = execSync("git rev-parse --short HEAD").toString().trim();

  return VitePWA({
    registerType: "autoUpdate",
    includeAssets: ["favicon.ico", "safari-pinned-tab.svg"],
    workbox: {
      globPatterns: ["**/*.{js,css,svg,woff2}"],
      additionalManifestEntries: [
        { url: "index.html", revision },
        { url: "pkg/app.js", revision },
        { url: "pkg/app.wasm", revision },
        { url: "bindings.mjs", revision },
      ],
    },
    manifest: {
      name: "App",
      short_name: "App",
      theme_color: "#ffffff",
      icons: [
        {
          src: "/pwa-192x192.png",
          sizes: "192x192",
          type: "image/png",
        },
        {
          src: "/pwa-512x512.png",
          sizes: "512x512",
          type: "image/png",
        },
        {
          src: "/pwa-512x512.png",
          sizes: "512x512",
          type: "image/png",
          purpose: "any maskable",
        },
      ],
    },
  });
};

export default defineConfig((_) => {
  const releaseMode = process.env.PROFILE !== "debug";
  const pwaEnabled = typeof process.env.CARGO_FEATURE_PWA === "string";
  return {
    build: {
      outDir: "../../target/client-prebuild/assets",
      emptyOutDir: true,
      minify: releaseMode,
      cssMinify: releaseMode && "lightningcss",
      lib: {
        formats: ["es"],
        entry: "bindings.ts",
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
  };
});
