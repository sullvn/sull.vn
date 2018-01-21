import * as React from 'react'
import Helmet from 'react-helmet'
import glamorous from 'glamorous'

import Base from '../templates/Base'
import {
  articleTextWidth,
  baseWidth,
  contentStartSpacing,
  lightTheme,
  largeHeaderSize,
} from '../style'

export default function ResumePage(): JSX.Element {
  return (
    <Base theme={lightTheme}>
      <Helmet title="Resume" />
      <Resume>
        <Name>Kevin Sullivan</Name>
        <Purpose>
          Empowering others at the intersection of technology and creativity.
        </Purpose>

        <Job smallRow="3/6">
          <Company>Aclima</Company>
          <Duration>2015 - Now</Duration>
        </Job>
        <Task>
          <Role>Engineer.</Role> Creating interactive geospatial visuals for
          hyper-dense environmental data in WebGL. Featured work at the United
          Nations and New York Times: Cities for Tomorrow.
        </Task>
        <Task>
          Architected an in-browser metadata graph engine for flexible querying
          and display. Also setup a versatile authentication platform for all of
          our user interfaces.
        </Task>
        <Task>
          State of the art practices for fast, correct, and maintainable user
          interfaces. Includes static type systems, isomorphic rendering,
          continuous deployment, and more.
        </Task>

        <Job>
          <Company>Amazon</Company>
          <Duration>2014 - 2015</Duration>
        </Job>
        <Task>
          <Role>Engineer.</Role> Worked on a generative testing system for a
          point-of-sales mobile app and hardware.
        </Task>

        <Job>
          <Company>
            Popular<wbr />Pays
          </Company>
          <Duration>2014</Duration>
        </Job>
        <Task>
          <Role>Consulting.</Role> Started a social graph web crawler and
          complementary fraud detection system using PostgreSQL.
        </Task>

        <Job>
          <Company>
            Study<wbr />Cloud
          </Company>
          <Duration>2013</Duration>
        </Job>
        <Task>
          <Role>Intern.</Role> Frontend engineering for a new learning
          management system design.
        </Task>

        <Job>
          <Company>
            Wolfram<wbr />Alpha
          </Company>
          <Duration>2011 - 2012</Duration>
        </Job>
        <Task>
          <Role>Intern.</Role> Frontend engineering, prototyping, and
          authentication system.
        </Task>

        <Education>
          <Company>University of Illinois at Urbana-Champaign</Company>
          <Duration>Class of 2014</Duration>
        </Education>
        <Task>
          <Role>Computer Science major.</Role> Leader of WebMonkeys SIG for ACM.
        </Task>
        <Task>
          Projects include architecting a basic processor, implementing an ML
          language, memory allocator, TCP stack, routing protocols, game AI,
          computer virus, and more.
        </Task>

        <Links>
          <Link href="mailto:kevin.sullivan@axolotl.industries">
            kevin.sullivan@axolotl.industries
          </Link>
          <Link href="https://github.com/awfulaxolotl">
            github.com/awfulaxolotl
          </Link>
          <Link href="https://axolotl.industries">axolotl.industries</Link>
          <Link href="https://keybase.io/awfulaxolotl">
            keybase.io/awfulaxolotl
          </Link>
        </Links>
      </Resume>
    </Base>
  )
}

const SYSTEM_FONT_FAMILY =
  '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen-Sans, Ubuntu, Cantarell, "Helvetica Neue", sans-serif'

const Resume = glamorous.article({
  display: 'grid',
  grid: 'repeat(6, 1fr) / repeat(4, 1fr)',
  gridGap: '2rem 1rem',

  margin: `calc(${contentStartSpacing} / 2) 0`,

  [`@media(max-width: ${baseWidth})`]: {
    gridTemplateRows: 'repeat(12, auto)',
    gridTemplateColumns: 'repeat(2, 1fr)',

    maxWidth: articleTextWidth,
    marginLeft: 'auto',
    marginRight: 'auto',
  },
})

const Name = glamorous.h1({
  gridColumn: '1 / 4',

  fontSize: largeHeaderSize,
  margin: 0,
  lineHeight: 0.8,

  [`@media(max-width: ${baseWidth})`]: {
    gridColumn: '1 / 3',
  },
})

const Purpose = glamorous.p({
  fontSize: '14px',
  fontFamily: SYSTEM_FONT_FAMILY,
  lineHeight: 1.45,
  margin: 0,

  [`@media(max-width: ${baseWidth})`]: {
    gridColumn: '1 / 3',
  },
})

const Job = glamorous.div(
  {
    display: 'flex',
    flexDirection: 'column',
  },
  ({ smallRow }) => ({
    [`@media(max-width: ${baseWidth})`]: {
      gridRow: smallRow,
    },
  }),
)

const Company = glamorous.h2({
  textAlign: 'right',
  margin: 0,
})

const Duration = glamorous.time({
  textAlign: 'right',
  margin: 0,
})

const Task = glamorous.p({
  fontSize: '14px',
  fontFamily: SYSTEM_FONT_FAMILY,
  lineHeight: 1.45,
  margin: 0,
})

const Role = glamorous.b({})

const Education = glamorous.div({
  display: 'flex',
  flexDirection: 'column',

  gridColumn: '1 / 3',

  [`@media(max-width: ${baseWidth})`]: {
    gridColumn: '1 / 2',
    gridRow: '10 / 12',
  },
})

const Links = glamorous.nav({
  display: 'flex',
  justifyContent: 'space-around',

  gridColumn: '1/5',

  [`@media(max-width: ${baseWidth})`]: {
    flexDirection: 'column',
    gridColumn: '1 / 3',
  },
})

const Link = glamorous.a({
  display: 'block',

  color: 'inherit',
  fontSize: '14px',
  fontFamily: SYSTEM_FONT_FAMILY,
  textAlign: 'center',
  textDecoration: 'none',
})
