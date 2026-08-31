import { expect } from "@playwright/test";
import { authenticator } from "otplib";
import { seedTestData } from "./seed";

function totpCode(secret: string): string {
  return authenticator.generate(secret.replace(/\s+/g, "").toUpperCase());
}

/**
 * Wait for Orbital boot overlay to finish and hydrate to mark the document ready.
 * Product hosts load a large split WASM; SSR nodes stay `hidden` until this clears.
 */
export async function waitForAppReady(
  page: import("@playwright/test").Page,
  timeoutMs = 180_000,
): Promise<void> {
  await expect
    .poll(
      async () =>
        page.evaluate(() => {
          const html = document.documentElement;
          if (html.getAttribute("data-orbital-boot-state") === "error") {
            return "error";
          }
          if (html.getAttribute("data-orbital-hydrated") === "true") {
            return "ready";
          }
          return "loading";
        }),
      { timeout: timeoutMs },
    )
    .toBe("ready");
  const overlay = page.getByTestId("orbital-boot-overlay");
  if ((await overlay.count()) > 0) {
    await expect(overlay).toBeHidden({ timeout: 30_000 });
  }
}

/** Best-effort: clear portaled auth chrome that intercepts funnel clicks. */
export async function dismissAuthOverlay(
  page: import("@playwright/test").Page,
): Promise<void> {
  for (let i = 0; i < 3; i += 1) {
    await page.keyboard.press("Escape").catch(() => undefined);
  }
  const visibleBackdrop = page
    .locator(".orbital-backdrop")
    .filter({ visible: true });
  try {
    await expect(visibleBackdrop).toHaveCount(0, { timeout: 5_000 });
  } catch {
    // Leave page usable; callers may force-click funnel controls.
  }
}

/** Click an Orbital Button wrapped in a native `data-testid` element. */
export async function clickTestId(
  page: import("@playwright/test").Page,
  testId: string,
): Promise<void> {
  const wrap = page.getByTestId(testId);
  await expect(wrap).toBeVisible({ timeout: 30_000 });
  const nested = wrap.getByRole("button");
  const target = (await nested.count()) > 0 ? nested : wrap;
  await expect(target).toBeEnabled({ timeout: 30_000 });
  try {
    await target.click({ timeout: 5_000 });
  } catch {
    await target.click({ force: true, timeout: 15_000 });
  }
}

/** Auth modal body (portaled); prefer over page-shell containers for form fills. */
export function authDialog(page: import("@playwright/test").Page) {
  return page.getByTestId("auth-dialog-root");
}

/** Password-reset modal (portaled). */
export function resetDialog(page: import("@playwright/test").Page) {
  return page.getByRole("dialog");
}

/**
 * After hydrate, re-query the portaled auth dialog, fill credentials, assert
 * values stuck, then submit. Retries once if a remount clears the fields.
 * Surfaces `signin-error` text if navigation stalls.
 */
export async function fillAndSubmitSignIn(
  page: import("@playwright/test").Page,
  email: string,
  password: string,
): Promise<void> {
  await waitForAppReady(page);

  let lastError: unknown;
  for (let attempt = 0; attempt < 2; attempt += 1) {
    const root = authDialog(page);
    await expect(root).toBeVisible({ timeout: 60_000 });

    const emailInput = root
      .getByTestId("signin-email")
      .locator('input[name="email"]');
    const passwordInput = root
      .getByTestId("signin-password")
      .locator('input[name="password"]');
    await emailInput.fill(email);
    await passwordInput.fill(password);
    await expect(emailInput).toHaveValue(email);
    await expect(passwordInput).toHaveValue(password);

    await root.getByTestId("signin-submit").getByRole("button").click();

    try {
      await expect(page).not.toHaveURL(/\/auth\/signin/, { timeout: 45_000 });
      return;
    } catch (err) {
      lastError = err;
      const signinError = root.getByTestId("signin-error");
      if ((await signinError.count()) > 0) {
        const msg = (await signinError.innerText()).trim();
        throw new Error(
          `Sign-in stayed on /auth/signin; signin-error: ${msg || "(empty)"}`,
        );
      }
      const stillEmail = await emailInput.inputValue().catch(() => "");
      if (stillEmail === email) {
        // Values stuck but no navigation — do not spin forever on the same form.
        break;
      }
      // Remount cleared inputs; retry fill/submit once.
    }
  }

  throw lastError instanceof Error
    ? lastError
    : new Error("Sign-in stayed on /auth/signin");
}

/**
 * Assert post-sign-in landing without a follow-up `page.goto`.
 * Re-checks the destination after opening the avatar menu so a stale
 * anonymous session apply cannot sneak past the first visible flash.
 */
