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
    computer.all();
    computer.render();
  `,
  alpha: `
    computer.alpha(0.5);
    computer.x();
    computer.render();
  `,
  render: `
    computer.render();
  `,
  brilliance: `
    computer.x();
    computer.rotateColor('green', 0.07);
    computer.rotate(0.07);
    for (let i = 0; i < 10; i++) {
      computer.render();
    }
    computer.rotateColor('blue', 0.09);
    computer.rotate(0.09);
    for (let i = 0; i < 10; i++) {
      computer.render();
    }
  `,
  carpet: `
    computer.circle();
    computer.scale(0.5);
    for (let i = 0; i < 8; i++) {
      computer.render();
      computer.wrap();
    }
  `,
  circle: `
    computer.circle();
    computer.render();
  `,
  circle_scale: `
    computer.scale(0.5);
    computer.circle();
    computer.render();
    computer.all();
    computer.scale(0.9);
    computer.wrap();
    computer.render();
  `,
  circle_scale_chain: `
    computer
      .scale(0.5)
      .circle()
      .render()
      .all()
      .scale(0.9)
      .wrap()
      .render();
  `,
  concentric_circles: `
    computer.scale(0.99);
    computer.circle();
    for (let i = 0; i < 100; i++) {
      computer.render();
    }
  `,
  cross: `
    computer.cross();
    computer.render();
  `,
  default_program: ` `,
  diamonds: `
    computer.rotate(0.3333);
    computer.rotateColor('green', 0.05);
    computer.circle();
    computer.scale(0.5);
    computer.wrap();
    for (let i = 0; i < 8; i++) {
      computer.render();
    }
    computer.rotate(0.8333);
    computer.rotateColor('blue', 0.05);
    for (let i = 0; i < 8; i++) {
      computer.render();
    }
  `,
  grain: `
    computer.rotate(0.111);
    for (let i = 0; i < 16; i++) {
      computer.square();
      computer.render();
      computer.circle();
      computer.render();
    }
  `,
  kaleidoscope: `
    computer.rotateColor('green', 0.05);
    computer.circle();
    computer.scale(0.75);
    computer.wrap();
    for (let i = 0; i < 8; i++) {
      computer.render();
    }
    computer.rotate(0.8333);
    computer.rotateColor('blue', 0.05);
    for (let i = 0; i < 8; i++) {
      computer.render();
    }
  `,
  mod_3: `
    computer.mod(3, 0);
    computer.render();
  `,
  orbs: `
    computer.rotateColor('green', 0.05);
    computer.circle();
    computer.scale(0.75);
    computer.wrap();
    for (let i = 0; i < 8; i++) {
      computer.render();
    }
    computer.rotateColor('blue', 0.05);
    for (let i = 0; i < 8; i++) {
      computer.render();
    }
  `,
  pattern: `
    computer.alpha(0.75);
    computer.circle();
    computer.scale(0.5);
    for (let i = 0; i < 8; i++) {
      computer.render();
      computer.wrap();
    }
  `,
  check: `
    computer.check();
    computer.render();
  `,
  choose_default_seed: `
    rng.choose([
      () => computer.all(),
      () => computer.circle(),
      () => computer.cross(),
      () => computer.square(),
      () => computer.top(),
      () => computer.x()
    ])();
    computer.render();
  `,
  choose_zero_seed: `
    rng.seed(0);
    rng.choose([
      () => computer.all(),
      () => computer.circle(),
      () => computer.cross(),
      () => computer.square(),
      () => computer.top(),
      () => computer.x()
    ])();
    computer.render();
  `,
  choose_nonzero_seed: `
    rng.seed(3);
    rng.choose([
      () => computer.all(),
      () => computer.circle(),
      () => computer.cross(),
      () => computer.square(),
      () => computer.top(),
      () => computer.x()
    ])();
    computer.render();
  `,
  rotate: `
    computer.rotate(0.05);
    computer.x();
    computer.render();
  `,
  rotate_0125_square: `
    computer.rotate(0.125);
    computer.square();
    computer.render();
  `,
  rotate_1_square: `
    computer.rotate(1.0);
    computer.square();
    computer.render();
  `,
  rotate_scale_x: `
    computer.rotate(0.05);
    computer.scale(2);
    computer.x();
    computer.render();
  `,
  rotate_square: `
    computer.rotate(0.05);
    computer.square();
    for (let i = 0; i < 2; i++) {
      computer.render();
    }
  `,
  rotate_square_for_x: `
    computer.rotate(0.05);
    computer.square();
    for (let i = 0; i < 2; i++) {
      computer.render();
    }
    computer.x();
    for (let i = 0; i < 1; i++) {
      computer.render();
    }
  `,
  rows: `
    computer.rows(1, 1);
    computer.render();
  `,
  rows_overflow: `
    computer.rows(4294967295, 4294967295);
    computer.render();
  `,
  rug: `
    computer.rotateColor('green', 0.05);
    computer.circle();
    computer.scale(0.5);
    computer.wrap();
    for (let i = 0; i < 8; i++) {
      computer.render();
    }
    computer.rotateColor('blue', 0.05);
    for (let i = 0; i < 8; i++) {
      computer.render();
    }
  `,
  scale: `
    computer.scale(0.5);
    computer.circle();
    computer.render();
  `,
  scale_circle_for: `
    computer.circle();
    computer.scale(0.5);
    for (let i = 0; i < 8; i++) {
      computer.render();
    }
  `,
  scale_circle_wrap: `
    computer.scale(0.5);
    computer.circle();
    computer.wrap();
    computer.render();
  `,
  scale_rotate: `
    computer.scale(2);
    computer.rotate(0.05);
    computer.x();
    computer.render();
  `,
  scale_x: `
    computer.scale(2);
    computer.x();
    computer.render();
  `,
  smear: `
    const masks = ([
      () => computer.all(),
      () => computer.circle(),
      () => computer.cross(),
      () => computer.square(),
      () => computer.top(),
      () => computer.x()
    ]);
    rng.seed(9);
    computer.rotateColor('green', 0.01);
    computer.rotate(0.01);
    for (let i = 0; i < 100; i++) {
      rng.choose(masks)();
      computer.render();
    }
    computer.rotateColor('blue', 0.01);
    computer.rotate(0.01);
    for (let i = 0; i < 100; i++) {
      rng.choose(masks)();
      computer.render();
    }
  `,
  square: `
    computer.square();
    computer.render();
  `,
  square_top: `
    computer.square();
    computer.render();
    computer.top();
    computer.render();
  `,
  starburst: `
    const masks = ([
      () => computer.all(),
      () => computer.circle(),
      () => computer.cross(),
      () => computer.square(),
      () => computer.top(),
      () => computer.x()
    ]);
    rng.seed(3);
    computer.rotateColor('green', 0.1);
    computer.rotate(0.1);
    for (let i = 0; i < 10; i++) {
      rng.choose(masks)();
      computer.render();
    }
    for (let i = 0; i < 10; i++) {
      rng.choose(masks)();
      computer.render();
    }
    computer.rotateColor('blue', 0.1);
    computer.rotate(0.1);
    for (let i = 0; i < 10; i++) {
      rng.choose(masks)();
      computer.render();
    }
  `,
  top: `
    computer.top();
    computer.render();
  `,
  x: `
    computer.x();
    computer.render();
  `,
  x_loop: `
    computer.x();
    computer.scale(0.5);
    for (let i = 0; i < 8; i++) {
      computer.render();
      computer.wrap();
    }
  `,
  x_scale: `
    computer.x();
    computer.scale(0.5);
    for (let i = 0; i < 8; i++) {
      computer.render();
    }
  `,
  x_wrap: `
    computer.x();
    computer.render();
    computer.scale(0.5);
    computer.wrap();
    computer.identity();
    computer.all();
    computer.render();
  `,
  debug_operation: `
    computer.debug();
    computer.render();
  `,
  mod_zero_is_always_false: `
    computer.mod(0, 1);
    computer.render();
  `,
  square_colors: `
    computer.rotate(0.01);
    computer.rotateColor('green', 0.1);
    computer.square();
    for (let i = 0; i < 10; i++) {
      computer.render();
    }
  `,
  nested_for_loops: `
    computer.circle();
    computer.scale(0.9);
    for (let i = 0; i < 2; i++) {
      for (let j = 0; j < 2; j++) {
        computer.render();
      }
    }
  `,
  for_zero: `
    computer.circle();
    for (let i = 0; i < 0; i++) {
      computer.render();
    }
  `,
  gpu_extra_pixels: `
    computer.rotate(0.01);
    computer.render();
    computer.render();
  `,
  default_color: `
    computer.defaultColor([255, 0, 255]);
    computer.rotate(0.01);
    computer.render();
  `,
  rotate_color_05_red: `
    computer.rotateColor('red', 0.5);
    computer.all();
    computer.render();
  `,
  rotate_color_blue_05_all: `
    computer.rotateColor('blue', 0.5);
    computer.all();
    computer.render();
  `,
  rotate_color_green: `
    computer.rotateColor('green', 0.5);
    computer.all();
    computer.render();
  `,
  rotate_color_blue_1_all: `
    computer.rotateColor('blue', 1.0);
    computer.all();
    computer.render();
  `,
  rotate_color_green_all: `
    computer.rotateColor('green', 1.0);
    computer.all();
    computer.render();
  `,
  rotate_color_red_all: `
    computer.rotateColor('red', 1.0);
    computer.all();
    computer.render();
  `,
  range_loop: `
    computer.scale(0.5);
    computer.circle();
    for (_ of range(10)) {
      computer.render();
    }
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

  await expect(await page.locator('textarea')).toHaveValue(`// Set the all mask
computer.all();

// Render to the canvas
computer.render();

// Press \`Shift + Enter\` to execute
`);
});
