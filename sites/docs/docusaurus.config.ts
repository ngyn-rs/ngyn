import {themes as prismThemes} from 'prism-react-renderer';
import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

const config: Config = {
  title: 'Ngyn',
  tagline: 'Dinosaurs are cool',
  favicon: 'https://avatars.githubusercontent.com/u/142031159?s=32&v=4',

  // Set the production url of your site here
  url: 'https://ngyn.rs',
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, it is often '/<projectName>/'
  baseUrl: '/',

  // GitHub pages deployment config.
  // If you aren't using GitHub pages, you don't need these.
  organizationName: 'ngyn-rs', // Usually your GitHub org/user name.
  projectName: 'ngyn', // Usually your repo name.

  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',

  // Even if you don't use internationalization, you can use this field to set
  // useful metadata like html lang. For example, if your site is Chinese, you
  // may want to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  plugins: [
    async function docusaurusTaillwind() {
      return {
        name: "docusaurus-tailwindcss",
        configurePostCss(postcssOptions) {
          // Appends TailwindCSS and AutoPrefixer.
          postcssOptions.plugins.push(require("tailwindcss"));
          postcssOptions.plugins.push(require("autoprefixer"));
          return postcssOptions;
        },
      };
    },
  [require.resolve('docusaurus-lunr-search'), {}]
  ],

  presets: [
    [
      'classic',
      {
        docs: {
          sidebarPath: './sidebars.ts',
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl:
            'https://github.com/ngyn-rs/ngyn/tree/main/sites/docs/',
        },
        theme: {
          customCss: './src/css/custom.css',
        },
      } satisfies Preset.Options,
    ],
  ],

  themeConfig: {
    // Replace with your project's social card
    image: 'img/docusaurus-social-card.jpg',
    announcementBar: {
      id: 'support_us',
      content:
        'We are looking to revamp our docs, please fill <a target="_blank" rel="noopener noreferrer" href="#">this survey</a>',
      backgroundColor: '#fafbfc',
      textColor: '#091E42',
      isCloseable: false,
    },
    navbar: {
      title: 'Ngyn',
      logo: {
        alt: 'Ngyn Logo',
        src: 'https://avatars.githubusercontent.com/u/142031159?s=200&v=4',
      },
      items: [
        {
          type: 'docsVersionDropdown',
        },
        {
          type: 'doc',
          docId: 'intro',
          position: 'right',
          label: 'Docs',
        },
        {
          type: 'doc',
          docId: 'intro',
          position: 'right',
          label: 'Showcase',
        },
        {
          href: 'https://github.com/ngyn-rs/ngyn',
          label: 'GitHub',
          position: 'right',
        },
      ],
    },
    footer: {
      style: 'light',
      copyright: `Made with ❤️ by Ngyn`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
