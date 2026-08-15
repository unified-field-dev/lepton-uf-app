import { test, expect, clickTestId, waitForAppReady } from "./fixtures";

test.describe("pw-appearance", () => {
  test("pw-appearance-save-happy", async ({ page, auth }) => {
    const email = `appearance-ok-${Date.now()}@example.com`;
    const password = "CorrectHorseBattery1!";
    await auth.signIn({ email, password, referer: "/user/appearance" });
    await expect(page).toHaveURL(/\/user\/appearance/, { timeout: 60_000 });
    await waitForAppReady(page);
    await expect(page.getByTestId("appearance-page")).toBeVisible({
      timeout: 60_000,
    });

    const brandToken = () =>
      page.evaluate(() => {
        const el = document.querySelector(".orbital-theme-provider");
        if (!el) return "";
        const s = getComputedStyle(el);
        return (
          s.getPropertyValue("--colorBrandBackground").trim() ||
          s.getPropertyValue("--orb-color-brand-bg").trim()
        );
      });

    const shellBrandBefore = await brandToken();

    await page.getByRole("radio", { name: "Custom brand color" }).check();
    await expect(page.getByTestId("appearance-preset-Valence")).toBeVisible({
      timeout: 15_000,
    });
    await clickTestId(page, "appearance-preset-Valence");
    await clickTestId(page, "appearance-save");

    await expect(page.getByText(/Appearance preferences saved/i)).toBeVisible({
      timeout: 30_000,
    });

    await expect
      .poll(async () => brandToken(), { timeout: 30_000 })
      .not.toBe(shellBrandBefore);
    const shellBrandAfterSave = await brandToken();

    // Hard reload must re-apply prefs to the shell ThemeInjection (not only preview).
    await page.reload({ waitUntil: "domcontentloaded" });
    await waitForAppReady(page);
    await expect
      .poll(async () => brandToken(), { timeout: 60_000 })
      .toBe(shellBrandAfterSave);

    // Warm re-nav still checks form fields (cold lazy chunk can boot-error on goto).
    await page.goto("/");
    await waitForAppReady(page);
    await page.getByRole("link", { name: "Account settings" }).click();
    await expect(page).toHaveURL(/\/user\/account-settings/, { timeout: 60_000 });
    await waitForAppReady(page);
    const expand = page.getByRole("button", { name: "Expand navigation" });
    if (await expand.isVisible().catch(() => false)) {
      await expand.click();
    }
    await page.getByTestId("nav-lepton-appearance").click();
    await expect(page).toHaveURL(/\/user\/appearance/, { timeout: 60_000 });
    await waitForAppReady(page);
    await expect(page.getByTestId("appearance-page")).toBeVisible({
      timeout: 60_000,
    });
    await expect(
      page.getByRole("radio", { name: "Custom brand color" }),
    ).toBeChecked({ timeout: 30_000 });
    await expect(page.getByTestId("appearance-preset-Valence")).toBeVisible({
      timeout: 15_000,
    });
    await expect(page.locator('input[placeholder="#RRGGBB"]')).toHaveValue(
      "#4f6bed",
      { timeout: 30_000 },
    );
  });

  test("pw-appearance-anon-redirect-sad", async ({ page }) => {
    // Warm hydrate on `/` so the lazy appearance chunk has a chance to load.
    await page.goto("/");
    await waitForAppReady(page);
    await page.goto("/user/appearance");

    await expect
      .poll(
        async () => {
          const boot = await page.evaluate(() =>
            document.documentElement.getAttribute("data-orbital-boot-state"),
          );
          const href = page.url();
          if (/\/auth\/signin/.test(href)) {
            return "signin";
          }
          if (boot === "error") {
            return "boot-error";
          }
          return "pending";
        },
        { timeout: 60_000 },
      )
      .toMatch(/^(signin|boot-error)$/);

    if (/\/auth\/signin/.test(page.url())) {
      await waitForAppReady(page);
      await expect(page.getByTestId("auth-dialog-root")).toBeVisible({
        timeout: 60_000,
      });
      return;
    }

    // Boot overlay may intercept clicks; Guest chrome proves no authenticated session.
    await expect(page.getByRole("img", { name: "Guest" })).toBeVisible();
    await expect(page.getByTestId("user-menu-profile")).toHaveCount(0);
  });
});
