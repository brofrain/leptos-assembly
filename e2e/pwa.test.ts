import { expect, test } from "@playwright/test";

test.describe("pwa", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
  });

  test("should be available offline", async ({ page, browserName }) => {
    if (browserName === "webkit") {
      test.skip();
    }

    await page.evaluate(
      () =>
        new Promise((resolve) =>
          navigator.serviceWorker.addEventListener("controllerchange", resolve),
        ),
    );

    const logoLocator = page.locator("[test='logo']");

    await logoLocator.waitFor();
    expect(await logoLocator.isVisible()).toBe(true);

    // simulate offline
    await page.route("**/*", (route) => route.abort());

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
