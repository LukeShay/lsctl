import {exec} from 'node:child_process';
import {promisify} from 'node:util';
import process from 'node:process';

const execAsync = promisify(exec);

const run = async () => {
    const {stdout, stderr} = await execAsync(
        `curl -fsSL https://raw.githubusercontent.com/lukeshay/lsctl/main/install.sh | LSCTL_INSTALL=$(pwd) sh -s -- v${process.env.npm_package_version}`
    );

    console.log(stdout);
    console.log(stderr);
};

void run();
