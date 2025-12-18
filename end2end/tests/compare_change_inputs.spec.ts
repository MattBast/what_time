import { test, expect } from "@playwright/test";

test("changing timezone time and date changes url", async ({ page }) => {
  await page.goto("http://localhost:3000/?zone=Europe__London");
  await page.waitForLoadState("networkidle"); // Somtimes Firefox requires a delay

  const london_time_picker = page.locator("input#time_picker_Europe__London");
  const london_date_picker = page.locator("input#date_picker_Europe__London");

  await london_time_picker.fill("09:40");
  await london_date_picker.fill("2025-12-18");

  expect(page.url()).toBe(
    "http://localhost:3000/?zone=Europe__London&current_time=1766050800",
  );
});

test("changing one timezone time changes other visible timezones", async ({
  page,
}) => {
  await page.goto("http://localhost:3000/?zone=Europe__London,Europe__Paris");
  await page.waitForLoadState("networkidle"); // Somtimes Firefox requires a delay

  const london_time_picker = page.locator("input#time_picker_Europe__London");
  const london_date_picker = page.locator("input#date_picker_Europe__London");

  await london_time_picker.fill("23:40");
  await london_date_picker.fill("2025-12-18");

  await expect(page.locator("input#time_picker_Europe__Paris")).toHaveValue(
    "00:40",
  );
  await expect(page.locator("input#date_picker_Europe__Paris")).toHaveValue(
    "2025-12-19",
  );
});
