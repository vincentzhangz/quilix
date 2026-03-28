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
    uniqueName: "host",
  },
  plugins: [
    new HtmlPlugin({ template: "./public/index.html" }),
    new container.ModuleFederationPlugin({
      name: "host",
      remotes: {
        remote: "remote@http://localhost:3001/remote.js",
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
    port: 3000,
    hot: true,
  },
});
