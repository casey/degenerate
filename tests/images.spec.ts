import * as fs from 'fs';
import * as path from 'path';
import * as png from 'fast-png';
import axios from 'axios';
import { exec } from './common';
import { test, expect, Page } from '@playwright/test';

async function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function imageBuffer(page) {
  return Buffer.from(
    (
      await page.evaluate(() =>
        document.getElementsByTagName('canvas')[0].toDataURL()
      )
    ).slice('data:image/png;base64,'.length),
    'base64'
  );
}

async function run(page, program) {
  await page.locator('textarea').fill(program);
  await page.keyboard.down('Shift');
  await page.keyboard.press('Enter');
  await page.waitForSelector('html.done');
}

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
  page.on('pageerror', (error) => {
    console.log(error.message);
    throw error;
  });
  page.on('console', (message) => {
    if (process.env.VERBOSE || message.type() == 'error') console.log(message);
  });
  await page.waitForSelector('html.ready');
});

function imageTest(name, program) {
  test(name, async ({ page }) => {
    await run(page, program);

    const encoded = await imageBuffer(page);

    const have = png.decode(encoded).data;

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

      await fs.promises.writeFile(destination, encoded);

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
}

const tests = fs
  .readdirSync('../features')
  .map((filename) => path.parse(filename))
  .reduce((tests, path) => {
    tests[path.name] = fs.readFileSync(`../features/${path.base}`).toString();
    return tests;
  }, {});

for (const test in tests) {
  imageTest(test, tests[test]);
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
  await page.selectOption('select', { label: 'All' });

  await expect(await page.locator('textarea')).toHaveValue(`reboot();

all();

render();

// Press \`Shift + Enter\` to execute`);
});

test('elapsed', async ({ page }) => {
  await run(
    page,
    `
      let first = elapsed();
      await sleep(100);
      let second = elapsed();

      if (second <= first) {
        throw "Arrow of time is broken!";
      }
    `
  );
});

test('checkbox', async ({ page }) => {
  await run(
    page,
    `
      checkbox('x');
    `
  );

  await expect(await page.isChecked('#widget-x')).toBeFalsy();

  await run(
    page,
    `
      if (checkbox('x')) {
        render();
      }
    `
  );

  let off = png.decode(await imageBuffer(page)).data;
  await expect(off[0]).toEqual(0);

  await page.check('#widget-x');

  await run(
    page,
    `
      if (checkbox('x')) {
        render();
      }
    `
  );

  let on = png.decode(await imageBuffer(page)).data;
  await expect(on[0]).toEqual(255);

  await expect(await page.locator('#widget-x').count()).toBe(1);
});

test('delta', async ({ page }) => {
  await run(
    page,
    `
      let x = delta();
      if (x === 0) {
        throw "Frame delta was zero: " + x;
      }
    `
  );
});

test('run', async ({ page }) => {
  await page.locator('textarea').fill(
    `
      render();
    `
  );

  await page.locator('text=run').click();

  await page.waitForSelector('html.done');

  let on = png.decode(await imageBuffer(page)).data;
  await expect(on[0]).toEqual(255);
});
