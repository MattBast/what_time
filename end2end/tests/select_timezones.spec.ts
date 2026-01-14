import { test, expect } from "@playwright/test";

test("select shows on larger screens", async ({ page, isMobile }) => {
  test.skip(isMobile);
  await page.goto("");
  await page.waitForLoadState("networkidle"); // Somtimes Firefox requires a delay

  await expect(page.locator("input#timezone_select")).toBeVisible();
});

test("select does not show on mobiles", async ({ page, isMobile }) => {
  test.skip(!isMobile);
  await page.goto("");
  await page.waitForLoadState("networkidle"); // Somtimes Firefox requires a delay

  await expect(page.locator("input#timezone_select")).toBeHidden();
});

test("clicking timezones dropdown displays list of timezones", async ({
  page,
  isMobile,
}) => {
  test.skip(isMobile);

  await page.goto("");
  await page.waitForLoadState("networkidle"); // Somtimes Firefox requires a delay

  await page.locator("input#timezone_select").click();

  // Timezones at top of list
  await expect(page.getByText("Abidjan").first()).toBeVisible();
  await expect(page.getByText("Accra").first()).toBeVisible();
  await expect(page.getByText("Addis Ababa").first()).toBeVisible();

  // Timezones in the middle of the list
  await expect(page.getByText("London").first()).toBeVisible();
  await expect(page.getByText("Paris").first()).toBeVisible();
});

test("selecting a timezone in the dropdown displays it in the url and on the page", async ({
  page,
  isMobile,
}) => {
  await page.goto("/?current_time=1766076397");

  if (!isMobile) {
    // Click on timezones input to reveal timezones and then click on first one.
    await page
      .getByRole("textbox", { name: "Search and add timezones..." })
      .click();

    await page.getByText("Abidjan").first().click();
  }

  if (isMobile) {
    // Click the floating button to open the drawer
    await page.locator("#floating_button label").click();

    await page.getByText("Abidjan").nth(1).click();
  }

  // Check that the timezone appeared on the page
  await expect(page.getByText("🇨🇮 Abidjan (GMT)")).toBeVisible();
  await expect(page.locator("#time_picker_Africa__Abidjan")).toHaveValue(
    "16:46",
  );
  await expect(page.locator("#date_picker_Africa__Abidjan")).toHaveValue(
    "2025-12-18",
  );

  // Check that the url now contains the selected timezone
  const url = new URL(page.url());
  expect(url.pathname + url.search).toBe(
    "/?current_time=1766076397&zone=Africa__Abidjan",
  );
});

test("selecting two timezones in the dropdown displays them in the url and on the page", async ({
  page,
  isMobile,
}) => {
  await page.goto("/?current_time=1766076397");

  if (!isMobile) {
    // Click on timezones input to reveal timezones and then click on first and second ones.
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
    await page
      .getByRole("listitem")
      .filter({ hasText: "🇬🇭 AccraGhana" })
      .click();
  }

  if (isMobile) {
    // Click the floating button to open the drawer and then click the first and second ones.
    await page.locator("#floating_button label").click();

    await page
      .locator("#drawer_timezones")
      .getByText("🇨🇮 AbidjanCôte d'Ivoire")
      .click();
    await page.locator("#drawer_timezones").getByText("🇬🇭 AccraGhana").click();
  }

  // Check that the timezones appeared on the page
  await expect(page.getByText("🇨🇮 Abidjan (GMT)")).toBeVisible();
  await expect(page.locator("#time_picker_Africa__Abidjan")).toHaveValue(
    "16:46",
  );
  await expect(page.locator("#date_picker_Africa__Abidjan")).toHaveValue(
    "2025-12-18",
  );

  await expect(page.getByText("🇬🇭 Accra (GMT)")).toBeVisible();
  await expect(page.locator("#time_picker_Africa__Accra")).toHaveValue("16:46");
  await expect(page.locator("#date_picker_Africa__Accra")).toHaveValue(
    "2025-12-18",
  );

  // Check that the url now contains the selected timezone
  const url = new URL(page.url());
  expect(url.pathname + url.search).toBe(
    "/?current_time=1766076397&zone=Africa__Abidjan%2CAfrica__Accra",
  );
});