export async function assertSignedInLanding(
  page: import("@playwright/test").Page,
  pathPattern: RegExp,
  contentTestId: string,
): Promise<void> {
  await expect(page).toHaveURL(pathPattern, { timeout: 60_000 });
  await expect(page).not.toHaveURL(/\/auth\/signin/);
  await waitForAppReady(page);
  await expect(page.getByTestId(contentTestId)).toBeVisible({ timeout: 60_000 });
  await page.getByTestId("user-avatar").click();
  await expect(page.getByTestId("user-menu-profile")).toBeVisible({
    timeout: 30_000,
  });
  await page.keyboard.press("Escape");
  await dismissAuthOverlay(page);
  await expect(page.getByTestId(contentTestId)).toBeVisible();
  await expect(page).toHaveURL(pathPattern);
  await expect(page).not.toHaveURL(/\/auth\/signin/);
}

/** Land after sign-in; use client nav for profile/appearance (page.goto boot-errors). */
export async function settleSignedIn(
  page: import("@playwright/test").Page,
  landingPath: string,
): Promise<void> {
  await page.waitForLoadState("domcontentloaded").catch(() => undefined);
  await dismissAuthOverlay(page);

  const path =
    landingPath && landingPath !== "/" && landingPath !== "/welcome"
      ? landingPath
      : "/";

  if (path === "/user/profile") {
    await page.goto("/");
    await waitForAppReady(page);
    await dismissAuthOverlay(page);
    await page.getByTestId("user-avatar").click();
    await expect(page.getByTestId("user-menu-profile")).toBeVisible({
      timeout: 15_000,
    });
    await page.getByTestId("user-menu-profile").click();
    await expect(page).toHaveURL(/\/user\/profile/, { timeout: 60_000 });
    await waitForAppReady(page);
    await dismissAuthOverlay(page);
    return;
  }

  if (path === "/user/appearance") {
    await page.goto("/");
    await waitForAppReady(page);
    await dismissAuthOverlay(page);
    await page.getByRole("link", { name: "Account settings" }).click();
    await expect(page).toHaveURL(/\/user\/account-settings/, { timeout: 60_000 });
    await waitForAppReady(page);
    const expand = page.getByRole("button", { name: "Expand navigation" });
    if (await expand.isVisible().catch(() => false)) {
      await expand.click();
    }
    await expect(page.getByTestId("nav-lepton-appearance")).toBeVisible({
      timeout: 15_000,
    });
    await page.getByTestId("nav-lepton-appearance").click();
    await expect(page).toHaveURL(/\/user\/appearance/, { timeout: 60_000 });
    await waitForAppReady(page);
    await dismissAuthOverlay(page);
    return;
  }

  if (path.startsWith("/user/") && !path.startsWith("/user/account-settings")) {
    await page.goto("/");
    await waitForAppReady(page);
    await dismissAuthOverlay(page);
    await page.goto(path);
    await waitForAppReady(page);
    await dismissAuthOverlay(page);
    return;
  }

  const pathname = new URL(page.url()).pathname;
  if (path === "/" && (pathname === "/welcome" || pathname === "/welcome/")) {
    await page.goto("/");
    await waitForAppReady(page);
    await dismissAuthOverlay(page);
    return;
  }

  await waitForAppReady(page);
  await dismissAuthOverlay(page);
}

export async function signInAs(
  page: import("@playwright/test").Page,
  email: string,
  password: string,
  referer = "/",
): Promise<void> {
  const useSoftReferer =
    !referer.startsWith("/user/") ||
    referer.startsWith("/user/account-settings");
  const qs =
    useSoftReferer && referer && referer !== "/"
      ? `?referer=${encodeURIComponent(referer)}`
      : "";
  await page.goto(`/auth/signin${qs}`);
  await fillAndSubmitSignIn(page, email, password);
  await settleSignedIn(page, referer);
}

