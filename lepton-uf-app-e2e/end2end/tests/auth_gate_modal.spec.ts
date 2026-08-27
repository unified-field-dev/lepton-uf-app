import {
  test,
  expect,
  authDialog,
  seedTestData,
  waitForAppReady,
  fillAndSubmitSignIn,
  assertSignedInLanding,
} from "./fixtures";

test.describe("pw-auth-gate-modal", () => {
  test("pw-auth-gate-modal-signin-opens-happy", async ({ page }) => {
    await page.goto("/gate/auth-required");
    await waitForAppReady(page);
    await expect(page.getByTestId("auth-required-empty-state")).toBeAttached();
    await expect(page.getByRole("dialog")).toBeVisible({ timeout: 60_000 });
    await expect(page.getByText("Sign in required")).toBeVisible();

    await page.getByRole("button", { name: "Sign In", exact: true }).click();
    await expect(page).toHaveURL(/\/gate\/auth-required/, { timeout: 15_000 });
    await expect(authDialog(page)).toBeVisible({ timeout: 30_000 });
    await expect(authDialog(page).getByTestId("signin-email")).toBeVisible();
  });

  test("pw-auth-gate-modal-signup-opens-happy", async ({ page }) => {
    await page.goto("/gate/auth-required");
    await waitForAppReady(page);
    await expect(page.getByTestId("auth-required-empty-state")).toBeAttached();
    await expect(page.getByRole("dialog")).toBeVisible({ timeout: 60_000 });

    await page.getByRole("button", { name: "Sign Up", exact: true }).click();
    await expect(page).toHaveURL(/\/gate\/auth-required/, { timeout: 15_000 });
    await expect(authDialog(page)).toBeVisible({ timeout: 30_000 });
    await expect(authDialog(page).getByTestId("signup-email")).toBeVisible();
  });

  test("pw-auth-gate-modal-signin-unlock-happy", async ({ page, request }) => {
    const email = `gate-unlock-${Date.now()}@example.com`;
    const password = "CorrectHorseBattery1!";
    await seedTestData(request, "auth_basic_user", { email, password });

    await page.goto("/gate/auth-required");
    await waitForAppReady(page);
    await expect(page.getByText("Sign in required")).toBeVisible({
      timeout: 60_000,
    });
    await page.getByRole("button", { name: "Sign In", exact: true }).click();

    const root = authDialog(page);
    await expect(root).toBeVisible({ timeout: 30_000 });
    await root
      .getByTestId("signin-email")
      .locator('input[name="email"]')
      .fill(email);
    await root
      .getByTestId("signin-password")
      .locator('input[name="password"]')
      .fill(password);
    await root.getByTestId("signin-submit").getByRole("button").click();

    await assertSignedInLanding(
      page,
      /\/gate\/auth-required/,
      "gate-auth-required-content",
    );
    await expect(page.getByTestId("auth-required-empty-state")).toHaveCount(0);
    await expect(page.getByText("Sign in required")).toHaveCount(0);
  });

  test("pw-auth-gate-modal-bad-creds-sad", async ({ page, request }) => {
    const email = `gate-bad-${Date.now()}@example.com`;
    await seedTestData(request, "auth_basic_user", {
      email,
      password: "CorrectHorseBattery1!",
    });

    await page.goto("/gate/auth-required");
    await waitForAppReady(page);
    await page.getByRole("button", { name: "Sign In", exact: true }).click();

    const root = authDialog(page);
    await expect(root).toBeVisible({ timeout: 30_000 });
    await root
      .getByTestId("signin-email")
      .locator('input[name="email"]')
      .fill(email);
    await root
      .getByTestId("signin-password")
      .locator('input[name="password"]')
      .fill("nope-nope-nope");
    await root.getByTestId("signin-submit").getByRole("button").click();

    await expect(root.getByTestId("signin-error")).toBeVisible({
      timeout: 30_000,
    });
    await expect(page).toHaveURL(/\/gate\/auth-required/);
    await expect(page.getByTestId("gate-auth-required-content")).toHaveCount(0);
  });

  test("pw-auth-gate-take-me-back-sad", async ({ page }) => {
    await page.goto("/");
    await waitForAppReady(page);
    await expect(page.getByTestId("home-root")).toBeVisible({
      timeout: 60_000,
    });

    await page.goto("/gate/auth-required");
    await waitForAppReady(page);
    await expect(page.getByText("Sign in required")).toBeVisible({
      timeout: 60_000,
    });

    await page.getByRole("button", { name: "Take me back", exact: true }).click();
    await expect(page).not.toHaveURL(/\/gate\/auth-required/, {
      timeout: 30_000,
    });
    await expect(page.getByTestId("gate-auth-required-content")).toHaveCount(0);
    await waitForAppReady(page);
    await expect(page.getByTestId("home-root")).toBeVisible({ timeout: 60_000 });
    // Still anonymous: avatar menu offers Sign in (not Profile).
    await page.getByTestId("user-avatar").click();
    await expect(page.getByTestId("user-menu-signin")).toBeVisible({
      timeout: 15_000,
    });
    await expect(page.getByTestId("user-menu-profile")).toHaveCount(0);
  });
});
