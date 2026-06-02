import { test, expect } from "@playwright/test";

test.describe("smoke", () => {
  test("add timezone button is visible", async ({ page }) => {
    await page.goto("/");
    await expect(page.getByRole("button", { name: "Add Timezone" })).toBeVisible();
  });

  test("drawer opens from add timezone button", async ({ page }) => {
    await page.goto("/");
    await page.getByRole("button", { name: "Add Timezone" }).click();
    await expect(page.locator("ul#drawer_timezones")).toBeVisible();
    await expect(
      page.getByRole("textbox", { name: "Search and add timezones..." }),
    ).toBeVisible();
  });

  test("selecting a timezone updates the URL", async ({ page }) => {
    await page.goto("/?current_time=1766076397");
    await page.getByRole("button", { name: "Add Timezone" }).click();
    await page
      .getByRole("textbox", { name: "Search and add timezones..." })
      .fill("Abidjan");
    await page.getByText("Abidjan").first().click();
    await expect(page).toHaveURL(
      /current_time=1766076397&zone=Africa__Abidjan/,
    );
  });

  test("theme selection sets data-theme on html", async ({ page }) => {
    await page.goto("/");
    await page.getByRole("button", { name: "Theme" }).click();
    await page.getByRole("radio", { name: "Valentine" }).check();
    await expect(page.locator("html")).toHaveAttribute("data-theme", "valentine");
  });
});
