import type { PlaywrightTestConfig } from '@playwright/test';
import { devices } from '@playwright/test';

const config: PlaywrightTestConfig = {
  expect: { timeout: 5000 },
  forbidOnly: !!process.env.CI,
  fullyParallel: true,
  globalSetup: require.resolve('./global-setup'),
  projects: [{ name: 'chromium', use: { ...devices['Desktop Chrome'] } }],
  reporter: [['html', { open: 'never' }]],
  retries: process.env.CI ? 2 : 0,
  testDir: '.',
  timeout: 30 * 1000,
  use: { actionTimeout: 0, trace: 'on-first-retry' },
  workers: 1,
};

export default config;
