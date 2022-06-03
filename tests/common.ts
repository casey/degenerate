import { exec as nodeExec } from 'child_process';
import { promisify } from 'util';

const exec = async (command) => {
  await promisify(nodeExec)(command);
};

export { exec };
