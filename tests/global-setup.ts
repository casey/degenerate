import * as fs from 'fs';
import express from 'express';
import { cmd } from './common';

const clean = async () => {
  const files = await fs.promises.opendir('../images');

  for await (const file of files) {
    const path = `../images/${file.name}`;
    if (path.endsWith('.actual-memory.png')) {
      await fs.promises.unlink(path);
    }
  }
};

const buildWasm = async () => {
  await cmd(`
    cd ..
    cargo build --target wasm32-unknown-unknown
    wasm-bindgen --target web --no-typescript \
      target/wasm32-unknown-unknown/debug/degenerate.wasm \
      --out-dir www
  `);
};

const runServer = () => {
  const app = express();
  app.use(express.static('../www'));

  const server = app.listen(0);
  process.env.PORT = server.address().port;

  return server;
};

async function globalSetup() {
  await clean();

  await buildWasm();

  const server = runServer();

  return () => {
    server.close();
  };
}

export default globalSetup;
