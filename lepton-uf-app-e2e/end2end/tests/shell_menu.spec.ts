import { test, expect, authDialog, waitForAppReady } from "./fixtures";

test.describe("pw-shell-menu", () => {
  test("pw-shell-anon-signin-opens-dialog-happy", async ({ page }) => {
    await page.goto("/");
    await waitForAppReady(page);
    await expect(page.getByTestId("home-root")).toBeVisible({
      timeout: 60_000,
    });
    await page.getByTestId("user-avatar").click();
    await expect(page.getByTestId("user-menu-signin")).toBeVisible({
      timeout: 15_000,
    });
    await page.getByTestId("user-menu-signin").click();
    await expect(authDialog(page)).toBeVisible({ timeout: 30_000 });
    await expect(authDialog(page).getByTestId("signin-email")).toBeVisible();
  });

  test("pw-shell-signed-in-nav-profile-happy", async ({ page, auth }) => {
    const email = `shell-nav-${Date.now()}@example.com`;
    const password = "CorrectHorseBattery1!";
    await auth.signIn({ email, password, referer: "/" });
    await waitForAppReady(page);
    // Soft post-login may land on `/welcome` (no route on this host → not home-root).
    // Bounce to `/` so the app-bar menu is on the known landing page.
    if (/\/welcome\/?$/.test(new URL(page.url()).pathname)) {
      await page.goto("/");
      await waitForAppReady(page);
    }
    await expect(page).toHaveURL(/\/(welcome)?$/, { timeout: 60_000 });
    if (!/\/welcome\/?$/.test(new URL(page.url()).pathname)) {
      await expect(page.getByTestId("home-root")).toBeVisible({
        timeout: 60_000,
      });
    }

    await page.getByTestId("user-avatar").click();
    await expect(page.getByTestId("user-menu-profile")).toBeVisible({
      timeout: 15_000,
    });
    await page.getByTestId("user-menu-profile").click();
    await expect(page).toHaveURL(/\/user\/profile/, { timeout: 60_000 });
    await waitForAppReady(page);
    await expect(page.getByTestId("profile-container")).toBeVisible({
      timeout: 60_000,
    });
  });

  test("pw-shell-signed-out-profile-gated-sad", async ({ page }) => {
    await page.goto("/");
    await waitForAppReady(page);
    await expect(page.getByTestId("home-root")).toBeVisible({
      timeout: 60_000,
    });
    await page.getByTestId("user-avatar").click();
    await expect(page.getByTestId("user-menu-signin")).toBeVisible();
    await expect(page.getByTestId("user-menu-profile")).toHaveCount(0);

    await page.goto("/user/profile");
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
    } else {
      // Boot error without an authenticated session: profile chrome unusable.
      await expect(page.getByTestId("profile-submit")).toHaveCount(0);
    }
  });
});
