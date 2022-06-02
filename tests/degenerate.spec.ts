import * as fs from 'fs';
import { exec } from 'child_process';
import { test, expect, Page } from '@playwright/test';
import UPNG from 'upng-js';

const VERBOSE = false;

const clean = async () => {
  const files = await fs.promises.opendir('../images');
  for await (const file of files) {
    const path = `../images/${file.name}`;
    if (path.endsWith('.actual-memory.png') && fs.existsSync(path)) {
      await fs.promises.unlink(path);
    }
  }
};

const cmd = (command) => {
  exec(command, (err, _stdout, _stderr) => {
    if (err) console.log(`error: ${err.message}`);
  });
};

test.describe.configure({ mode: 'serial' });

test.beforeAll(async () => {
  await clean();
  cmd(`
    cd ..
    cargo build --target wasm32-unknown-unknown
    wasm-bindgen --target web --no-typescript target/wasm32-unknown-unknown/debug/degenerate.wasm --out-dir www
    cargo run --package serve
  `);
});

test.beforeEach(async ({ page }) => {
  await page.waitForTimeout(1000);
  await page.setViewportSize({ width: 256, height: 256 });
  await page.goto('http://localhost:8000');
  await page.evaluate('window.test = true');
  if (VERBOSE) {
    page.on('console', (message) => console.log(message));
    page.on('pageerror', (error) => console.log(error.message));
  }
});

test.afterAll(async () => {
  cmd('npx kill-port 8000');
});

const imageTest = (name, program) => {
  test(name, async ({ page }) => {
    await page.locator('textarea').fill(program);

    await page.waitForSelector('canvas.done');

    const dataURL = await page.evaluate(() =>
      document.getElementsByTagName('canvas')[0].toDataURL()
    );

    const encoded = dataURL.slice('data:image/png;base64,'.length);

    const have = UPNG.decode(Buffer.from(encoded, 'base64')).data;

    const wantPath = `../images/${name}.png`;

    const want = UPNG.decode(await fs.promises.readFile(wantPath)).data;

    if (Buffer.compare(have, want) != 0) {
      const destination = `../images/${name}.actual-memory.png`;
      await fs.promises.writeFile(destination, encoded, 'base64');
      throw `Image test failed: expected ${wantPath}, got ${destination}`;
    }
  });
};

imageTest(
  'all',
  `
    computer.all();
    computer.apply();
  `
);

imageTest(
  'alpha',
  `
    computer.alpha(0.5);
    computer.x();
    computer.apply();
  `
);

imageTest(
  'apply',
  `
    computer.apply();
  `
);

imageTest(
  'brilliance',
  `
    computer.x();
    computer.rotateColor('g', 0.07);
    computer.rotate(0.07);
    for (let i = 0; i < 10; i++) {
      computer.apply();
    }
    computer.rotateColor('b', 0.09);
    computer.rotate(0.09);
    for (let i = 0; i < 10; i++) {
      computer.apply();
    }
  `
);

imageTest(
  'carpet',
  `
    computer.circle();
    computer.scale(0.5);
    for (let i = 0; i < 8; i++) {
      computer.apply();
      computer.wrap();
    }
  `
);

imageTest(
  'circle',
  `
    computer.circle();
    computer.apply();
  `
);

imageTest(
  'circle_scale',
  `
    computer.scale(0.5);
    computer.circle();
    computer.apply();
    computer.all();
    computer.scale(0.9);
    computer.wrap();
    computer.apply();
  `
);

imageTest(
  'concentric_circles',
  `
    computer.scale(0.99);
    computer.circle();
    for (let i = 0; i < 100; i++) {
      computer.apply();
    }
  `
);

imageTest(
  'cross',
  `
    computer.cross();
    computer.apply();
  `
);

imageTest('default_program', ``);

imageTest(
  'diamonds',
  `
    computer.rotate(0.3333);
    computer.rotateColor('g', 0.05);
    computer.circle();
    computer.scale(0.5);
    computer.wrap();
    for (let i = 0; i < 8; i++) {
      computer.apply();
    }
    computer.rotate(0.8333);
    computer.rotateColor('b', 0.05);
    for (let i = 0; i < 8; i++) {
      computer.apply();
    }
  `
);

