const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports
const browserConfig = {
  mode: "production",
  entry: {
    index: "./js/index.js"
  },
  output: {
    path: dist,
    filename: "[name].js",
    globalObject: `(typeof self !== 'undefined' ? self : this)`
  },
  devServer: {
    contentBase: dist,
  },
  plugins: [
    new CopyPlugin([
      path.resolve(__dirname, "static")
    ]),
    new WasmPackPlugin({
      crateDirectory: __dirname,
      extraArgs: "-t no-modules --out-dir static/pkg --out-name index"
    }),
  ]
};

const workerConfig = {
  mode: "production",
  entry: "./js/worker.js",
  output: {
    path: dist,
    filename: "worker.js",
    globalObject: `(typeof self !== 'undefined' ? self : this)`
  }
};

module.exports = [browserConfig, workerConfig]
