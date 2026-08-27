import { test, expect, clickTestId, waitForAppReady } from "./fixtures";

test.describe("pw-profile", () => {
  test("pw-profile-update-happy", async ({ page, auth }) => {
    const email = `profile-ok-${Date.now()}@example.com`;
    const password = "CorrectHorseBattery1!";
    await auth.signIn({ email, password, referer: "/user/profile" });
    await expect(page).toHaveURL(/\/user\/profile/, { timeout: 60_000 });
    await waitForAppReady(page);
    await expect(page.getByTestId("profile-container")).toBeVisible({
      timeout: 60_000,
    });

    const nameField = page
      .getByTestId("profile-display-name")
      .locator('input[name="display_name"]');
    await expect(nameField).toBeVisible({ timeout: 30_000 });
    await nameField.fill("Ada Lovelace");
    await clickTestId(page, "profile-submit");

    await expect(page.getByTestId("profile-success")).toBeVisible({
      timeout: 30_000,
    });
    await expect(page.getByTestId("profile-success")).toContainText(
      /Profile updated successfully/i,
    );

    // Durable: warm re-nav keeps the saved display name.
    await page.goto("/");
    await waitForAppReady(page);
    await page.getByTestId("user-avatar").click();
    await page.getByTestId("user-menu-profile").click();
    await expect(page).toHaveURL(/\/user\/profile/, { timeout: 60_000 });
    await waitForAppReady(page);
    await expect(
      page
        .getByTestId("profile-display-name")
        .locator('input[name="display_name"]'),
    ).toHaveValue("Ada Lovelace", { timeout: 30_000 });
  });

  test("pw-profile-empty-name-sad", async ({ page, auth }) => {
    const email = `profile-empty-${Date.now()}@example.com`;
    const password = "CorrectHorseBattery1!";
    await auth.signIn({ email, password, referer: "/user/profile" });
    await waitForAppReady(page);
    await expect(page.getByTestId("profile-container")).toBeVisible({
      timeout: 60_000,
    });

    const nameField = page
      .getByTestId("profile-display-name")
      .locator('input[name="display_name"]');
    await expect(nameField).toBeVisible({ timeout: 30_000 });
    await nameField.fill("   ");
    await clickTestId(page, "profile-submit");

    await expect(page.getByTestId("profile-error")).toBeVisible({
      timeout: 30_000,
    });
    await expect(page.getByTestId("profile-error")).toContainText(
      /cannot be empty|Display name/i,
    );
  });

  test("pw-profile-anon-redirect-sad", async ({ page }) => {
    await page.goto("/");
    await waitForAppReady(page);
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
      await expect(page.getByTestId("auth-dialog-root")).toBeVisible({
        timeout: 60_000,
      });
    } else {
      await expect(page.getByTestId("profile-submit")).toHaveCount(0);
    }
  });
});
