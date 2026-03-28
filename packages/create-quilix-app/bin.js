#!/usr/bin/env node

import { createRequire } from 'module';
import { resolve, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const require = createRequire(import.meta.url);

/**
 * CLI tool to create a new Quilix application.
 * @param {string[]} args - Command line arguments
 */
async function main(args) {
  const projectName = args[0];

  if (!projectName) {
    console.error('Please provide a project name: npx create-quilix-app my-app');
    process.exit(1);
  }

  console.log(`Creating Quilix app: ${projectName}...`);

  const { handlebars } = await import('handlebars');
  const fs = await import('fs/promises');
  const pathModule = await import('path');

  const templateDir = pathModule.join(__dirname, '..', '@quilix', 'create-app', 'templates', 'basic');
  const targetDir = pathModule.resolve(process.cwd(), projectName);

  await copyTemplate(templateDir, targetDir, handlebars, fs, pathModule, projectName);

  console.log(`Created ${projectName} successfully!`);
  console.log(`\ncd ${projectName}`);
  console.log(`${getPackageManager()} install`);
  console.log(`${getPackageManager()} dev`);
}

/**
 * Recursively copies template files from src to dest.
 * Handles handlebars templates for .json files.
 */
async function copyTemplate(src, dest, handlebars, fs, pathModule, projectName) {
  const stats = await fs.stat(src);

  if (stats.isDirectory()) {
    await fs.mkdir(dest, { recursive: true });
    for (const file of await fs.readdir(src)) {
      await copyTemplate(
        pathModule.join(src, file),
        pathModule.join(dest, file),
        handlebars,
        fs,
        pathModule,
        projectName
      );
    }
  } else if (src.endsWith('.json')) {
    const content = await fs.readFile(src, 'utf-8');
    const template = handlebars.compile(content);
    const result = template({ name: projectName });
    await fs.writeFile(dest, result);
  } else {
    await fs.copyFile(src, dest);
  }
}

/**
 * Detects the package manager from environment.
 */
function getPackageManager() {
  if (process.env.npm_config_user_agent?.startsWith('pnpm')) {
    return 'pnpm';
  }
  if (process.env.npm_config_user_agent?.startsWith('yarn')) {
    return 'yarn';
  }
  if (process.env.npm_config_user_agent?.startsWith('bun')) {
    return 'bun';
  }
  return 'npm';
}

main(process.argv.slice(2));
