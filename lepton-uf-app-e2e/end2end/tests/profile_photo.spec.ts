import { test, expect, clickTestId, waitForAppReady } from "./fixtures";
import * as fs from "node:fs";
import * as os from "node:os";
import * as path from "node:path";

/** Minimal 1×1 PNG. */
const TINY_PNG = Buffer.from(
  "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==",
  "base64",
);

function writeTempUpload(name: string, bytes: Buffer): string {
  const filePath = path.join(os.tmpdir(), `lepton-photo-${Date.now()}-${name}`);
  fs.writeFileSync(filePath, bytes);
  return filePath;
}

test.describe("pw-profile-photo", () => {
  test("pw-profile-photo-upload-happy", async ({ page, auth }) => {
    const email = `photo-ok-${Date.now()}@example.com`;
    const password = "CorrectHorseBattery1!";
    await auth.signIn({ email, password, referer: "/user/profile" });
    await expect(page).toHaveURL(/\/user\/profile/, { timeout: 60_000 });
    await waitForAppReady(page);
    await expect(page.getByTestId("profile-photo-upload")).toBeVisible({
      timeout: 60_000,
    });

    const pngPath = writeTempUpload("ok.png", TINY_PNG);
    await page
      .getByTestId("profile-photo-upload")
      .locator('input[type="file"]')
      .setInputFiles(pngPath);

    // Server upload + profile refetch remounts the upload widget (toast is racy).
    await expect
      .poll(
        async () => {
          const img = page
            .getByTestId("profile-photo-display")
            .locator("img")
            .first();
          if ((await img.count()) === 0) {
            return "";
          }
          return (await img.getAttribute("src")) ?? "";
        },
        { timeout: 60_000 },
      )
      .toMatch(/\/api\/files\//);

    const src = await page
      .getByTestId("profile-photo-display")
      .locator("img")
      .first()
      .getAttribute("src");
    const res = await page.request.get(src!);
    expect(res.ok()).toBeTruthy();
    expect(res.headers()["content-type"] ?? "").toMatch(/image\//);
  });

  test("pw-profile-photo-reject-sad", async ({ page, auth }) => {
    const email = `photo-bad-${Date.now()}@example.com`;
    const password = "CorrectHorseBattery1!";
    await auth.signIn({ email, password, referer: "/user/profile" });
    await waitForAppReady(page);
    await expect(page.getByTestId("profile-photo-upload")).toBeVisible({
      timeout: 60_000,
    });

    const exePath = writeTempUpload("bad.exe", Buffer.from("MZ-not-an-image"));
    await page
      .getByTestId("profile-photo-upload")
      .locator('input[type="file"]')
      .setInputFiles(exePath);

    await expect(
      page.getByTestId("profile-photo-upload").getByText(/not allowed|File type/i),
    ).toBeVisible({ timeout: 15_000 });

    await expect(
      page.getByTestId("profile-photo-display").locator("img"),
    ).toHaveCount(0);
  });
});
