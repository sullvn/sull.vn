module.exports = {
  siteMetadata: {
    title: 'Gatsby Test',
  },
  plugins: [
    'gatsby-plugin-glamor',
    'gatsby-plugin-catch-links',
    'gatsby-plugin-react-helmet',
    'gatsby-plugin-typescript',
    'gatsby-plugin-offline',
    {
      resolve: `gatsby-transformer-remark`,
      options: {
        plugins: [ 
          'gatsby-remark-prismjs',
          'gatsby-remark-smartypants',
          'gatsby-remark-katex',
          {
            resolve: 'gatsby-remark-images',
            options: {
              maxWidth: 864,
              linkImagesToOriginal: false,
              backgroundColor: '#fffbf4',
            },
          },
        ],
      },
    },
    {
      resolve: 'gatsby-source-filesystem',
      options: {
        path: `${ __dirname }/src/babbles`,
        name: 'babbles',
      },
    },
    {
      resolve: 'gatsby-plugin-typography',
      options: {
        pathToConfigModule: 'src/config/typography.js',
      },
    },
  ],
}
