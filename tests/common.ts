import { exec } from 'child_process';
import { promisify } from 'util';

const cmd = async (command) => {
  await promisify(exec)(command);
};

export { cmd };
