/**
 * Product harness fixtures — re-exports [`../shared`](../shared) plus the `auth`
 * fixture that seeds a user and signs in. Specs live beside this file; the shared
 * helpers drive the real `/auth` and `/user` routes, not kit fixtures.
 */
import { test as base, expect } from "@playwright/test";
import { seedAndSignIn, seedAndSignInWithTotp } from "../shared/auth";

export {
  authDialog,
  clearMailpit,
  clearSmsSink,
  clickTestId,
  dismissAuthOverlay,
  fillAndSubmitSignIn,
  assertSignedInLanding,
  installVirtualAuthenticator,
  resetDialog,
  seedTestData,
  seedAndSignInWithTotp,
  settleSignedIn,
  signInAs,
  signupNewUser,
  waitForAppReady,
  waitMailpitCode,
  waitMailpitMessage,
  waitSmsOtp,
  type SeedResult,
} from "../shared";

export const test = base.extend<{
  auth: {
    signIn: (opts: {
      email: string;
      password: string;
      referer?: string;
    }) => Promise<void>;
    signInWithTotp: (opts: {
      email: string;
      password: string;
      referer?: string;
    }) => Promise<{ totp_secret: string }>;
  };
}>({
  auth: async ({ page, request }, use) => {
    await use({
      async signIn(opts) {
        await seedAndSignIn(page, request, opts);
      },
      async signInWithTotp(opts) {
        return seedAndSignInWithTotp(page, request, opts);
      },
    });
  },
});

export { expect };
