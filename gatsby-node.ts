import type { GatsbyNode } from 'gatsby'
import path from 'path'

export const createPages: GatsbyNode['createPages'] = async ({
  graphql,
  actions: { createPage },
}) => {
  const babbleTemplate = path.resolve('src/templates/Babble.tsx')

  const { data } = await graphql(`
    {
      allMarkdownRemark(
        sort: { frontmatter: { date: DESC }}
        limit: 1000
      ) {
        edges {
          node {
            excerpt(pruneLength: 250)
            html
            id
            fields {
              slug
            }
            frontmatter {
              date
              path
              title
            }
          }
        }
      }
    }
  `)

  const { edges } = (data as any).allMarkdownRemark

  for (const { node } of edges) {
    createPage({
      path: node.frontmatter.path,
      component: babbleTemplate,
      context: { id: node.id },
    })
  }
}

export const onCreateNode: GatsbyNode['onCreateNode'] = ({
  actions: { createNodeField },
  node,
}) => {
  if (node.internal.type === `MarkdownRemark`) {
    createNodeField({
      name: `slug`,
      node,
      value: (node as any).frontmatter.path,
    })
  }
}
