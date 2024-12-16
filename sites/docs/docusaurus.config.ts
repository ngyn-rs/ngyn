import { themes as prismThemes } from "prism-react-renderer";
import type { Config } from "@docusaurus/types";
import type * as Preset from "@docusaurus/preset-classic";

const config: Config = {
	title: "Ngyn",
	tagline: "Web Framework for ergonomic crabs",
	favicon: "https://avatars.githubusercontent.com/u/142031159?s=32&v=4",

	customFields: {
		chipSearchUrl: process.env.CHIP_SET_URL ?? "http://127.0.0.1:8000",
	},

	// Set the production url of your site here
	url: "https://ngyn.rs",
	// Set the /<baseUrl>/ pathname under which your site is served
	// For GitHub pages deployment, it is often '/<projectName>/'
	baseUrl: "/",

	onBrokenLinks: "ignore",
	onBrokenMarkdownLinks: "warn",

	// Even if you don't use internationalization, you can use this field to set
	// useful metadata like html lang. For example, if your site is Chinese, you
	// may want to replace "en" with "zh-Hans".
	i18n: {
		defaultLocale: "en",
		locales: ["en"],
	},
	themes: [],

	plugins: [
		async function docusaurusTaillwind() {
			return {
				name: "docusaurus-tailwindcss",
				configurePostCss(postcssOptions) {
					// Appends TailwindCSS and AutoPrefixer.
					// eslint-disable-next-line @typescript-eslint/no-require-imports
					postcssOptions.plugins.push(require("tailwindcss"));
					// eslint-disable-next-line @typescript-eslint/no-require-imports
					postcssOptions.plugins.push(require("autoprefixer"));
					return postcssOptions;
				},
			};
		},
	],

	presets: [
		[
			"classic",
			{
				docs: {
					sidebarPath: "./sidebars.ts",
					// Please change this to your repo.
					// Remove this to remove the "edit this page" links.
					editUrl: "https://github.com/ngyn-rs/ngyn/tree/main/sites/docs/",
				},
				theme: {
					customCss: "./src/css/custom.css",
				},
			} satisfies Preset.Options,
		],
	],

	themeConfig: {
		colorMode: {
			defaultMode: "dark",
			disableSwitch: true,
		},
		// Replace with your project's social card
		image: "img/docusaurus-social-card.jpg",
		announcementBar: {
			id: "support_us",
			content:
				"We are continuously looking for contributors. If you are interested in contributing, please reach out to us on GitHub.",
			backgroundColor: "#fafbfc",
			textColor: "#091E42",
			isCloseable: true,
		},
		navbar: {
			title: "ngyn",
			logo: {
				alt: "Ngyn Logo",
				src: "https://avatars.githubusercontent.com/u/142031159?s=200&v=4",
			},
			items: [
				{
					type: "docsVersionDropdown",
				},
				{
					type: "doc",
					docId: "index",
					label: "Docs",
				},
				{
					href: "https://github.com/ngyn-rs/ngyn/tree/main/examples",
					label: "Examples",
				},
				{
					href: "https://docs.rs/ngyn",
					label: "API Reference",
					position: "right",
				},
				{
					href: "https://showcase.ngyn.rs",
					label: "Showcase",
					position: "right",
				},
				{
					href: "https://github.com/orgs/ngyn-rs/discussions",
					label: "Discussions",
					position: "right",
				},
				{
					href: "https://github.com/ngyn-rs/ngyn",
					html: '<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="currentColor"><path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/></svg>',
					className: "navbar__icon",
					position: "right",
				},
			],
		},
		footer: {
			style: "light",
			copyright: "Made with ❤️ by Ngyn",
		},
		prism: {
			theme: prismThemes.oneLight,
			darkTheme: prismThemes.oneDark,
		},
	} satisfies Preset.ThemeConfig,
};

export default config;
