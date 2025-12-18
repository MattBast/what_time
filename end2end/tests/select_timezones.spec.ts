import { test, expect } from "@playwright/test";

test("clicking timezones dropdown displays list of timezones", async ({
  page,
}) => {
  await page.goto("http://localhost:3000");
  await page.waitForLoadState("networkidle"); // Somtimes Firefox requires a delay

  await page.locator("input#timezone_select").click();

  // Timezones at top of list
  await expect(page.getByText("Abidjan")).toBeVisible();
  await expect(page.getByText("Accra")).toBeVisible();
  await expect(page.getByText("Addis Ababa")).toBeVisible();

  // Timezones in the middle of the list
  await expect(page.getByText("London")).toBeVisible();
  await expect(page.getByText("Paris")).toBeVisible();
});

test("selecting a timezone in the dropdown displays it in the url and on the page", async ({
  page,
}) => {
  await page.goto("http://localhost:3000/?current_time=1766076397");

  // Click on timezones input to reveal timezones and then click on first one.
  await page
    .getByRole("textbox", { name: "Search and add timezones..." })
    .click();
  await page.getByText("🇨🇮 AbidjanCôte d'Ivoire").click();

  // Check that the timezone appeared on the page
  await expect(
    page.getByRole("heading", { name: "🇨🇮 Abidjan (GMT)" }),
  ).toBeVisible();
  await expect(page.locator("#time_picker_Africa__Abidjan")).toHaveValue(
    "16:46",
  );
  await expect(page.locator("#date_picker_Africa__Abidjan")).toHaveValue(
    "2025-12-18",
  );

  // Check that the url now contains the selected timezone
  expect(page.url()).toBe(
    "http://localhost:3000/?current_time=1766076397&zone=Africa__Abidjan",
  );
});

test("selecting two timezones in the dropdown displays them in the url and on the page", async ({
  page,
}) => {
  await page.goto("http://localhost:3000/?current_time=1766076397");

  // Click on timezones input to reveal timezones and then click on first and second one.
  await page
    .getByRole("textbox", { name: "Search and add timezones..." })
    .click();
  await page
    .getByRole("listitem")
    .filter({ hasText: "🇨🇮 AbidjanCôte d'Ivoire" })
    .click();
  await page
    .getByRole("textbox", { name: "Search and add timezones..." })
    .click();
  await page.getByRole("listitem").filter({ hasText: "🇬🇭 AccraGhana" }).click();

  // Check that the timezones appeared on the page
  await expect(
    page.getByRole("heading", { name: "🇨🇮 Abidjan (GMT)" }),
  ).toBeVisible();
  await expect(page.locator("#time_picker_Africa__Abidjan")).toHaveValue(
    "16:46",
  );
  await expect(page.locator("#date_picker_Africa__Abidjan")).toHaveValue(
    "2025-12-18",
  );

  await expect(
    page.getByRole("heading", { name: "🇬🇭 Accra (GMT)" }),
  ).toBeVisible();
  await expect(page.locator("#time_picker_Africa__Accra")).toHaveValue("16:46");
  await expect(page.locator("#date_picker_Africa__Accra")).toHaveValue(
    "2025-12-18",
  );

  // Check that the url now contains the selected timezone
  expect(page.url()).toBe(
    "http://localhost:3000/?current_time=1766076397&zone=Africa__Abidjan%2CAfrica__Accra",
  );
});
