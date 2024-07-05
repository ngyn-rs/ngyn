import globals from "globals";
import pluginJs from "@eslint/js";
import tseslint from "typescript-eslint";
import rawPluginReactConfig from "eslint-plugin-react/configs/recommended.js";
import { fixupConfigRules } from "@eslint/compat";

const pluginReactConfig = {
    ...rawPluginReactConfig,
    rules: {
        ...rawPluginReactConfig.rules,
        "react/react-in-jsx-scope": 0,
    },
};

const tsConfigs = tseslint.configs.recommended.map((c) => {
    if (c.rules && "@typescript-eslint/no-var-requires" in c.rules) {
        c.rules["@typescript-eslint/no-var-requires"] = 0;
    }
    return c;
});

export default [
    {
        ignores: [".docusaurus/*", "tailwind.config.js", "babel.config.js"],
    },
    { files: ["**/*.{js,mjs,cjs,ts,jsx,tsx}"] },
    { languageOptions: { parserOptions: { ecmaFeatures: { jsx: true } } } },
    { languageOptions: { globals: globals.browser } },
    pluginJs.configs.recommended,
    ...tsConfigs,
    ...fixupConfigRules(pluginReactConfig),
];
