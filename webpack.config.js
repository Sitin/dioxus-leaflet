const path = require('path');

module.exports = {
    entry: path.resolve(__dirname, 'bindings', 'src', 'index.ts'),
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                use: 'ts-loader',
                exclude: /node_modules/,
            },
        ],
    },
    resolve: {
        extensions: ['.ts', '.js'],
    },
    output: {
        filename: 'bindings.js',
        path: path.resolve(__dirname, 'bindings', 'dist'),
    },
    optimization: {
        minimize: true,
        mangleExports: true,
    },
};
