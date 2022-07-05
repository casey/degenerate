import type { PlaywrightTestConfig } from '@playwright/test';
import { devices } from '@playwright/test';

const config: PlaywrightTestConfig = {
  expect: { timeout: 5000 },
  forbidOnly: !!process.env.CI,
  fullyParallel: false,
  globalSetup: require.resolve('./global-setup'),
  projects: [
    { name: 'chromium', use: devices['Desktop Chrome'] },
    { name: 'webkit', use: devices['Desktop Webkit'] },
  ],
  reporter: [['html', { open: 'never' }]],
  retries: 3,
  testDir: '.',
  timeout: 5 * 1000,
  use: { actionTimeout: 0, trace: 'on-first-retry' },
};

export default config;
