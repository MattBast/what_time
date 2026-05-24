import { test, expect } from "@playwright/test";

test.describe("smoke", () => {
  test("desktop shows timezone search", async ({ page, isMobile }) => {
    test.skip(isMobile);
    await page.goto("/");
    await expect(page.locator("input#timezone_select")).toBeVisible();
  });

  test("mobile shows add-timezone control", async ({ page, isMobile }) => {
    test.skip(!isMobile);
    await page.goto("/");
    await expect(page.locator("div#floating_button")).toBeVisible();
  });

  test("mobile drawer opens", async ({ page, isMobile }) => {
    test.skip(!isMobile);
    await page.goto("/");
    await page.locator("#floating_button label").click();
    await expect(page.locator("ul#drawer_timezones")).toBeVisible();
  });

  test("selecting a timezone updates the URL", async ({ page, isMobile }) => {
    test.skip(isMobile);
    await page.goto("/?current_time=1766076397");
    await page
      .getByRole("textbox", { name: "Search and add timezones..." })
      .click();
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
