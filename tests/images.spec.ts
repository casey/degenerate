import * as fs from 'fs';
import * as path from 'path';
import * as png from 'fast-png';
import axios from 'axios';
import { test, expect, Page } from '@playwright/test';
const util = require('node:util');
const execFile = util.promisify(require('node:child_process').execFile);

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

async function run(page, script) {
  await page.locator('textarea').fill(script);
  await page.keyboard.press('Shift+Enter');
  await animation_frame(page);
}

async function animation_frame(page, script) {
  await page.keyboard.press('Control+Enter');
  await page.waitForSelector('html.done');

  let messages = await page.locator('samp > *');
  let count = await messages.count();

  await expect(count).toBe(0);
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
  await page.goto(`http://localhost:${process.env.PORT}/#/test`);
  await page.evaluate('window.preserveDrawingBuffer = true');
  page.on('pageerror', (error) => {
    console.log(error.message);
    throw error;
  });
  page.on('console', (message) => {
    if (process.env.VERBOSE || message.type() == 'error') {
      console.log(message);
    }
  });
  await page.waitForSelector('html.ready');
});

function imageTest(name, script) {
  test(name, async ({ page }) => {
    await run(page, script);

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
        await execFile('xattr', [
          '-wx',
          'com.apple.FinderInfo',
          '0000000000000000000C00000000000000000000000000000000000000000000',
          destination,
        ]);
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

// Press the \`Run\` button or \`Shift + Enter\` to execute`);
});

test('elapsed', async ({ page }) => {
  await run(
    page,
    `
      if (this.first == undefined) {
        this.first = elapsed();
      } else {
        let second = elapsed();
        if (second <= first) {
          throw "Arrow of time is broken!";
        }
      }
    `
  );
});

async function outerHTML(page, selector) {
  return await page.$$eval(selector, (elements) => elements[0].outerHTML);
}

test('slider', async ({ page }) => {
  let id = '#widget-slider-x';

  await run(page, `slider('x', 0, 1, 0.1, 0.5);`);

  await expect(await outerHTML(page, id)).toBe(
    '<label id="widget-slider-x">x<input type="range" min="0" max="1" step="0.1" value="0.5"><span>0.5</span></label>'
  );

  await run(page, `slider('x', 0, 1, 0.1, 0.5);`);

  await expect(await page.locator(id).count()).toBe(1);

  await run(
    page,
    'console.assert(slider("x", 0, 1, 0.1, 0.5) === 0.5, "expected 0.5")'
  );

  await page.fill(id, '0.2');

  await run(
    page,
    'console.assert(slider("x", 0, 1, 0.1, 0.5) === 0.2, "expected 0.2")'
  );
});

test('checkbox', async ({ page }) => {
  await run(
    page,
    `
      checkbox('x');
    `
  );

  await expect(await page.isChecked('#widget-checkbox-x')).toBeFalsy();

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

  await page.check('#widget-checkbox-x');

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

  await expect(await page.locator('#widget-checkbox-x').count()).toBe(1);
});

test('radio', async ({ page }) => {
  await run(
    page,
    `
      radio('foo', ['a', 'b', 'c']);
    `
  );

  await expect(await page.isChecked('#widget-radio-foo-a')).toBeTruthy();
  await expect(await page.isChecked('#widget-radio-foo-b')).toBeFalsy();
  await expect(await page.isChecked('#widget-radio-foo-c')).toBeFalsy();

  await run(
    page,
    `
      if (radio('foo', ['a', 'b', 'c']) == 'b') {
        render();
      }
    `
  );

  let off = png.decode(await imageBuffer(page)).data;
  await expect(off[0]).toEqual(0);

  await page.check('#widget-radio-foo-b');

  await run(
    page,
    `
      if (radio('foo', ['a', 'b', 'c']) == 'b') {
        render();
      }
    `
  );

  let on = png.decode(await imageBuffer(page)).data;
  await expect(on[0]).toEqual(255);

  await expect(await page.locator('#widget-radio-foo').count()).toBe(1);
});

test('radio-checkbox', async ({ page }) => {
  await run(
    page,
    `
      radio('foo', ['a', 'b', 'c']);
      checkbox('foo');
    `
  );

  await expect(await page.locator('#widget-radio-foo').count()).toBe(1);
  await expect(await page.locator('#widget-checkbox-foo').count()).toBe(1);
});

test('radio-initial', async ({ page }) => {
  await run(
    page,
    `
      if (radio('foo', ['a', 'b', 'c']) != 'a') {
        throw 'Incorrect return value';
      }
    `
  );
});

test('widgets-are-removed-on-new-script', async ({ page }) => {
  await run(page, "checkbox('x');");

  await expect(await page.locator('#widget-checkbox-x').count()).toBe(1);

  await run(page, '');

  await expect(await page.locator('#widget-checkbox-x').count()).toBe(0);
});

test('delta', async ({ page }) => {
  await run(
    page,
    `
      if (delta() === 0) {
        throw "Frame delta was zero.";
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

  await page.locator('button', { hasText: 'run' }).click();

  await page.waitForSelector('html.done');

  let on = png.decode(await imageBuffer(page)).data;
  await expect(on[0]).toEqual(255);
});

test('assert-fail', async ({ page }) => {
  test.fail();
  await run(page, 'assert(false)');
});

test('throw-fail', async ({ page }) => {
  test.fail();
  await run(page, "throw 'foobar'");
});

test('js-error-fail', async ({ page }) => {
  test.fail();
  await run(page, 'foo');
});

test('throw-samp', async ({ page }) => {
  try {
    await run(page, "throw 'foobar'");
  } catch {}
  await expect(await page.locator('samp > *')).toHaveText('foobar');
});

test('js-error-samp', async ({ page }) => {
  try {
    await run(page, 'foo');
  } catch {}
  await expect(await page.locator('samp > *')).toHaveText(
    'ReferenceError: foo is not defined'
  );
});
