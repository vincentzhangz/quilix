#!/usr/bin/env node

import { spawn } from 'child_process';
import { resolve, dirname } from 'path';
import { fileURLToPath } from 'url';
import { existsSync } from 'fs';

const __dirname = dirname(fileURLToPath(import.meta.url));

// Detect platform
const platform = process.platform;
const arch = process.arch;

function getBinaryName() {
  const isWindows = platform === 'win32';
  const isMac = platform === 'darwin';

  if (isMac) {
    return arch === 'arm64' ? 'quilix-macos-arm64' : 'quilix-macos-x64';
  }
  if (isWindows) {
    return 'quilix-win-x64.exe';
  }
  // Linux (x64 only for now)
  return 'quilix-linux-x64';
}

function getBinaryPath() {
  const binaryName = getBinaryName();
  return resolve(__dirname, '..', 'bin', binaryName);
}

async function main() {
  const binaryPath = getBinaryPath();

  if (!existsSync(binaryPath)) {
    console.error('Quilix CLI not installed. Run `npm install` or `npx @quilix/cli` to install.');
    process.exit(1);
  }

  // Forward arguments to the Rust CLI
  const args = process.argv.slice(2);
  const child = spawn(binaryPath, args, {
    stdio: 'inherit',
    shell: platform === 'win32'
  });

  child.on('exit', (code) => {
    process.exit(code || 0);
  });
}

main();
