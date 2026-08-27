import { test, expect, waitForAppReady } from "./fixtures";

test.describe("pw-account-settings-composition", () => {
  test("pw-account-settings-composition-happy", async ({ page, auth }) => {
    const email = `compose-${Date.now()}@example.com`;
    const password = "CorrectHorseBattery1!";
    await auth.signIn({
      email,
      password,
      referer: "/user/account-settings",
    });
    await expect(page).toHaveURL(/\/user\/account-settings/, {
      timeout: 60_000,
    });
    await waitForAppReady(page);
    await expect(page.getByTestId("account-settings-container")).toBeVisible({
      timeout: 60_000,
    });
    await expect(page.getByTestId("user-app-layout-root")).toBeVisible();

    await expect(page.getByTestId("account-masked-email")).toBeVisible();
    await expect(page.getByTestId("totp-settings-section")).toBeVisible();
    await expect(page.getByTestId("connected-accounts-section")).toBeVisible();
    await expect(page.getByTestId("devices-section")).toBeVisible();
    await expect(page.getByTestId("account-wipe-section")).toBeVisible();

    // Left-nav testids live in the collapsed sidebar; expand when present.
    const expandNav = page.getByRole("button", { name: "Expand navigation" });
    if (await expandNav.isVisible().catch(() => false)) {
      await expandNav.click();
    }
    await expect(page.getByTestId("nav-user-profile")).toBeVisible({
      timeout: 15_000,
    });
    await expect(page.getByTestId("nav-lepton-appearance")).toBeVisible();
    await expect(page.getByTestId("nav-user-account-settings")).toBeVisible();
  });

  test("pw-user-index-redirects-to-account-settings", async ({
    page,
    auth,
  }) => {
    const email = `user-index-${Date.now()}@example.com`;
    const password = "CorrectHorseBattery1!";
    await auth.signIn({
      email,
      password,
      referer: "/",
    });
    await waitForAppReady(page);
    // Client nav: `page.goto("/user")` SSR-boots without the session resource.
    await page.getByRole("link", { name: "User home" }).click();
    await expect(page).toHaveURL(/\/user\/account-settings\/?/, {
      timeout: 60_000,
    });
    await expect(page.getByTestId("account-settings-container")).toBeVisible({
      timeout: 60_000,
    });
    await expect(page.getByText("Coming Soon", { exact: true })).toHaveCount(0);
  });
});
