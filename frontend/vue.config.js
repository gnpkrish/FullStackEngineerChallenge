module.exports = {
  devServer: {
    proxy: {
      '^/api': {
        target: 'http://localhost:3000/',
        ws: true,
        changeOrigin: true
      },
      '^/graphql': {
        target: 'http://localhost:3000/'
      }
    }
  },
  pluginOptions: {
    i18n: {
      locale: "en",
      fallbackLocale: "en",
      localeDir: "locales",
      enableInSFC: true
    }
  }
};
