import { test, expect } from "@playwright/test";

test("compare page contains london timezone when it's in the url", async ({
  page,
}) => {
  await page.goto("http://localhost:3000/?zone=Europe__London");
  await page.waitForLoadState("networkidle"); // Somtimes Firefox requires a delay

  await expect(page.getByText("🇬🇧 London (GMT)")).toBeVisible();
});

test("compare page contains london and paris timezones when they're in the url", async ({
  page,
}) => {
  await page.goto("http://localhost:3000/?zone=Europe__London%2CEurope__Paris");
  await page.waitForLoadState("networkidle"); // Somtimes Firefox requires a delay

  await expect(page.getByText("🇬🇧 London (GMT)")).toBeVisible();
  await expect(page.getByText("🇫🇷 Paris (CET)")).toBeVisible();
});
