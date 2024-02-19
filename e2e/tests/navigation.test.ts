import { expect, test } from "@playwright/test";
import selectors from "~client-selectors";

test.describe("navigation", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
  });

  const HOME_REG = /^https?:\/\/localhost:\d{4}\/?$/;

  test("home", async ({ page }) => {
    await expect(page).toHaveURL(HOME_REG);
  });

  test.describe("hi", () => {
    const HI_NAME = "John";

    test("go via button click", async ({ page }) => {
      await expect(page).toHaveURL(HOME_REG);

      await page.locator("input").fill(HI_NAME);

      const button = page.locator("button");
      await expect(button).toBeEnabled();
      await button.click();

      await page.locator(selectors.app.the_confirms.confirm).click();

      await expect(page).toHaveURL(new RegExp(`/hi/${HI_NAME}$`));
    });

    test("go via enter key", async ({ page }) => {
      await expect(page).toHaveURL(HOME_REG);

      const input = page.locator("input");
      await input.fill(HI_NAME);
      await input.press("Enter");

      await page.locator(selectors.app.the_confirms.confirm).click();

      await expect(page).toHaveURL(new RegExp(`/hi/${HI_NAME}$`));
    });
  });

  test("about", async ({ page }) => {
    await expect(page).toHaveURL(HOME_REG);

    await page.locator('a[title="About"]').click();

    await expect(page).toHaveURL(/\/about$/);
  });
});