export async function signupNewUser(
  page: import("@playwright/test").Page,
  email: string,
  password = "CorrectHorseBattery1!",
): Promise<void> {
  await page.goto("/auth/signup");
  await waitForAppReady(page);
  const root = authDialog(page);
  await expect(root).toBeVisible({ timeout: 60_000 });
  await expect(root.getByTestId("signup-page-email")).toBeVisible();
  await root
    .getByTestId("signup-email")
    .locator('input[name="email"]')
    .fill(email);
  await root.getByTestId("signup-email-continue").getByRole("button").click();
  await expect(root.getByTestId("signup-page-details")).toBeVisible({
    timeout: 30_000,
  });
  await root
    .getByTestId("signup-legal-name")
    .locator('input[name="legal_name"]')
    .fill("Alex Rivera");
  await root
    .getByTestId("signup-display-name")
    .locator('input[name="display_name"]')
    .fill("Alex");
  await root
    .getByTestId("signup-password")
    .locator('input[name="password"]')
    .fill(password);
  await root
    .getByTestId("signup-confirm")
    .locator('input[name="confirm"]')
    .fill(password);
  await root.getByTestId("signup-submit").getByRole("button").click();
  await expect(root.getByTestId("signup-page-email-verify")).toBeVisible({
    timeout: 60_000,
  });
  await root.getByTestId("signup-email-skip").getByRole("button").click();
  await expect(root.getByTestId("signup-page-phone")).toBeVisible({
    timeout: 30_000,
  });
  await root.getByTestId("signup-phone-skip").getByRole("button").click();
  await expect(root.getByTestId("signup-page-totp")).toBeVisible({
    timeout: 30_000,
  });
  await root.getByTestId("signup-totp-skip").getByRole("button").click();
  await page.goto("/user/confirm-account");
  await waitForAppReady(page);
  await expect(page.getByTestId("confirm-account-container")).toBeVisible({
    timeout: 60_000,
  });
  await dismissAuthOverlay(page);
}

/** Seed `auth_basic_user` then complete the sign-in form. */
export async function seedAndSignIn(
  page: import("@playwright/test").Page,
  request: import("@playwright/test").APIRequestContext,
  opts: { email: string; password: string; referer?: string },
): Promise<void> {
  const { email, password, referer = "/" } = opts;
  await seedTestData(request, "auth_basic_user", { email, password });
  const useSoftReferer =
    !referer.startsWith("/user/") ||
    referer.startsWith("/user/account-settings");
  const qs =
    useSoftReferer && referer && referer !== "/"
      ? `?referer=${encodeURIComponent(referer)}`
      : "";
  await page.goto(`/auth/signin${qs}`);
  await fillAndSubmitSignIn(page, email, password);
  await settleSignedIn(page, referer);
}

/** Seed `auth_user_with_totp`, sign in, and complete the MFA TOTP step. */
export async function seedAndSignInWithTotp(
  page: import("@playwright/test").Page,
  request: import("@playwright/test").APIRequestContext,
  opts: { email: string; password: string; referer?: string },
): Promise<{ totp_secret: string }> {
  const { email, password, referer = "/" } = opts;
  const seeded = await seedTestData(request, "auth_user_with_totp", {
    email,
    password,
  });
  if (!seeded.totp_secret) {
    throw new Error("auth_user_with_totp seed missing totp_secret");
  }
  const useSoftReferer =
    !referer.startsWith("/user/") ||
    referer.startsWith("/user/account-settings");
  const qs =
    useSoftReferer && referer && referer !== "/"
      ? `?referer=${encodeURIComponent(referer)}`
      : "";
  await page.goto(`/auth/signin${qs}`);
  await waitForAppReady(page);
  const root = authDialog(page);
  await expect(root).toBeVisible({ timeout: 60_000 });
  const emailInput = root
    .getByTestId("signin-email")
    .locator('input[name="email"]');
  const passwordInput = root
    .getByTestId("signin-password")
    .locator('input[name="password"]');
  await emailInput.fill(email);
  await passwordInput.fill(password);
  await expect(emailInput).toHaveValue(email);
  await expect(passwordInput).toHaveValue(password);
  await root.getByTestId("signin-submit").getByRole("button").click();
  await expect(root.getByTestId("signin-mfa-step")).toBeVisible({
    timeout: 30_000,
  });
  await root
    .getByTestId("signin-mfa-totp")
    .locator('input[name="code"]')
    .fill(totpCode(seeded.totp_secret));
  await root.getByTestId("signin-mfa-submit").getByRole("button").click();
  await expect(page).not.toHaveURL(/\/auth\/signin/, { timeout: 60_000 });
  await settleSignedIn(page, referer);
  return { totp_secret: seeded.totp_secret };
}

/** Install a Chromium virtual authenticator for WebAuthn ceremonies. */
export async function installVirtualAuthenticator(
  context: import("@playwright/test").BrowserContext,
  page: import("@playwright/test").Page,
): Promise<string> {
  const client = await context.newCDPSession(page);
  await client.send("WebAuthn.enable");
  const { authenticatorId } = await client.send("WebAuthn.addVirtualAuthenticator", {
    options: {
      protocol: "ctap2",
      transport: "internal",
      hasResidentKey: true,
      hasUserVerification: true,
      isUserVerified: true,
      automaticPresenceSimulation: true,
    },
  });
  return authenticatorId as string;
}
