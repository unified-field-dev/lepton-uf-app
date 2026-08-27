/**
 * Shared Playwright helpers for Lepton auth UI e2e.
 *
 * Product hosts can copy or import from this folder (path relative to the
 * harness). Seed scenarios are owned by `lepton-test-support` in Rust.
 */

export { seedTestData, type SeedResult } from "./seed";
export { clearMailpit, waitMailpitCode, waitMailpitMessage } from "./mail";
export { clearSmsSink, waitSmsOtp } from "./sms";
export {
  authDialog,
  clickTestId,
  dismissAuthOverlay,
  fillAndSubmitSignIn,
  assertSignedInLanding,
  installVirtualAuthenticator,
  resetDialog,
  seedAndSignIn,
  seedAndSignInWithTotp,
  settleSignedIn,
  signInAs,
  signupNewUser,
  waitForAppReady,
} from "./auth";
