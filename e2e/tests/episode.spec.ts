import { test, expect } from "@playwright/test";

test.describe("Episode Page", () => {
  test("shows 404 for non-existent episode", async ({ page }) => {
    await page.goto("/podcasts/test-podcast/999");
    await expect(page.locator("header .title")).not.toContainText("Loading");
    await expect(page.locator("header .title")).toContainText(
      "Episode not found"
    );
    await expect(page.locator("header .subtitle")).toContainText("404");
    await expect(page.getByText("Unable to find episode")).toBeVisible();
  });

  test("has back button to podcast", async ({ page }) => {
    await page.goto("/podcasts/test-podcast/1");
    await expect(page.locator("header .title")).not.toContainText("Loading");
    const backButton = page.locator("header a").first();
    await expect(backButton).toBeVisible();
    await backButton.click();
    await expect(page).toHaveURL("/podcasts/test-podcast");
  });

  test("displays error context in 404", async ({ page }) => {
    await page.goto("/podcasts/my-podcast/42");
    await expect(page.locator("header .title")).not.toContainText("Loading");
    await expect(page.getByText("my-podcast")).toBeVisible();
    await expect(page.getByText("42")).toBeVisible();
  });
});
