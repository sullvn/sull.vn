import * as React from 'react'
import Helmet from 'react-helmet'
import styled from 'react-emotion'
import 'katex/dist/katex.min.css'

import Base from '../templates/Base'
import TextLink from '../components/TextLink'
import {
  fade,
  purple,
  articleTextWidth,
  contentStartSpacing,
  largeHeaderSize,
} from '../style'

interface BabblesPageProps {
  data: {
    allMarkdownRemark: {
      edges: Array<{
        node: Babble
      }>
    }
  }
}

interface Babble {
  id: string
  excerpt: string
  frontmatter: {
    title: string
    date: string
    path: string
  }
}

export default function BabblesPage(props: BabblesPageProps): JSX.Element {
  const { data } = props
  const babbles = data.allMarkdownRemark.edges.map(e => e.node)

  return (
    <Base>
      <Helmet title="Babbles" />
      <Section>
        <H1>Babbles</H1>
        <nav>
          <Ul>
            {babbles.map(b => (
              <li key={b.id}>
                <BabbleLink href={b.frontmatter.path}>
                  <Title>{b.frontmatter.title}</Title>
                  <Time>{b.frontmatter.date}</Time>
                </BabbleLink>
              </li>
            ))}
          </Ul>
        </nav>
      </Section>
    </Base>
  )
}

const Section = styled('section')({
  marginTop: `calc(${contentStartSpacing} / 2)`,
  fontSize: '1.5em',
})

const H1 = styled('h1')({
  fontSize: largeHeaderSize,
  marginBottom: `calc(${contentStartSpacing} / 2)`,
})

const Ul = styled('ul')({
  listStyle: 'none',
  margin: 0,
})

const BabbleLink = styled(TextLink)({
  display: 'flex',
  justifyContent: 'space-between',
  alignItems: 'flex-end',

  margin: '2em 0',
  paddingBottom: '.1em',

  ':hover': {
    borderColor: purple,
  },

  ':hover time': {
    opacity: 1,
  },

  [`@media(max-width: ${articleTextWidth})`]: {
    flexDirection: 'column',
    alignItems: 'flex-start',

    '& time': {
      margin: '.5em 0 0',
    },
  },
})

const Title = styled('span')({
  margin: 0,
})

const Time = styled('time')({
  marginLeft: '2em',
  flexShrink: 0,

  opacity: fade,
})

export const pageQuery = graphql`
  query BabblesQuery {
    allMarkdownRemark(sort: { order: DESC, fields: [frontmatter___date] }) {
      edges {
        node {
          id
          excerpt(pruneLength: 250)
          frontmatter {
            title
            date(formatString: "MMMM DD, YYYY")
            path
          }
        }
      }
    }
  }
`
