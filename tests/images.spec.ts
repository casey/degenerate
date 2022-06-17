import * as fs from 'fs';
import * as png from 'fast-png';
import axios from 'axios';
import { exec } from './common';
import { test, expect, Page } from '@playwright/test';

const sleep = async (msecs) => {
  return new Promise((resolve) => setTimeout(resolve, msecs));
};

test.beforeAll(async () => {
  let done = false;
  while (!done) {
    const res = await axios.get(`http://localhost:${process.env.PORT}`);
    done = res.status === 200;
    await sleep(100);
  }
});

test.beforeEach(async ({ page }) => {
  await page.setViewportSize({ width: 256, height: 256 });
  await page.goto(`http://localhost:${process.env.PORT}`);
  await page.evaluate('window.preserveDrawingBuffer = true');
  page.on('pageerror', (error) => console.log(error.message));
  page.on('console', (message) => {
    if (process.env.VERBOSE || message.type() == 'error') console.log(message);
  });
});

const imageTest = (name, program) => {
  test(name, async ({ page }) => {
    await page.waitForSelector('html.ready');

    await page.locator('textarea').fill(program);

    await page.keyboard.down('Shift');
    await page.keyboard.press('Enter');

    await page.waitForSelector('html.done');

    const encoded = (
      await page.evaluate(() =>
        document.getElementsByTagName('canvas')[0].toDataURL()
      )
    ).slice('data:image/png;base64,'.length);

    const have = png.decode(Buffer.from(encoded, 'base64')).data;

    const wantPath = `../images/${name}.png`;

    const missing = !fs.existsSync(wantPath);

    if (
      missing ||
      Buffer.compare(
        have,
        png.decode(await fs.promises.readFile(wantPath)).data
      ) != 0
    ) {
      const destination = `../images/${name}.actual-memory.png`;

      await fs.promises.writeFile(destination, encoded, 'base64');

      if (process.platform === 'darwin') {
        await exec(`
          xattr \
          -wx \
          com.apple.FinderInfo \
          0000000000000000000C00000000000000000000000000000000000000000000 \
          ${destination}
        `);
      }

      if (missing) {
        throw `Image test failed: expected image missing ${wantPath}, got ${destination}`;
      } else {
        throw `Image test failed: expected ${wantPath}, got ${destination}`;
      }
    }
  });
};

