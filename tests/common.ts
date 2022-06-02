import { exec } from 'child_process';

const cmd = (command) => {
  exec(command, (err, _stdout, _stderr) => {
    if (err) throw `error: ${err.message}`;
  });
};

export { cmd };
