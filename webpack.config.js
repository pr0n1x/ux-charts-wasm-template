const path = require("path");
const CopyPlugin = require("copy-webpack-plugin/dist/index").default;
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "production",
  experiments: {
    asyncWebAssembly: true
  },
  entry: {
    index: "./web/index.js"
  },
  output: {
    path: dist,
    filename: "[name].js"
  },
  optimization: {
    minimize: true
  },
  devServer: {
    contentBase: dist,
    port: 3000,
    compress: false,
  },
  module: {
    rules: [
      {
        test: /\.s[ac]ss$/i,
        use: [
          "style-loader", // Creates `style` nodes from JS strings
          "css-loader",   // Translates CSS into CommonJS
          "sass-loader",  // Compiles Sass to CSS
        ],
      },
    ],
  },
  resolve: {
    extensions: ['.js', '.scss', '.sass']
  },
  plugins: [
    new CopyPlugin({
      patterns: [
        {
          from: "web/assets",
          to: "assets", //"to" - relative to entry.output
          globOptions: {
            ignore: [".gitignore", ".gitkeep"]
          }
        }
      ]
    }),
    new HtmlWebpackPlugin({
      hash: true,
      template: "./web/index.html",
      filename: "./index.html" // relative to entry.output
    }),
    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),

  ],
};