imageTest(
  'grain',
  `
    computer.rotate(0.111);
    for (let i = 0; i < 16; i++) {
      computer.square();
      computer.apply();
      computer.circle();
      computer.apply();
    }
  `
);

imageTest(
  'kaleidoscope',
  `
    computer.rotateColor('g', 0.05);
    computer.circle();
    computer.scale(0.75);
    computer.wrap();
    for (let i = 0; i < 8; i++) {
      computer.apply();
    }
    computer.rotate(0.8333);
    computer.rotateColor('b', 0.05);
    for (let i = 0; i < 8; i++) {
      computer.apply();
    }
  `
);

imageTest(
  'mod_3',
  `
    computer.mod(3, 0);
    computer.apply();
  `
);

imageTest(
  'orbs',
  `
    computer.rotateColor('g', 0.05);
    computer.circle();
    computer.scale(0.75);
    computer.wrap();
    for (let i = 0; i < 8; i++) {
      computer.apply();
    }
    computer.rotateColor('b', 0.05);
    for (let i = 0; i < 8; i++) {
      computer.apply();
    }
  `
);

imageTest(
  'pattern',
  `
    computer.alpha(0.75);
    computer.circle();
    computer.scale(0.5);
    for (let i = 0; i < 8; i++) {
      computer.apply();
      computer.wrap();
    }
  `
);

// TODO
// imageTest(
//   'choose_default_seed',
//   `
//     choose all circle cross square top x
//     apply
//   `,
// );

// TODO
// imageTest(
//   'choose_zero_seed',
//   `
//     choose all circle cross square top x
//     apply
//   `,
// );

// TODO
// imageTest(
//   'choose_nonzero_seed',
//   `
//     seed 2
//     choose all circle cross square top x
//     apply
//   `,
// );

imageTest(
  'rotate',
  `
    computer.rotate(0.05);
    computer.x();
    computer.apply();
  `
);

imageTest(
  'rotate_0125_square',
  `
    computer.rotate(0.125);
    computer.square();
    computer.apply();
  `
);

imageTest(
  'rotate_1_square',
  `
    computer.rotate(1.0);
    computer.square();
    computer.apply();
  `
);

imageTest(
  'rotate_color_05_red',
  `
    computer.rotateColor('red', 0.5);
    computer.all();
    computer.apply();
  `
);

imageTest(
  'rotate_color_blue_05_all',
  `
    computer.rotateColor('blue', 0.5);
    computer.all();
    computer.apply();
  `
);

imageTest(
  'rotate_color_blue_1_all',
  `
    computer.rotateColor('blue', 1.0);
    computer.all();
    computer.apply();
  `
);

imageTest(
  'rotate_color_blue_all',
  `
    computer.rotateColor('b', 0.5);
    computer.all();
    computer.apply();
  `
);

imageTest(
  'rotate_color_g',
  `
    computer.rotateColor('g', 0.5);
    computer.all();
    computer.apply();
  `
);

imageTest(
  'rotate_color_green',
  `
    computer.rotateColor('green', 0.5);
    computer.all();
    computer.apply();
  `
);

imageTest(
  'rotate_color_green_all',
  `
    computer.rotateColor('green', 1.0);
    computer.all();
    computer.apply();
  `
);

imageTest(
  'rotate_color_r',
  `
    computer.rotateColor('r', 0.5);
    computer.all();
    computer.apply();
  `
);

imageTest(
  'rotate_color_red_all',
  `
    computer.rotateColor('red', 1.0);
    computer.all();
    computer.apply();
  `
);

imageTest(
  'rotate_scale_x',
  `
    computer.rotate(0.05);
    computer.scale(2);
    computer.x();
    computer.apply();
  `
);

imageTest(
  'rotate_square',
  `
    computer.rotate(0.05);
    computer.square();
    for (let i = 0; i < 2; i++) {
      computer.apply();
    }
  `
);

imageTest(
  'rotate_square_for_x',
  `
    computer.rotate(0.05);
    computer.square();
    for (let i = 0; i < 2; i++) {
      computer.apply();
    }
    computer.x();
    for (let i = 0; i < 1; i++) {
      computer.apply();
    }
  `
);

