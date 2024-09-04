module.exports = {
    content: {
        files: ["*.html", "./src/**/*.rs"],
    },
    darkMode: "class",
    theme: {
        extend: {
            fontFamily: {
                sans: [
                    "Inter",
                    "-apple-system",
                    "BlinkMacSystemFont",
                    '"Segoe UI"',
                    "Roboto",
                    "Oxygen-Sans",
                    "Ubuntu",
                    "Cantarell",
                    '"Helvetica Neue"',
                    "sans-serif",
                ],
            },
        },
    },
};
