import { test, expect } from "@playwright/test";

test.describe("Index Page", () => {
  test("loads successfully", async ({ page }) => {
    await page.goto("/");
    await expect(page).toHaveTitle("Alnwick");
  });

  test("shows page title after loading", async ({ page }) => {
    await page.goto("/");
    await expect(page.locator("header .title")).not.toContainText("Loading");
    await expect(page.locator("header .title")).toContainText("Podcasts");
  });

  test("has navigation bar with links", async ({ page }) => {
    await page.goto("/");
    const nav = page.locator("footer");
    await expect(nav).toBeVisible();
    await expect(nav.getByRole("link")).toHaveCount(3);
  });

  test("navigation bar has correct links", async ({ page }) => {
    await page.goto("/");
    const nav = page.locator("footer");
    await expect(nav.getByText("Podcasts")).toBeVisible();
    await expect(nav.getByText("Add Podcast")).toBeVisible();
    await expect(nav.getByText("Settings")).toBeVisible();
  });

  test("add podcast link in navbar navigates to add page", async ({ page }) => {
    await page.goto("/");
    await page.locator("footer").getByText("Add Podcast").click();
    await expect(page).toHaveURL("/add");
  });
});
