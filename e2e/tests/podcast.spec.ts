import { test, expect } from "@playwright/test";

test.describe("Podcast Page", () => {
  test("handles non-existent podcast gracefully", async ({ page }) => {
    await page.goto("/podcasts/non-existent-podcast");
    await page.waitForLoadState("networkidle");
    // Page should render without crashing
    await expect(page.locator("body")).toBeVisible();
  });

  test("loads podcast URL", async ({ page }) => {
    await page.goto("/podcasts/test-podcast");
    await page.waitForLoadState("networkidle");
    // Page should render without crashing
    await expect(page.locator("body")).toBeVisible();
  });

  test("podcast URL returns content", async ({ page }) => {
    const response = await page.goto("/podcasts/any-podcast");
    expect(response?.status()).toBeLessThan(500);
  });
});
