import type { PlaywrightTestConfig } from "@playwright/test";
import { devices } from "@playwright/test";

const releaseMode = process.env.PW_WEBSERVER_RELEASE_MODE === "true";

const config: PlaywrightTestConfig = {
  testDir: "e2e",
  timeout: 30 * 1000,
  expect: { timeout: 5000 },
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: [["html", { open: "never" }]],
  use: { actionTimeout: 0, trace: "on-first-retry" },
  projects: [
    { name: "chromium", use: { ...devices["Desktop Chrome"] } },
    { name: "firefox", use: { ...devices["Desktop Firefox"] } },
  ],
  webServer: {
    command: releaseMode ? "just serve-release" : "just serve-pwa",
    port: 3333,
    timeout: 1000 * 60 * 10,
  },
};

export default config;
