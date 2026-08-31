import {
  test,
  expect,
  dismissAuthOverlay,
  signInAs,
  waitForAppReady,
} from "./fixtures";

async function gotoAccountSettingsSignedIn(
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
  await expect(page.getByTestId("connected-accounts-section")).toBeVisible();
}

async function logout(page: import("@playwright/test").Page) {
  await dismissAuthOverlay(page);
  await page.goto("/auth/logout");
  await waitForAppReady(page);
  const logoutBtn = page.getByTestId("logout-button").getByRole("button");
  await expect(logoutBtn).toBeAttached({ timeout: 60_000 });
  await logoutBtn.click({ force: true });
  // Product shell may keep the auth dialog mounted but hidden; land and open the menu.
  await page.goto("/");
  await waitForAppReady(page);
  await page.getByTestId("user-avatar").click();
  await expect(page.getByTestId("user-menu-signin")).toBeVisible({
    timeout: 60_000,
  });
}

/** Unlink Google if present so the shared mock subject is free for the next serial case. */
async function unlinkGoogleIfPresent(page: import("@playwright/test").Page) {
  await dismissAuthOverlay(page);
  const row = page.getByTestId("connected-accounts-row");
  if ((await row.count()) === 0) {
    return;
  }
  if (!(await row.getByText(/Google/i).count().catch(() => 0))) {
    return;
  }
  const unlinkBtn = row.getByRole("button", { name: "Unlink" });
  try {
    await unlinkBtn.click({ timeout: 5_000 });
  } catch {
    await unlinkBtn.click({ force: true, timeout: 15_000 });
  }
  await expect(
    page.getByTestId("connected-accounts-unlink-confirm"),
  ).toBeVisible();
  await page
    .getByTestId("connected-accounts-unlink")
    .getByRole("button")
    .click({ force: true });
  await expect(page.getByTestId("connected-accounts-empty")).toBeVisible({
    timeout: 30_000,
  });
}

test.describe("pw-oauth link settings", () => {
  // Shared mock IdP subjects — run serially so claims do not race across workers.
  test.describe.configure({ mode: "serial" });

  test("pw-oauth-link-google-happy", async ({ page, auth }) => {
    const email = `oauth-link-${Date.now()}@example.com`;
    const password = "CorrectHorseBattery1!";
    await gotoAccountSettingsSignedIn(page, auth, email, password);
    await unlinkGoogleIfPresent(page);

    await expect(page.getByTestId("connected-accounts-empty")).toBeVisible();
    await page
      .getByTestId("connected-accounts-link-google")
      .getByRole("button")
      .click();

    await expect(page).toHaveURL(/\/user\/account-settings/, {
      timeout: 60_000,
    });
    await expect(page.getByTestId("connected-accounts-section")).toBeVisible({
      timeout: 60_000,
    });
    await expect(page.getByTestId("connected-accounts-row")).toBeVisible({
      timeout: 30_000,
    });
    await expect(page.getByTestId("connected-accounts-row")).toContainText(
      /Google/i,
    );

    await unlinkGoogleIfPresent(page);
  });

  test("pw-oauth-link-account-taken-sad", async ({ page, auth }) => {
    // Claim the shared mock Google subject with a password user.
    const ownerEmail = `oauth-owner-${Date.now()}@example.com`;
    const password = "CorrectHorseBattery1!";
    await gotoAccountSettingsSignedIn(page, auth, ownerEmail, password);
    await unlinkGoogleIfPresent(page);
    await page
      .getByTestId("connected-accounts-link-google")
      .getByRole("button")
      .click();
    await expect(page).toHaveURL(/\/user\/account-settings/, {
      timeout: 60_000,
    });
    await expect(page.getByTestId("connected-accounts-row")).toBeVisible({
      timeout: 30_000,
    });
    await logout(page);

    const otherEmail = `oauth-taken-${Date.now()}@example.com`;
    await gotoAccountSettingsSignedIn(page, auth, otherEmail, password);
    await page
      .getByTestId("connected-accounts-link-google")
      .getByRole("button")
      .click();

    // Product AuthDialog may keep oauth-callback-container mounted but hidden.
    await expect(
      page.getByText(/already linked to another user/i),
    ).toBeVisible({ timeout: 60_000 });

    // Free the shared mock subject so a later serial re-run can start clean.
    await logout(page);
    await signInAs(page, ownerEmail, password, "/user/account-settings");
    await expect(page).toHaveURL(/\/user\/account-settings/, {
      timeout: 60_000,
    });
    await waitForAppReady(page);
    await unlinkGoogleIfPresent(page);
  });
});
