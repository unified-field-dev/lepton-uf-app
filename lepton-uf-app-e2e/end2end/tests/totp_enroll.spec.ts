import {
  test,
  expect,
  clickTestId,
  waitForAppReady,
} from "./fixtures";
import {
  enrollTotpToEnabled,
} from "./helpers/totp_settings";
import { totpSecretFromManualLocator } from "./helpers/totp";

async function gotoAccountSettings(
  page: import("@playwright/test").Page,
  auth: {
    signIn: (opts: {
      email: string;
      password: string;
      referer?: string;
    }) => Promise<void>;
  },
  email: string,
  password: string,
) {
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
  await expect(page.getByTestId("totp-settings-section")).toBeVisible();
}

test.describe("pw-totp-enroll", () => {
  test("pw-totp-enroll-happy", async ({ page, auth }) => {
    const email = `totp-enroll-${Date.now()}@example.com`;
    const password = "CorrectHorseBattery1!";
    await gotoAccountSettings(page, auth, email, password);
    await enrollTotpToEnabled(page);
  });

  test("pw-totp-enroll-bad-code-sad", async ({ page, auth }) => {
    const email = `totp-bad-${Date.now()}@example.com`;
    const password = "CorrectHorseBattery1!";
    await gotoAccountSettings(page, auth, email, password);

    await expect(page.getByTestId("totp-settings-idle")).toBeVisible();
    await clickTestId(page, "totp-settings-setup");
    await expect(
      page
        .getByTestId("totp-settings-scan")
        .or(page.getByTestId("totp-settings-error")),
    ).toBeVisible({ timeout: 30_000 });
    await expect(page.getByTestId("totp-settings-scan")).toBeVisible();
    await totpSecretFromManualLocator(
      page.getByTestId("totp-settings-manual-secret"),
    );
    await clickTestId(page, "totp-settings-continue");
    await page.getByTestId("totp-settings-code").fill("000000");
    await clickTestId(page, "totp-settings-confirm-submit");

    await expect(page.getByTestId("totp-settings-error")).toBeVisible({
      timeout: 30_000,
    });
    await expect(page.getByTestId("totp-settings-enabled")).toHaveCount(0);
  });
});
