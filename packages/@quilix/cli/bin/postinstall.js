#!/usr/bin/env node

import { createWriteStream, chmod, existsSync, mkdirSync } from 'fs';
import { createRequire } from 'module';
import { dirname, resolve } from 'path';
import { fileURLToPath } from 'url';
import { promisify } from 'util';
import { pipeline } from 'stream/promises';
import https from 'https';
import http from 'http';

const __dirname = dirname(fileURLToPath(import.meta.url));
const require = createRequire(import.meta.url);

// Get version from package.json
const pkg = require('../package.json');
const version = pkg.version;

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
  const installDir = resolve(__dirname, '..', 'bin');
  return resolve(installDir, binaryName);
}

function isInstalled() {
  const binaryPath = getBinaryPath();
  return existsSync(binaryPath);
}

async function downloadFile(url, dest) {
  return new Promise((resolve, reject) => {
    const protocol = url.startsWith('https') ? https : http;
    const file = createWriteStream(dest);

    protocol.get(url, (response) => {
      if (response.statusCode === 302 || response.statusCode === 301) {
        // Follow redirect
        file.close();
        downloadFile(response.headers.location, dest).then(resolve).catch(reject);
        return;
      }

      if (response.statusCode !== 200) {
        reject(new Error(`Failed to download: ${response.statusCode}`));
        return;
      }

      pipeline(response, file)
        .then(() => {
          chmod(dest, 0o755);
          resolve();
        })
        .catch(reject);
    }).on('error', reject);
  });
}

async function getReleaseInfo() {
  const owner = 'vincentzhangz';
  const repo = 'quilix';
  const url = `https://api.github.com/repos/${owner}/${repo}/releases/tags/v${version}`;

  return new Promise((resolve, reject) => {
    const req = https.get(url, {
      headers: { 'User-Agent': 'quilix-cli' }
    }, (res) => {
      if (res.statusCode !== 200) {
        reject(new Error(`Failed to get release info: ${res.statusCode}`));
        return;
      }

      let data = '';
      res.on('data', chunk => data += chunk);
      res.on('end', () => {
        try {
          resolve(JSON.parse(data));
        } catch (e) {
          reject(e);
        }
      });
    });
    req.on('error', reject);
  });
}

async function main() {
  const binaryPath = getBinaryPath();

  if (isInstalled()) {
    console.log('Quilix CLI already installed.');
    return;
  }

  console.log(`Downloading Quilix CLI v${version}...`);

  try {
    const release = await getReleaseInfo();
    const binaryName = getBinaryName();
    const asset = release.assets.find(a => a.name === binaryName);

    if (!asset) {
      console.warn(`Binary not found for platform: ${binaryName}`);
      console.warn('You can install via cargo: cargo install quilix-cli');
      return;
    }

    const installDir = resolve(__dirname, '..', 'bin');
    mkdirSync(installDir, { recursive: true });

    console.log(`Downloading ${asset.name}...`);
    await downloadFile(asset.browser_download_url, binaryPath);
    console.log('Quilix CLI installed successfully!');
  } catch (error) {
    if (error.message.includes('404') || error.message.includes('Failed to get release info')) {
      console.warn('Quilix CLI binary not available yet for this platform.');
      console.warn('You can install via cargo: cargo install quilix-cli');
      console.warn('Or build from source: cargo build --release --package quilix-cli');
      return;
    }
    console.error('Failed to install Quilix CLI:', error.message);
    console.error('You can install via cargo: cargo install quilix-cli');
    process.exit(1);
  }
}

main();
