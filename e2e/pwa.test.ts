import { expect, test } from "@playwright/test";

test.describe("pwa", () => {
  test("should be available offline", async ({ page, context }) => {
    await page.goto("/");

    const logoLocator = page.locator("[test='logo']");

    await logoLocator.waitFor();
    expect(await logoLocator.isVisible()).toBe(true);

    await page.evaluate(() => navigator.serviceWorker.ready);
    await context.setOffline(true);
    await page.reload();

    await logoLocator.waitFor();
    expect(await logoLocator.isVisible()).toBe(true);

    await page.goto("/hi/abc");

    const welcomeLocator = page.locator("[test='welcome']");
    await welcomeLocator.waitFor();
    expect(await welcomeLocator.isVisible()).toBe(true);
    expect(await welcomeLocator.textContent()).toBe("Hi, abc!");
  });
});
