import { test, expect } from "@playwright/test";

test.describe("Settings Flow", () => {
  test("navigates to settings from navbar", async ({ page }) => {
    await page.goto("/");
    await page.locator("footer").getByText("Settings").click();
    await expect(page).toHaveURL("/settings");
    await expect(page.locator("header .title")).toContainText("Settings");
  });

  test("shows settings menu with sections", async ({ page }) => {
    await page.goto("/settings");
    // Verify menu labels exist
    await expect(page.locator(".menu-label").first()).toBeVisible();
  });

  test("navigates to player settings", async ({ page }) => {
    await page.goto("/settings");
    await page.getByRole("link", { name: "Player" }).first().click();
    await expect(page).toHaveURL("/settings/player");
    await expect(page.locator("header .title")).toContainText("Player");
  });

  test("player settings has skip time fields", async ({ page }) => {
    await page.goto("/settings/player");
    await expect(page.getByText("Skip forward time")).toBeVisible();
    await expect(page.getByText("Skip back time")).toBeVisible();
  });

  test("player settings accepts valid input", async ({ page }) => {
    await page.goto("/settings/player");
    const skipForward = page.locator("input").first();
    await skipForward.fill("30");
    await expect(skipForward).toHaveValue("30");
  });

  test("player settings input clears correctly", async ({ page }) => {
    await page.goto("/settings/player");
    const skipForward = page.locator("input").first();
    await skipForward.fill("45");
    await expect(skipForward).toHaveValue("45");
    await skipForward.clear();
    await expect(skipForward).toHaveValue("");
  });

  test("has back button to settings", async ({ page }) => {
    await page.goto("/settings/player");
    const backButton = page.locator("header a").first();
    await backButton.click();
    await expect(page).toHaveURL("/settings");
  });
});
