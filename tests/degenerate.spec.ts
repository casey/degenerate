import * as fs from 'fs';
import axios from 'axios';
import { exec } from './common';
import * as png from 'node-libpng';
import { test, expect, Page } from '@playwright/test';

const sleep = async (secs) => {
  return new Promise(resolve => setTimeout(resolve, secs * 1000));
}

test.beforeAll(async () => {
  let done = false;
  while (!done) {
    const res = await axios.get(`http://localhost:${process.env.PORT}`);
    done = res.status === 200;
    await sleep(1);
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
    await page.waitForSelector('canvas.ready');

    await page.locator('textarea').fill(program);

    await page.waitForSelector('canvas.done');

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

imageTest(
  'all',
  `
    computer.all();
    computer.render();
  `
);

imageTest(
  'alpha',
  `
    computer.alpha(0.5);
    computer.x();
    computer.render();
  `
);

imageTest(
  'render',
  `
    computer.render();
  `
);

imageTest(
  'brilliance',
  `
    computer.x();
    computer.rotateColor('g', 0.07);
    computer.rotate(0.07);
    for (let i = 0; i < 10; i++) {
      computer.render();
    }
    computer.rotateColor('b', 0.09);
    computer.rotate(0.09);
    for (let i = 0; i < 10; i++) {
      computer.render();
    }
  `
);

imageTest(
  'carpet',
  `
    computer.circle();
    computer.scale(0.5);
    for (let i = 0; i < 8; i++) {
      computer.render();
      computer.wrap();
    }
  `
);

imageTest(
  'circle',
  `
    computer.circle();
    computer.render();
  `
);

imageTest(
  'circle_scale',
  `
    computer.scale(0.5);
    computer.circle();
    computer.render();
    computer.all();
    computer.scale(0.9);
    computer.wrap();
    computer.render();
  `
);

imageTest(
  'concentric_circles',
  `
    computer.scale(0.99);
    computer.circle();
    for (let i = 0; i < 100; i++) {
      computer.render();
    }
  `
);

imageTest(
  'cross',
  `
    computer.cross();
    computer.render();
  `
);

imageTest('default_program', ` `);

imageTest(
  'diamonds',
  `
    computer.rotate(0.3333);
    computer.rotateColor('g', 0.05);
    computer.circle();
    computer.scale(0.5);
    computer.wrap();
    for (let i = 0; i < 8; i++) {
      computer.render();
    }
    computer.rotate(0.8333);
    computer.rotateColor('b', 0.05);
    for (let i = 0; i < 8; i++) {
      computer.render();
    }
  `
);

imageTest(
  'grain',
  `
    computer.rotate(0.111);
    for (let i = 0; i < 16; i++) {
      computer.square();
      computer.render();
      computer.circle();
      computer.render();
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
      computer.render();
    }
    computer.rotate(0.8333);
    computer.rotateColor('b', 0.05);
    for (let i = 0; i < 8; i++) {
      computer.render();
    }
  `
);

imageTest(
  'mod_3',
  `
    computer.mod(3, 0);
    computer.render();
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
      computer.render();
    }
    computer.rotateColor('b', 0.05);
    for (let i = 0; i < 8; i++) {
      computer.render();
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
      computer.render();
      computer.wrap();
    }
  `
);

imageTest(
  'choose_default_seed',
  `
    rng.choose([
      () => computer.all(),
      () => computer.circle(),
      () => computer.cross(),
      () => computer.square(),
      () => computer.top(),
      () => computer.x()
    ])();
    computer.render();
  `
);

imageTest(
  'choose_zero_seed',
  `
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
  `
);

imageTest(
  'choose_nonzero_seed',
  `
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
  `
);

imageTest(
  'rotate',
  `
    computer.rotate(0.05);
    computer.x();
    computer.render();
  `
);

imageTest(
  'rotate_0125_square',
  `
    computer.rotate(0.125);
    computer.square();
    computer.render();
  `
);

imageTest(
  'rotate_1_square',
  `
    computer.rotate(1.0);
    computer.square();
    computer.render();
  `
);

imageTest(
  'rotate_color_05_red',
  `
    computer.rotateColor('red', 0.5);
    computer.all();
    computer.render();
  `
);

imageTest(
  'rotate_color_blue_05_all',
  `
    computer.rotateColor('blue', 0.5);
    computer.all();
    computer.render();
  `
);

imageTest(
  'rotate_color_blue_1_all',
  `
    computer.rotateColor('blue', 1.0);
    computer.all();
    computer.render();
  `
);

imageTest(
  'rotate_color_blue_all',
  `
    computer.rotateColor('b', 0.5);
    computer.all();
    computer.render();
  `
);

imageTest(
  'rotate_color_g',
  `
    computer.rotateColor('g', 0.5);
    computer.all();
    computer.render();
  `
);

imageTest(
  'rotate_color_green',
  `
    computer.rotateColor('green', 0.5);
    computer.all();
    computer.render();
  `
);

imageTest(
  'rotate_color_green_all',
  `
    computer.rotateColor('green', 1.0);
    computer.all();
    computer.render();
  `
);

imageTest(
  'rotate_color_r',
  `
    computer.rotateColor('r', 0.5);
    computer.all();
    computer.render();
  `
);

imageTest(
  'rotate_color_red_all',
  `
    computer.rotateColor('red', 1.0);
    computer.all();
    computer.render();
  `
);

imageTest(
  'rotate_scale_x',
  `
    computer.rotate(0.05);
    computer.scale(2);
    computer.x();
    computer.render();
  `
);

imageTest(
  'rotate_square',
  `
    computer.rotate(0.05);
    computer.square();
    for (let i = 0; i < 2; i++) {
      computer.render();
    }
  `
);

imageTest(
  'rotate_square_for_x',
  `
    computer.rotate(0.05);
    computer.square();
    for (let i = 0; i < 2; i++) {
      computer.render();
    }
    computer.x();
    for (let i = 0; i < 1; i++) {
      computer.render();
    }
  `
);

imageTest(
  'rows',
  `
    computer.rows(1, 1);
    computer.render();
  `
);

imageTest(
  'rows_overflow',
  `
    computer.rows(4294967295, 4294967295);
    computer.render();
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
      computer.render();
    }
    computer.rotateColor('b', 0.05);
    for (let i = 0; i < 8; i++) {
      computer.render();
    }
  `
);

imageTest(
  'scale',
  `
    computer.scale(0.5);
    computer.circle();
    computer.render();
  `
);

imageTest(
  'scale_circle_for',
  `
    computer.circle();
    computer.scale(0.5);
    for (let i = 0; i < 8; i++) {
      computer.render();
    }
  `
);

imageTest(
  'scale_circle_wrap',
  `
    computer.scale(0.5);
    computer.circle();
    computer.wrap();
    computer.render();
  `
);

imageTest(
  'scale_rotate',
  `
    computer.scale(2);
    computer.rotate(0.05);
    computer.x();
    computer.render();
  `
);

imageTest(
  'scale_x',
  `
    computer.scale(2);
    computer.x();
    computer.render();
  `
);

imageTest(
  'smear',
  `
    const masks = ([
      () => computer.all(),
      () => computer.circle(),
      () => computer.cross(),
      () => computer.square(),
      () => computer.top(),
      () => computer.x()
    ]);
    rng.seed(9);
    computer.rotateColor('g', 0.01);
    computer.rotate(0.01);
    for (let i = 0; i < 100; i++) {
      rng.choose(masks)();
      computer.render();
    }
    computer.rotateColor('b', 0.01);
    computer.rotate(0.01);
    for (let i = 0; i < 100; i++) {
      rng.choose(masks)();
      computer.render();
    }
  `
);

imageTest(
  'square',
  `
    computer.square();
    computer.render();
  `
);

imageTest(
  'square_top',
  `
    computer.square();
    computer.render();
    computer.top();
    computer.render();
 `
);

imageTest(
  'starburst',
  `
    const masks = ([
      () => computer.all(),
      () => computer.circle(),
      () => computer.cross(),
      () => computer.square(),
      () => computer.top(),
      () => computer.x()
    ]);
    rng.seed(3);
    computer.rotateColor('g', 0.1);
    computer.rotate(0.1);
    for (let i = 0; i < 10; i++) {
      rng.choose(masks)();
      computer.render();
    }
    for (let i = 0; i < 10; i++) {
      rng.choose(masks)();
      computer.render();
    }
    computer.rotateColor('b', 0.1);
    computer.rotate(0.1);
    for (let i = 0; i < 10; i++) {
      rng.choose(masks)();
      computer.render();
    }
  `
);

imageTest(
  'top',
  `
    computer.top();
    computer.render();
  `
);

imageTest(
  'x',
  `
    computer.x();
    computer.render();
  `
);

imageTest(
  'x_loop',
  `
    computer.x();
    computer.scale(0.5);
    for (let i = 0; i < 8; i++) {
      computer.render();
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
      computer.render();
    }
  `
);

imageTest(
  'x_wrap',
  `
    computer.x();
    computer.render();
    computer.scale(0.5);
    computer.wrap();
    computer.identity();
    computer.all();
    computer.render();
  `
);

imageTest(
  'debug_operation',
  `
    computer.debug();
    computer.render();
  `
);

imageTest(
  'mod_zero_is_always_false',
  `
    computer.mod(0, 1);
    computer.render();
  `
);

imageTest(
  'square_colors',
  `
    computer.rotate(0.01);
    computer.rotateColor('g', 0.1);
    computer.square();
    for (let i = 0; i < 10; i++) {
      computer.render();
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
        computer.render();
      }
    }
  `
);

imageTest(
  'for_zero',
  `
    computer.circle();
    for (let i = 0; i < 0; i++) {
      computer.render();
    }
  `
);

imageTest(
  'gpu_extra_pixels',
  `
    computer.rotate(0.01);
    computer.render();
    computer.render();
  `
);

imageTest(
  'default_color',
  `
    computer.defaultColor([255, 0, 255]);
    computer.rotate(0.01);
    computer.render();
  `
);
