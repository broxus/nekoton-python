import vue from '@vitejs/plugin-vue';

const HELP_URL = 'https://t.me/everdev';
const FEEDBACK_URL = '';
const GITHUB_URL = '';

module.exports = {
  title: 'Nekoton Python Docs',
  base: '/',
  description: 'nekoton-python',

  plugins: [vue()],
  rewrites: {
    'src/pages/index.md': 'index.md',
    'src/pages/concepts/data-representation.md': 'concepts/data-representation.md',
    'src/pages/concepts/abi.md': 'concepts/abi.md',
    'src/pages/guides/installation-and-quick-start.md': 'installation-and-quick-start.md',
    'src/pages/guides/keys-and-signatures.md': 'guides/keys-and-signatures.md',
    'src/pages/guides/working-with-cells.md': 'guides/working-with-cells.md',
    'src/pages/guides/working-with-abi.md': 'guides/working-with-abi.md',
  },
  themeConfig: {
    search: {
      provider: 'local',
    },
    nav: [
      { text: 'Feedback', link: FEEDBACK_URL },
      { text: 'Community', link: HELP_URL },
    ],
    sidebar: [
      { text: 'Overview', link: '/' },
      {
        text: 'Installation & Quick Start',
        link: '/installation-and-quick-start.md',
      },

      {
        text: 'Concepts',
        collapsable: false,
        items: [
          {
            text: 'Data Representation',
            link: '/concepts/data-representation.md',
          },
          { text: 'ABI', link: '/concepts/abi.md' },
        ],
      },

      {
        text: 'Guide',
        collapsable: false,

        items: [
          {
            text: 'Keys & Signatures',
            link: '/guides/keys-and-signatures.md',
          },
          {
            text: 'Working with Cells',
            link: '/guides/working-with-cells.md',
          },
          {
            text: 'Working with ABI',
            link: '/guides/working-with-abi.md',
          },
        ],
      },
    ],

    socialLinks: [{ icon: 'github', link: GITHUB_URL }],
  },

  esbuild: {
    target: ['chrome89', 'edge89', 'firefox79', 'safari14.1'],
  },
};