imageTest(
  'rows',
  `
    computer.rows(1, 1);
    computer.apply();
  `
);

imageTest(
  'rows_overflow',
  `
    computer.rows(4294967295, 4294967295);
    computer.apply();
  `
);

imageTest(
  'rug',
  `
    computer.rotateColor('g', 0.05);
    computer.circle();
    computer.scale(0.5);
    computer.wrap();
    for (let i = 0; i < 8; i++) {
      computer.apply();
    }
    computer.rotateColor('b', 0.05);
    for (let i = 0; i < 8; i++) {
      computer.apply();
    }
  `
);

imageTest(
  'scale',
  `
    computer.scale(0.5);
    computer.circle();
    computer.apply();
  `
);

imageTest(
  'scale_circle_for',
  `
    computer.circle();
    computer.scale(0.5);
    for (let i = 0; i < 8; i++) {
      computer.apply();
    }
  `
);

imageTest(
  'scale_circle_wrap',
  `
    computer.scale(0.5);
    computer.circle();
    computer.wrap();
    computer.apply();
  `
);

imageTest(
  'scale_rotate',
  `
    computer.scale(2);
    computer.rotate(0.05);
    computer.x();
    computer.apply();
  `
);

imageTest(
  'scale_x',
  `
    computer.scale(2);
    computer.x();
    computer.apply();
  `
);

// TODO
// imageTest(
//   'smear',
//   `
//     seed 9
//     rotate-color g 0.01
//     rotate 0.01
//     for 100
//       choose all circle cross square top x
//       apply
//     loop
//     rotate-color b 0.01
//     rotate 0.01
//     for 100
//       choose all circle cross square top x
//       apply
//     loop
//   `,
// );

imageTest(
  'square',
  `
    computer.square();
    computer.apply();
  `
);

imageTest(
  'square_top',
  `
    computer.square();
    computer.apply();
    computer.top();
    computer.apply();
 `
);

// TODO
// imageTest(
//   'starburst',
//   `
//     seed 8
//     rotate-color g 0.1
//     rotate 0.1
//     for 10
//       choose all circle cross square top x
//       apply
//     loop
//     rotate-color b 0.1
//     rotate 0.1
//     for 10
//       choose all circle cross square top x
//       apply
//     loop
//   `,
// );

imageTest(
  'top',
  `
    computer.top();
    computer.apply();
  `
);

imageTest(
  'x',
  `
    computer.x();
    computer.apply();
  `
);

imageTest(
  'x_loop',
  `
    computer.x();
    computer.scale(0.5);
    for (let i = 0; i < 8; i++) {
      computer.apply();
      computer.wrap();
    }
  `
);

imageTest(
  'x_scale',
  `
    computer.x();
    computer.scale(0.5);
    for (let i = 0; i < 8; i++) {
      computer.apply();
    }
  `
);

imageTest(
  'x_wrap',
  `
    computer.x();
    computer.apply();
    computer.scale(0.5);
    computer.wrap();
    computer.identity();
    computer.all();
    computer.apply();
  `
);

imageTest(
  'debug_operation',
  `
    computer.debug();
    computer.apply();
  `
);

imageTest(
  'mod_zero_is_always_false',
  `
    computer.mod(0, 1);
    computer.apply();
  `
);

imageTest(
  'square_colors',
  `
    computer.rotate(0.01);
    computer.rotateColor('g', 0.1);
    computer.square();
    for (let i = 0; i < 10; i++) {
      computer.apply();
    }
  `
);

imageTest(
  'nested_for_loops',
  `
    computer.circle();
    computer.scale(0.9);
    for (let i = 0; i < 2; i++) {
      for (let j = 0; j < 2; j++) {
        computer.apply();
      }
    }
  `
);

imageTest(
  'for_zero',
  `
    computer.circle();
    for (let i = 0; i < 0; i++) {
      computer.apply();
    }
  `
);

imageTest(
  'gpu_extra_pixels',
  `
    computer.rotate(0.01);
    computer.apply();
    computer.apply();
  `
);

imageTest(
  'default_color',
  `
    computer.defaultColor([255, 0, 255]);
    computer.rotate(0.01);
    computer.apply();
  `
);
