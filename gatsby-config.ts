import type { GatsbyConfig } from "gatsby";

const config: GatsbyConfig = {
  siteMetadata: {
    title: 'Kevin Sullivan',
    author: 'Kevin Sullivan',
    siteUrl: 'https://sull.vn',
    description: 'Code, art, and absurdity by Kevin Sullivan',
  },
  // More easily incorporate content into your pages through automatic TypeScript type generation and better GraphQL IntelliSense.
  // If you use VSCode you can also use the GraphQL plugin
  // Learn more at: https://gatsby.dev/graphql-typegen
  graphqlTypegen: true,
  plugins: [
    'gatsby-plugin-catch-links',
    'gatsby-plugin-emotion',
    'gatsby-plugin-react-helmet',
    'gatsby-plugin-sharp',
    'gatsby-plugin-sitemap',
    'gatsby-plugin-typescript',
    'gatsby-transformer-sharp',
    {
      resolve: 'gatsby-transformer-remark',
      options: {
        plugins: [
          'gatsby-remark-prismjs',
          'gatsby-remark-smartypants',
          'gatsby-remark-katex',
          {
            resolve: 'gatsby-remark-images',
            options: {
              maxWidth: 672,
              linkImagesToOriginal: false,
              backgroundColor: '#fffbf4',
            },
          },
        ],
      },
      __key: 'remark',
    },
    {
      resolve: 'gatsby-source-filesystem',
      options: {
        path: './src/babbles',
        name: 'babbles',
      },
      __key: 'babbles',
    },
    {
      resolve: 'gatsby-plugin-typography',
      options: {
        pathToConfigModule: 'src/config/typography.js',
      },
      __key: 'typography',
    },
  ],
}

export default config;
