import { test, expect } from "@playwright/test";

test("clicking theme toggle dropdown displays list of themes", async ({
  page,
}) => {
  await page.goto("");
  await page.waitForLoadState("networkidle"); // Somtimes Firefox requires a delay

  await page.getByRole("button", { name: "Theme" }).click();

  await expect(page.getByRole("radio", { name: "Default" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Light" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Cupcake" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Bumblebee" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Emerald" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Corporate" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Synthwave" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Retro" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Cyberpunk" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Valentine" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Halloween" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Garden" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Forest" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Aqua" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Lofi" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Pastel" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Fantasy" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Black" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Luxury" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Dracula" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Cmyk" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Autumn" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Business" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Acid" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Lemonade" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Night" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Coffee" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Winter" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Dim" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Nord" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Sunset" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Caramellatte" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Abyss" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Silk" })).toBeVisible();
});

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
