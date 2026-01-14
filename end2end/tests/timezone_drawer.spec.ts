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
