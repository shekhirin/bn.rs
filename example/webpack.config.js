const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

module.exports = {
    mode: process.env.NODE_ENV || 'development',
    target: 'node',
    entry: './index.ts',
    output: {
        path: __dirname + '/dist'
    },
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                loader: 'ts-loader'
            }
        ]
    },
    plugins: [
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, '.'),
            outDir: path.resolve(__dirname, 'pkg')
        }),
        // new HtmlWebpackPlugin(),
    ],
    resolve: {
        extensions: ['.js', '.wasm']
    },
    experiments: {
        asyncWebAssembly: true,
        topLevelAwait: true
    }
};
