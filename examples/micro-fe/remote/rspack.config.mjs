import { defineConfig } from "@rspack/cli";
import rspackHtmlPlugin from "@rspack/plugin-html";
import { container } from "@rspack/core";

const HtmlPlugin =
  rspackHtmlPlugin.default?.default || rspackHtmlPlugin.default;

export default defineConfig({
  entry: {
    index: "./app/entry.tsx",
  },
  output: {
    distPath: { root: "dist" },
    filename: "[name].js",
    chunkFilename: "[name].js",
    uniqueName: "remote",
  },
  plugins: [
    new HtmlPlugin({ template: "./public/index.html" }),
    new container.ModuleFederationPlugin({
      name: "remote",
      filename: "remote.js",
      exposes: {
        "./Button": "./app/components/Button.tsx",
      },
      shared: ["react", "react-dom"],
    }),
  ],
  resolve: {
    extensions: [".tsx", ".ts", ".jsx", ".js", ".css"],
  },
  module: {
    rules: [
      {
        test: /\.(tsx|ts|jsx|js)$/,
        use: {
          loader: "builtin:swc-loader",
          options: {
            jsc: {
              parser: { syntax: "typescript", tsx: true },
              transform: { react: { runtime: "automatic" } },
            },
          },
        },
        exclude: /node_modules/,
      },
      {
        test: /\.css$/,
        use: ["postcss-loader"],
      },
    ],
  },
  experiments: {
    css: true,
  },
  devServer: {
    port: 3001,
    hot: true,
  },
});
