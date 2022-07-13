const util = require('node:util');
const execFile = util.promisify(require('node:child_process').execFile);
import * as fs from 'fs';
import express from 'express';

export default async function () {
  const files = await fs.promises.opendir('../images');

  for await (const file of files) {
    const path = `../images/${file.name}`;
    if (path.endsWith('.actual-memory.png')) {
      await fs.promises.unlink(path);
    }
  }

  await execFile('cargo', ['build', '--target', 'wasm32-unknown-unknown']);

  await execFile(
    'wasm-bindgen',
    [
      '--target',
      'web',
      '--no-typescript',
      '--out-dir',
      'www',
      'target/wasm32-unknown-unknown/debug/degenerate.wasm',
    ],
    {
      cwd: '..',
    }
  );

  const app = express();
  app.use(express.static('../www'));

  const server = app.listen(0);
  process.env.PORT = server.address().port;

  return () => {
    server.close();
  };
}
