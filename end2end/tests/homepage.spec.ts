import { test, expect } from "@playwright/test";

test("homepage has title", async ({ page }) => {
  await page.goto("/");
  await page.waitForLoadState("networkidle"); // Somtimes Firefox requires a delay

  await expect(page).toHaveTitle("What Time - Compare timezones, fast");
});

test("homepage has headings", async ({ page }) => {
  await page.goto("/");
  await page.waitForLoadState("networkidle"); // Somtimes Firefox requires a delay

  await expect(
    page.getByRole("heading", { name: "Compare timezones, fast" }),
  ).toBeVisible();
  await expect(page.getByText("🙂 Pick your timezone.")).toBeVisible();
  await expect(page.getByText("😀 Compare with another.")).toBeVisible();
  await expect(page.getByText("😁 Keep adding more.")).toBeVisible();
});
