import type { PlaywrightTestConfig } from "@playwright/test";
import { devices } from "@playwright/test";

const webServerCommand = process.env.PLAYWRIGHT_WEBSERVER_COMMAND;

if (!webServerCommand) {
  throw new Error(
    "PLAYWRIGHT_WEBSERVER_COMMAND environment variable must be set",
  );
}

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
    { name: "webkit", use: { ...devices["Desktop Safari"] } },
  ],
  webServer: {
    command: webServerCommand,
    port: 3333,
    timeout: 1000 * 60 * 10,
  },
};

export default config;
