import { test, expect } from "@playwright/test";

test("floating button does not shows on larger screens", async ({
  page,
  isMobile,
}) => {
  test.skip(isMobile);
  await page.goto("");
  await page.waitForLoadState("networkidle"); // Somtimes Firefox requires a delay

  await expect(page.locator("div#floating_button")).toBeHidden();
});

test("floating button shows on mobiles", async ({ page, isMobile }) => {
  test.skip(!isMobile);
  await page.goto("");
  await page.waitForLoadState("networkidle"); // Somtimes Firefox requires a delay

  await expect(page.locator("div#floating_button")).toBeVisible();
});

test("clicking the floating button shows the timezone drawer", async ({
  page,
  isMobile,
}) => {
  test.skip(!isMobile);
  await page.goto("");
  await page.waitForLoadState("networkidle"); // Somtimes Firefox requires a delay

  await page.locator("div#floating_button").click();
  await expect(page.locator("ul#drawer_timezones")).toBeInViewport();
});
