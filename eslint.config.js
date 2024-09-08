import typescriptParser from "@typescript-eslint/parser";
import unocss from "@unocss/eslint-config/flat";

export default [
    { files: ["**/*.tsx"] },
    {
        languageOptions: {
            parser: typescriptParser,
        },
    },
    {
        rules: {
            "semi": ["error", "always"],
            "comma-dangle": [
                "error",
                "always-multiline",
            ],
            "quotes": ["error", "double"],
        },
    },
    unocss,
];
