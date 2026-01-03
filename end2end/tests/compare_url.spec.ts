import { test, expect } from "@playwright/test";

test("compare page contains london timezone when it's in the url", async ({
  page,
}) => {
  await page.goto("/?zone=Europe__London");
  await page.waitForLoadState("networkidle"); // Somtimes Firefox requires a delay

  await expect(page.getByText("🇬🇧 London (GMT)")).toBeVisible();
});

test("compare page contains london and paris timezones when they're in the url", async ({
  page,
}) => {
  await page.goto("/?zone=Europe__London%2CEurope__Paris");
  await page.waitForLoadState("networkidle"); // Somtimes Firefox requires a delay

  await expect(page.getByText("🇬🇧 London (GMT)")).toBeVisible();
  await expect(page.getByText("🇫🇷 Paris (CET)")).toBeVisible();
});

test("current time in url query is displayed on the page as a human readable time and date", async ({
  page,
}) => {
  await page.goto("/?zone=Europe__London&current_time=1765987708");
  await page.waitForLoadState("networkidle"); // Somtimes Firefox requires a delay

  await expect(page.locator("input#time_picker_Europe__London")).toHaveValue(
    "16:08",
  );
  await expect(page.locator("input#date_picker_Europe__London")).toHaveValue(
    "2025-12-17",
  );
});

test("human readable times and dates show for all timezones in url", async ({
  page,
}) => {
  await page.goto(
    "/?zone=Europe__London%2CEurope__Paris&current_time=1765987708",
  );
  await page.waitForLoadState("networkidle"); // Somtimes Firefox requires a delay

  // London
  await expect(page.locator("input#time_picker_Europe__London")).toHaveValue(
    "16:08",
  );
  await expect(page.locator("input#date_picker_Europe__London")).toHaveValue(
    "2025-12-17",
  );

  // Paris
  await expect(page.locator("input#time_picker_Europe__Paris")).toHaveValue(
    "17:08",
  );
  await expect(page.locator("input#date_picker_Europe__Paris")).toHaveValue(
    "2025-12-17",
  );
});
