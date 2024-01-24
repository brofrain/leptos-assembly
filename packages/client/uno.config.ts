import {
  defineConfig,
  presetTypography,
  presetUno,
  transformerDirectives,
  transformerVariantGroup,
} from "unocss";
import { presetBetterNestedColors } from "unocss-preset-better-nested-colors";

export default defineConfig({
  safelist: [],

  shortcuts: [
    ["cover", "absolute inset-0 max-w-full max-h-full rounded-[inherit]"],
    ["flex-center", "flex items-center justify-center"],
  ],

  theme: { duration: { DEFAULT: "200ms" } },

  presets: [
    presetBetterNestedColors({
      colors: {
        primary: {
          DEFAULT: "#f5f5f5",
          ":dark": "#1a1a1a",
        },

        secondary: {
          DEFAULT: "rgba(40,40,40,.8)",
          ":dark": "rgba(250,250,250,.9)",

          interactive: {
            DEFAULT: "~",
            ":dark": "~",
            ":hover": "accent",
          },
        },

        accent: {
          DEFAULT: "#d74f3f",
          contrast: "#fff",
          focus: {
            DEFAULT: "#a33605",
            ":dark": "#f76220",
          },

          interactive: {
            DEFAULT: "~",
            ":hover": {
              DEFAULT: "accent-focus",
              ":dark": "accent-focus:dark",
            },
            ":disabled": {
              DEFAULT: "#999",
              ":dark": "#333",
            },

            contrast: {
              DEFAULT: "accent-contrast",
              ":dark": "#333",
              ":disabled": {
                DEFAULT: "#f1f1f1",
                ":dark": "#888",
              },
            },
          },
        },
      },
    }),

    presetUno(),
    presetTypography(),
  ],

  transformers: [transformerDirectives(), transformerVariantGroup()],

  content: {
    filesystem: ["{app,components,composables,layouts,pages}/**/*.rs"],
    pipeline: { include: [] },
  },
});
