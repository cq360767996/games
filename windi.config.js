const { defineConfig } = require('windicss/helpers');

module.exports = defineConfig({
  extract: {
    include: ['src/**/*.rs', 'src/**/*.css', 'index.html'],
    exclude: ['src/style/windi.css'],
  },
});
