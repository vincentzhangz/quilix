#!/usr/bin/env node

import { chmod, createWriteStream, existsSync, mkdirSync } from "fs";
import http from "http";
import https from "https";
import { dirname, resolve } from "path";
import { pipeline } from "stream/promises";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const platform = process.platform;
const arch = process.arch;

/**
 * Returns the binary name for the current platform.
 * @returns {string} Binary filename
 */
function getBinaryName() {
	if (platform === "darwin") {
		return arch === "arm64" ? "quilix-macos-arm64" : "quilix-macos-x64";
	}
	if (platform === "win32") {
		return "quilix-win-x64.exe";
	}
	return "quilix-linux-x64";
}

/**
 * Returns the full path to the binary.
 * @returns {string} Binary path
 */
function getBinaryPath() {
	const binaryName = getBinaryName();
	const installDir = resolve(__dirname, "..", "bin");
	return resolve(installDir, binaryName);
}

/**
 * Checks if the binary is already installed.
 * @returns {boolean} True if installed
 */
function isInstalled() {
	return existsSync(getBinaryPath());
}

/**
 * Downloads a file from URL to destination.
 * @param {string} url Download URL
 * @param {string} dest Destination path
 * @returns {Promise<void>}
 */
async function downloadFile(url, dest) {
	return new Promise((resolve, reject) => {
		const protocol = url.startsWith("https") ? https : http;
		const file = createWriteStream(dest);

		protocol
			.get(url, (response) => {
				if (response.statusCode === 302 || response.statusCode === 301) {
					file.close();
					downloadFile(response.headers.location, dest)
						.then(resolve)
						.catch(reject);
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
			})
			.on("error", reject);
	});
}

/**
 * Fetches release info for a specific tag.
 * @param {string} tagName GitHub release tag name
 * @returns {Promise<object>} Release info
 */
async function getReleaseInfo(tagName) {
	const url = `https://api.github.com/repos/vincentzhangz/quilix/releases/tags/${tagName}`;

	return new Promise((resolve, reject) => {
		https.get(url, { headers: { "User-Agent": "quilix-cli" } }, (res) => {
			if (res.statusCode !== 200) {
				reject(new Error(`Failed to get release info: ${res.statusCode}`));
				return;
			}

			let data = "";
			res.on("data", (chunk) => (data += chunk));
			res.on("end", () => {
				try {
					resolve(JSON.parse(data));
				} catch (e) {
					reject(e);
				}
			});
		}).on("error", reject);
	});
}

/**
 * Fetches the latest GitHub release.
 * @returns {Promise<object>} Latest release info
 */
async function tryGetLatestRelease() {
	const url = "https://api.github.com/repos/vincentzhangz/quilix/releases/latest";

	return new Promise((resolve, reject) => {
		https.get(url, { headers: { "User-Agent": "quilix-cli" } }, (res) => {
			if (res.statusCode !== 200) {
				reject(new Error(`Failed to get release info: ${res.statusCode}`));
				return;
			}

			let data = "";
			res.on("data", (chunk) => (data += chunk));
			res.on("end", () => {
				try {
					resolve(JSON.parse(data));
				} catch (e) {
					reject(e);
				}
			});
		}).on("error", reject);
	});
}

/**
 * Main entry point - downloads and installs the Quilix CLI binary.
 */
async function main() {
	const args = process.argv.slice(2);
	const isNightly = args.includes("--nightly") || args.includes("--experimental");

	if (isInstalled()) {
		console.log("Quilix CLI already installed.");
		return;
	}

	console.log("Downloading Quilix CLI...");

	let release;

	if (isNightly) {
		console.log("Looking for nightly release...");
		try {
			const data = await new Promise((resolve, reject) => {
				https.get(
					"https://api.github.com/repos/vincentzhangz/quilix/releases",
					{ headers: { "User-Agent": "quilix-cli" } },
					(res) => {
						let data = "";
						res.on("data", (chunk) => (data += chunk));
						res.on("end", () => {
							try {
								resolve(JSON.parse(data));
							} catch (e) {
								reject(e);
							}
						});
					},
				).on("error", reject);
			});

			const nightlyRelease = Array.isArray(data)
				? data.find((r) => r.tag_name.includes("nightly"))
				: null;

			if (nightlyRelease) {
				release = nightlyRelease;
				console.log(`Found nightly release: ${release.tag_name}`);
			} else {
				console.warn("No nightly release found.");
				console.warn("You can install via cargo: cargo install quilix-cli");
				return;
			}
		} catch {
			console.warn("Failed to fetch releases.");
			console.warn("You can install via cargo: cargo install quilix-cli");
			return;
		}
	} else {
		try {
			release = await tryGetLatestRelease();
			console.log(`Using latest release: ${release.tag_name}`);
		} catch {
			console.warn("Quilix CLI binary not available yet.");
			console.warn("You can install via cargo: cargo install quilix-cli");
			return;
		}
	}

	try {
		const binaryName = getBinaryName();
		const asset = release.assets.find((a) => a.name === binaryName);

		if (!asset) {
			console.warn(`Binary not found for platform: ${binaryName}`);
			console.warn("You can install via cargo: cargo install quilix-cli");
			return;
		}

		const installDir = resolve(__dirname, "..", "bin");
		mkdirSync(installDir, { recursive: true });

		console.log(`Downloading ${asset.name}...`);
		await downloadFile(asset.browser_download_url, getBinaryPath());
		console.log("Quilix CLI installed successfully!");
	} catch (error) {
		console.error("Failed to install Quilix CLI:", error.message);
		console.error("You can install via cargo: cargo install quilix-cli");
		process.exit(1);
	}
}

main();