const tests = {
  all: `
    all();
    render();
  `,
  alpha: `
    alpha(0.5);
    x();
    render();
  `,
  render: `
    render();
  `,
  brilliance: `
    x();
    rotateColor('green', 0.07);
    rotate(0.07);
    for (let i = 0; i < 10; i++) {
      render();
    }
    rotateColor('blue', 0.09);
    rotate(0.09);
    for (let i = 0; i < 10; i++) {
      render();
    }
  `,
  carpet: `
    circle();
    scale(0.5);
    for (let i = 0; i < 8; i++) {
      render();
      wrap();
    }
  `,
  circle: `
    circle();
    render();
  `,
  circle_scale: `
    scale(0.5);
    circle();
    render();
    all();
    scale(0.9);
    wrap();
    render();
  `,
  concentric_circles: `
    scale(0.99);
    circle();
    for (let i = 0; i < 100; i++) {
      render();
    }
  `,
  cross: `
    cross();
    render();
  `,
  default_program: ` `,
  diamonds: `
    rotate(0.3333);
    rotateColor('green', 0.05);
    circle();
    scale(0.5);
    wrap();
    for (let i = 0; i < 8; i++) {
      render();
    }
    rotate(0.8333);
    rotateColor('blue', 0.05);
    for (let i = 0; i < 8; i++) {
      render();
    }
  `,
  grain: `
    rotate(0.111);
    for (let i = 0; i < 16; i++) {
      square();
      render();
      circle();
      render();
    }
  `,
  kaleidoscope: `
    rotateColor('green', 0.05);
    circle();
    scale(0.75);
    wrap();
    for (let i = 0; i < 8; i++) {
      render();
    }
    rotate(0.8333);
    rotateColor('blue', 0.05);
    for (let i = 0; i < 8; i++) {
      render();
    }
  `,
  mod_3: `
    mod(3, 0);
    render();
  `,
  orbs: `
    rotateColor('green', 0.05);
    circle();
    scale(0.75);
    wrap();
    for (let i = 0; i < 8; i++) {
      render();
    }
    rotateColor('blue', 0.05);
    for (let i = 0; i < 8; i++) {
      render();
    }
  `,
  pattern: `
    alpha(0.75);
    circle();
    scale(0.5);
    for (let i = 0; i < 8; i++) {
      render();
      wrap();
    }
  `,
  check: `
    check();
    render();
  `,
  choose_default_seed: `
    rng.choose([
      () => all(),
      () => circle(),
      () => cross(),
      () => square(),
      () => top(),
      () => x()
    ])();
    render();
  `,
  choose_zero_seed: `
    rng.seed(0);
    rng.choose([
      () => all(),
      () => circle(),
      () => cross(),
      () => square(),
      () => top(),
      () => x()
    ])();
    render();
  `,
  choose_nonzero_seed: `
    rng.seed(3);
    rng.choose([
      () => all(),
      () => circle(),
      () => cross(),
      () => square(),
      () => top(),
      () => x()
    ])();
    render();
  `,
  rotate: `
    rotate(0.05);
    x();
    render();
  `,
  rotate_0125_square: `
    rotate(0.125);
    square();
    render();
  `,
  rotate_1_square: `
    rotate(1.0);
    square();
    render();
  `,
  rotate_scale_x: `
    rotate(0.05);
    scale(2);
    x();
    render();
  `,
  rotate_square: `
    rotate(0.05);
    square();
    for (let i = 0; i < 2; i++) {
      render();
    }
  `,
  rotate_square_for_x: `
    rotate(0.05);
    square();
    for (let i = 0; i < 2; i++) {
      render();
    }
    x();
    for (let i = 0; i < 1; i++) {
      render();
    }
  `,
  rows: `
    rows(1, 1);
    render();
  `,
  rows_overflow: `
    rows(4294967295, 4294967295);
    render();
  `,
  rug: `
    rotateColor('green', 0.05);
    circle();
    scale(0.5);
    wrap();
    for (let i = 0; i < 8; i++) {
      render();
    }
    rotateColor('blue', 0.05);
    for (let i = 0; i < 8; i++) {
      render();
    }
  `,
  scale: `
    scale(0.5);
    circle();
    render();
  `,
  scale_circle_for: `
    circle();
    scale(0.5);
    for (let i = 0; i < 8; i++) {
      render();
    }
  `,
  scale_circle_wrap: `
    scale(0.5);
    circle();
    wrap();
    render();
  `,
  scale_rotate: `
    scale(2);
    rotate(0.05);
    x();
    render();
  `,
  scale_x: `
    scale(2);
    x();
    render();
  `,
  smear: `
    const masks = ([
      () => all(),
      () => circle(),
      () => cross(),
      () => square(),
      () => top(),
      () => x()
    ]);
    rng.seed(9);
    rotateColor('green', 0.01);
    rotate(0.01);
    for (let i = 0; i < 100; i++) {
      rng.choose(masks)();
      render();
    }
    rotateColor('blue', 0.01);
    rotate(0.01);
    for (let i = 0; i < 100; i++) {
      rng.choose(masks)();
      render();
    }
  `,
  square: `
    square();
    render();
  `,
  square_top: `
    square();
    render();
    top();
    render();
  `,
  starburst: `
    const masks = ([
      () => all(),
      () => circle(),
      () => cross(),
      () => square(),
      () => top(),
      () => x()
    ]);
    rng.seed(3);
    rotateColor('green', 0.1);
    rotate(0.1);
    for (let i = 0; i < 10; i++) {
      rng.choose(masks)();
      render();
    }
    for (let i = 0; i < 10; i++) {
      rng.choose(masks)();
      render();
    }
    rotateColor('blue', 0.1);
    rotate(0.1);
    for (let i = 0; i < 10; i++) {
      rng.choose(masks)();
      render();
    }
  `,
  top: `
    top();
    render();
  `,
  x: `
    x();
    render();
  `,
  x_loop: `
    x();
    scale(0.5);
    for (let i = 0; i < 8; i++) {
      render();
      wrap();
    }
  `,
  x_scale: `
    x();
    scale(0.5);
    for (let i = 0; i < 8; i++) {
      render();
    }
  `,
  x_wrap: `
    x();
    render();
    scale(0.5);
    wrap();
    identity();
    all();
    render();
  `,
  debug_operation: `
    debug();
    render();
  `,
  mod_zero_is_always_false: `
    mod(0, 1);
    render();
  `,
  square_colors: `
    rotate(0.01);
    rotateColor('green', 0.1);
    square();
    for (let i = 0; i < 10; i++) {
      render();
    }
  `,
  nested_for_loops: `
    circle();
    scale(0.9);
    for (let i = 0; i < 2; i++) {
      for (let j = 0; j < 2; j++) {
        render();
      }
    }
  `,
  for_zero: `
    circle();
    for (let i = 0; i < 0; i++) {
      render();
    }
  `,
  gpu_extra_pixels: `
    rotate(0.01);
    render();
    render();
  `,
  default_color: `
    defaultColor([255, 0, 255]);
    rotate(0.01);
    render();
  `,
  rotate_color_05_red: `
    rotateColor('red', 0.5);
    all();
    render();
  `,
  rotate_color_blue_05_all: `
    rotateColor('blue', 0.5);
    all();
    render();
  `,
  rotate_color_green: `
    rotateColor('green', 0.5);
    all();
    render();
  `,
  rotate_color_blue_1_all: `
    rotateColor('blue', 1.0);
    all();
    render();
  `,
  rotate_color_green_all: `
    rotateColor('green', 1.0);
    all();
    render();
  `,
  rotate_color_red_all: `
    rotateColor('red', 1.0);
    all();
    render();
  `,
  range_loop: `
    scale(0.5);
    circle();
    for (_ of range(10)) {
      render();
    }
  `,
  clear: `
    render();
    clear();
  `,
  reset: `
    x();
    render();
    reset();
    render();
  `,
  reboot: `
    x();
    render();
    reboot();
    render();
  `,
};

for (const name in tests) {
  imageTest(name, tests[name]);
}

test('forbid-unused-images', async () => {
  let testNames = new Set(Object.getOwnPropertyNames(tests));
  let unused = [];

  for (const filename of await fs.promises.readdir('../images')) {
    if (filename === '.DS_Store' || filename.endsWith('.actual-memory.png')) {
      continue;
    }

    let name = filename.replace(/\.png$/, '');

    if (!testNames.has(name)) {
      unused.push(name);
    }
  }

  expect(unused).toEqual([]);
});

test('all-example', async ({ page }) => {
  await page.waitForSelector('html.ready');

  await page.selectOption('select', { label: 'all' });

  await expect(await page.locator('textarea'))
    .toHaveValue(`// Reset state and clear the canvas
reboot();

// Set the all mask
all();

// Render to the canvas
render();

// Press \`Shift + Enter\` to execute
`);
});
