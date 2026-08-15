import {
  test,
  expect,
  authDialog,
  clearMailpit,
  waitForAppReady,
  waitMailpitMessage,
} from "./fixtures";

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
}

test.describe("pw-account-email-password", () => {
  test("pw-account-change-password-happy", async ({ page, auth }) => {
    const email = `pw-ok-${Date.now()}@example.com`;
    const password = "CorrectHorseBattery1!";
    const nextPassword = "CorrectHorseBattery2!";
    await gotoAccountSettings(page, auth, email, password);

    await expect(page.getByTestId("account-masked-email")).toBeVisible();

    const form = page.locator("form").filter({
      has: page.getByRole("button", { name: "Update password" }),
    });
    await form.locator('input[name="current_password"]').fill(password);
    await form.locator('input[name="new_password"]').fill(nextPassword);
    await form.locator('input[name="confirm_password"]').fill(nextPassword);
    await form.getByRole("button", { name: "Update password" }).click();

    await expect(
      page.getByText(/Password updated successfully/i),
    ).toBeVisible({ timeout: 30_000 });

    // Password change updates the session stamp; continue via sign-in form.
    await page.goto("/auth/signin?referer=%2Fuser%2Faccount-settings");
    await waitForAppReady(page);
    const root = authDialog(page);
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
    await expect(page.getByTestId("signin-error")).toBeVisible({
      timeout: 30_000,
    });

    await root
      .getByTestId("signin-password")
      .locator('input[name="password"]')
      .fill(nextPassword);
    await root.getByTestId("signin-submit").getByRole("button").click();
    await expect(page).toHaveURL(/\/user\/account-settings/, {
      timeout: 60_000,
    });
  });

  test("pw-account-change-password-bad-current-sad", async ({ page, auth }) => {
    const email = `pw-bad-${Date.now()}@example.com`;
    const password = "CorrectHorseBattery1!";
    await gotoAccountSettings(page, auth, email, password);

    const form = page.locator("form").filter({
      has: page.getByRole("button", { name: "Update password" }),
    });
    await form.locator('input[name="current_password"]').fill("WrongPassword!!!!1");
    await form.locator('input[name="new_password"]').fill("CorrectHorseBattery2!");
    await form
      .locator('input[name="confirm_password"]')
      .fill("CorrectHorseBattery2!");
    await form.getByRole("button", { name: "Update password" }).click();

    const changePasswordCard = page
      .locator("*")
      .filter({ hasText: "Change current password" })
      .filter({
        has: page.getByRole("button", { name: "Update password" }),
      })
      .first();
    await expect(
      changePasswordCard.getByText(/Current password is incorrect/i),
    ).toBeVisible({ timeout: 30_000 });
  });

  test("pw-account-change-email-happy", async ({ page, auth, request }) => {
    const email = `email-ok-${Date.now()}@example.com`;
    const password = "CorrectHorseBattery1!";
    const nextEmail = `email-next-${Date.now()}@example.com`;
    await clearMailpit(request);
    await gotoAccountSettings(page, auth, email, password);

    const form = page.locator("form").filter({
      has: page.getByRole("button", { name: "Request email change" }),
    });
    await form.locator('input[name="new_email"]').fill(nextEmail);
    await form.locator('input[name="current_password"]').fill(password);
    await form.getByRole("button", { name: "Request email change" }).click();

    const changeEmailCard = page
      .locator("*")
      .filter({ hasText: "Change email" })
      .filter({
        has: page.getByRole("button", { name: "Request email change" }),
      })
      .first();
    await expect(
      changeEmailCard.getByText(/Verification sent for the new email/i),
    ).toBeVisible({ timeout: 30_000 });
    await waitMailpitMessage(request, nextEmail);
  });

  test("pw-account-change-email-invalid-sad", async ({ page, auth }) => {
    const email = `email-bad-${Date.now()}@example.com`;
    const password = "CorrectHorseBattery1!";
    await gotoAccountSettings(page, auth, email, password);

    const form = page.locator("form").filter({
      has: page.getByRole("button", { name: "Request email change" }),
    });
    await form.locator('input[name="new_email"]').fill("not-an-email");
    await form.locator('input[name="current_password"]').fill(password);
    await form.getByRole("button", { name: "Request email change" }).click();

    await expect(
      page.getByText(/invalid|email|unable/i).first(),
    ).toBeVisible({ timeout: 30_000 });
  });
});
