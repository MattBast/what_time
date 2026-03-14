import { test, expect } from "@playwright/test";

test("changing theme changes the pages colour", async ({ page }) => {
  await page.goto("");
  await page.waitForLoadState("networkidle"); // Somtimes Firefox requires a delay

  // Open dropdown and select valentine theme.
  await page.getByRole("button", { name: "Theme" }).click();
  await page.getByRole("radio", { name: "Valentine" }).check();

  // Page header changed to pink
  await expect(
    page.getByRole("heading", { name: "Compare timezones, fast" }),
  ).toHaveCSS("color", "oklch(0.52 0.223 3.958)");

  // Check that reloading the page does not reset the theme.
  await page.reload();
  await expect(
    page.getByRole("heading", { name: "Compare timezones, fast" }),
  ).toHaveCSS("color", "oklch(0.52 0.223 3.958)");
});
