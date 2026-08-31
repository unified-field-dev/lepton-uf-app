import {
  test,
  expect,
  seedTestData,
  fillAndSubmitSignIn,
  waitForAppReady,
  assertSignedInLanding,
} from "./fixtures";

test.describe("pw-auth-referer", () => {
  test("pw-auth-referer-safe-happy", async ({ page, request }) => {
    const email = `referer-ok-${Date.now()}@example.com`;
    const password = "CorrectHorseBattery1!";
    await seedTestData(request, "auth_basic_user", { email, password });

    await page.goto("/auth/signin?referer=%2Fuser%2Fprofile");
    await fillAndSubmitSignIn(page, email, password);

    await expect(page).not.toHaveURL(/evil\.example/);
    await assertSignedInLanding(page, /\/user\/profile/, "profile-container");
  });

  test("pw-auth-referer-gate-path-happy", async ({ page, request }) => {
    const email = `referer-gate-${Date.now()}@example.com`;
    const password = "CorrectHorseBattery1!";
    await seedTestData(request, "auth_basic_user", { email, password });

    await page.goto(
      `/auth/signin?referer=${encodeURIComponent("/gate/auth-required/")}`,
    );
    await fillAndSubmitSignIn(page, email, password);

    await expect(page).not.toHaveURL(/\/auth\/signin/);
    await assertSignedInLanding(
      page,
      /\/gate\/auth-required/,
      "gate-auth-required-content",
    );
    await expect(page.getByText("Sign in required")).toHaveCount(0);
  });

  test("pw-auth-referer-evil-sad", async ({ page, request }) => {
    const email = `referer-evil-${Date.now()}@example.com`;
    const password = "CorrectHorseBattery1!";
    await seedTestData(request, "auth_basic_user", { email, password });

    await page.goto(
      `/auth/signin?referer=${encodeURIComponent("//evil.example")}`,
    );
    await fillAndSubmitSignIn(page, email, password);

    // Sanitized evil referer → `/`; post_login_path may map `/` → `/welcome`.
    await expect(page).not.toHaveURL(/evil\.example/);
    await expect(page).toHaveURL(/\/(welcome)?$/, { timeout: 60_000 });
    await waitForAppReady(page).catch(() => undefined);
    const homeRoot = page.getByTestId("home-root");
    if ((await homeRoot.count()) > 0) {
      await expect(homeRoot).toBeVisible({ timeout: 15_000 });
    }
    await page.getByTestId("user-avatar").click();
    await expect(page.getByTestId("user-menu-profile")).toBeVisible({
      timeout: 30_000,
    });
  });
});
