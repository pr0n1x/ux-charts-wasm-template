const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "production",
  experiments: {
    asyncWebAssembly: true
  },
  entry: {
    index: "./src/js/index.js"
  },
  output: {
    path: dist,
    filename: "[name].js"
  },
  devServer: {
    contentBase: dist,
    port: 3000,
    compress: true,
  },
  plugins: [
    new CopyPlugin({
      patterns: [
        {from: "static", to: "."} //"to" - relative to entry.output
      ]
    }),

    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ]
};
