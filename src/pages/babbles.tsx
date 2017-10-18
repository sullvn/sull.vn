import * as React from 'react'
import Helmet from 'react-helmet'
import {
  default as glamorous,
  Section, H1, Ul, Time, Span,
  CSSProperties,
} from 'glamorous'

import Base from '../templates/Base'
import TextLink from '../components/TextLink'
import {
  fade, purple,
  articleTextWidth, contentStartSpacing, largeHeaderSize,
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


export default function BabblesPage( props: BabblesPageProps ): JSX.Element {
  const { data } = props
  const babbles = data.allMarkdownRemark.edges.map( e => e.node )

  return (
    <Base>
      <Helmet title="Babbles" />
      <Section css={ sectionStyle }>
        <H1 css={ h1Style }>Babbles</H1>
        <nav>
          <Ul css={ ulStyle }>
            { babbles.map( b => (
              <li key={ b.id }>
                <BabbleLink href={ b.frontmatter.path }>
                  <Span css={ spanStyle }>{ b.frontmatter.title }</Span>
                  <Time css={ timeStyle }>{ b.frontmatter.date }</Time>
                </BabbleLink>
              </li>
            )) }
          </Ul>
        </nav>
      </Section>
    </Base>
  )
}


const sectionStyle: CSSProperties = {
  marginTop: `calc(${ contentStartSpacing } / 2)`,
  fontSize: '1.5rem',
}

const h1Style: CSSProperties = {
  fontSize: largeHeaderSize,
  marginBottom: `calc(${ contentStartSpacing } / 2)`,
}

const ulStyle: CSSProperties = {
  listStyle: 'none',
  margin: 0,
}

const BabbleLink = glamorous( TextLink )({
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

  [ `@media(max-width: ${ articleTextWidth })` ]: {
    flexDirection: 'column',
    alignItems: 'flex-start',

    '& time': {
      margin: '.5em 0 0',
    },
  },
})

const spanStyle: CSSProperties = {
  margin: 0,
}

const timeStyle: CSSProperties = {
  marginLeft: '2em',
  flexShrink: 0,

  opacity: fade,
}


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
