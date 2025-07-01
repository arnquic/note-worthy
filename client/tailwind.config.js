module.exports = {
  purge: {
    mode: "all",
    content: [
      // include all rust, html and css files in the src directory
      "./src/**/*.rs",
      "./src/**/*.html",
      "./src/**/*.css",
      // include all html files in the output (dist) directory
      "./dist/**/*.html",
    ],
  }
};
