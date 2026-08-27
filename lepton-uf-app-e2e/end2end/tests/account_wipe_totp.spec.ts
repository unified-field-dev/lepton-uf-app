import { test, expect, clickTestId, waitForAppReady } from "./fixtures";
import { totpCode } from "./helpers/totp";

async function gotoAccountSettingsWithTotp(
  page: import("@playwright/test").Page,
  auth: {
    signInWithTotp: (opts: {
      email: string;
      password: string;
      referer?: string;
    }) => Promise<{ totp_secret: string }>;
  },
  email: string,
  password: string,
): Promise<string> {
  const { totp_secret } = await auth.signInWithTotp({
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
  await expect(page.getByTestId("account-wipe-form")).toBeVisible();
  return totp_secret;
}

test.describe("pw-account-wipe-totp", () => {
  test("pw-account-wipe-totp-bad-code-sad", async ({ page, auth }) => {
    const email = `wipe-totp-bad-${Date.now()}@example.com`;
    const password = "CorrectHorseBattery1!";
    await gotoAccountSettingsWithTotp(page, auth, email, password);

    await page
      .getByTestId("account-wipe-form")
      .locator('input[name="confirm_phrase"]')
      .fill("DELETE");
    await page
      .getByTestId("account-wipe-form")
      .locator('input[name="current_password"]')
      .fill(password);
    await page
      .getByTestId("account-wipe-form")
      .locator('input[name="totp_code"]')
      .fill("000000");
    await clickTestId(page, "account-wipe-submit");

    await expect(page.getByTestId("account-wipe-error")).toBeVisible({
      timeout: 30_000,
    });
    await expect(page.getByTestId("account-settings-container")).toBeVisible();
  });

  test("pw-account-wipe-totp-happy", async ({ page, auth }) => {
    const email = `wipe-totp-ok-${Date.now()}@example.com`;
    const password = "CorrectHorseBattery1!";
    const secret = await gotoAccountSettingsWithTotp(page, auth, email, password);

    await page
      .getByTestId("account-wipe-form")
      .locator('input[name="confirm_phrase"]')
      .fill("DELETE");
    await page
      .getByTestId("account-wipe-form")
      .locator('input[name="current_password"]')
      .fill(password);
    await page
      .getByTestId("account-wipe-form")
      .locator('input[name="totp_code"]')
      .fill(totpCode(secret));
    await clickTestId(page, "account-wipe-submit");

    await expect(page).not.toHaveURL(/\/user\/account-settings/, {
      timeout: 60_000,
    });

    await page.goto("/auth/signin");
    await waitForAppReady(page);
    const root = page.getByTestId("auth-dialog-root");
    await expect(root).toBeVisible({ timeout: 60_000 });
    await root
      .getByTestId("signin-email")
      .locator('input[name="email"]')
      .fill(email);
    await root
      .getByTestId("signin-password")
      .locator('input[name="password"]')
      .fill(password);
    await root.getByTestId("signin-submit").getByRole("button").click();
    await expect(root.getByTestId("signin-error")).toBeVisible({
      timeout: 30_000,
    });
  });
});
