import * as React from 'react'
import Helmet from 'react-helmet'
import glamorous from 'glamorous'

import Base from '../templates/Base'
import { contentStartSpacing, lightTheme } from '../style'


export default function ResumePage(): JSX.Element {
  return (
    <Base theme={ lightTheme }>
      <Helmet title="Resume" />
      <Resume>
        <Name>Kevin Sullivan</Name>
        <Purpose>
          Hanging out at the intersection
          of technology and creativity.
        </Purpose>

        <Job columns="1/2" rows="2/3">
          <Company>
            Aclima
          </Company>
          <Duration>
            2015 - Now
          </Duration>
        </Job>
        <Task columns="2/3" rows="2/3">
          <Role>Engineer.</Role> Creating interactive
          geospatial visuals for hyper-dense
          environmental data. Includes custom WebGL
          shaders for Deck.GL.
        </Task>
        <Task columns="3/4" rows="2/3">
          Architected an in-browser metadata graph engine
          for flexible querying and display. Also setup a 
          versatile authentication platform for all of our
          user interfaces.
        </Task>
        <Task columns="4/5" rows="2/3">
          State of the art practices for fast, correct, and
          maintainable user interfaces. Includes static type
          systems, isomorphic rendering, continuous deployment,
          and more.
        </Task>

        <Job columns="1/2" rows="3/4">
          <Company>
            Amazon
          </Company>
          <Duration>
            2014 - 2015
          </Duration>
        </Job>
        <Task columns="2/3" rows="3/4">
          <Role>Engineer.</Role> Worked on a generative testing system for a point-of-sales
          mobile app and hardware.
        </Task>

        <Job columns="3/4" rows="3/4">
          <Company>
            PopularPays
          </Company>
          <Duration>
            2014
          </Duration>
        </Job>
        <Task columns="4/5" rows="3/4">
          <Role>Consulting.</Role> Started a social graph web crawler and complementary
          fraud detection system using PostgreSQL.
        </Task>

        <Job columns="1/2" rows="4/5">
          <Company>
            StudyCloud
          </Company>
          <Duration>
            2013
          </Duration>
        </Job>
        <Task columns="2/3" rows="4/5">
          <Role>Intern.</Role> Frontend engineering for a new learning management system design.
        </Task>

        <Job columns="3/4" rows="4/5">
          <Company>
            WolframAlpha
          </Company>
          <Duration>
            2011 - 2012
          </Duration>
        </Job>
        <Task columns="4/5" rows="4/5">
          <Role>Intern.</Role> Frontend engineering, prototyping, and authentication system.
        </Task>

        <Job columns="1/3" rows="5/6">
          <Company>
            University of Illinois at Urbana-Champaign
          </Company>
          <Duration>
            Class of 2014
          </Duration>
        </Job>
        <Task columns="3/4" rows="5/6">
          <Role>Computer Science major.</Role> Leader of WebMonkeys SIG for ACM.
        </Task>
        <Task columns="4/5" rows="5/6">
          Projects include architecting a basic processor, implementing an ML
          language, memory allocator, TCP stack, routing protocols, game AI,
          computer virus, and more.
        </Task>

        <Links columns="1/5" rows="6/7">
          <Link href="mailto:kevin.sullivan@axolotl.industries">
            kevin.sullivan@axolotl.industries
          </Link>
          <Link href="https://github.com/awfulaxolotl">
            github.com/awfulaxolotl
          </Link>
          <Link href="https://axolotl.industries">
            axolotl.industries
          </Link>
          <Link href="https://keybase.io/awfulaxolotl">
            keybase.io/awfulaxolotl
          </Link>
        </Links>
      </Resume>
    </Base>
  )
}


const SYSTEM_FONT_FAMILY = '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen-Sans, Ubuntu, Cantarell, "Helvetica Neue", sans-serif'

const Resume = glamorous.article({
  display: 'grid',
  grid: 'repeat(6, 1fr) / repeat(4, 1fr)',
  gridGap: '2rem 1rem',

  margin: `calc(${ contentStartSpacing } / 2) 0`
})

const Name = glamorous.h1({
  gridColumn: '1 / 4',
  gridRow: '1 / 2',

  fontSize: '72px',
  margin: 0,
  lineHeight: .8,
})

const Purpose = glamorous.p({
  gridColumn: '4 / 5',
  gridRow: '1 / 2',

  fontSize: '14px',
  fontFamily: SYSTEM_FONT_FAMILY,
  lineHeight: 1.45,
  margin: 0,
})

const Job = glamorous.div({
  display: 'flex',
  flexDirection: 'column',
}, ({ columns, rows }) => ({
  gridColumn: columns,
  gridRow: rows,
}))

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
}, ({ columns, rows }) => ({
  gridColumn: columns,
  gridRow: rows,
}))

const Role = glamorous.b({
})

const Links = glamorous.nav({
  display: 'flex',
  justifyContent: 'space-around',
}, ({ columns, rows }) => ({
  gridColumn: columns,
  gridRow: rows,
}))

const Link = glamorous.a({
  display: 'block',

  fontSize: '14px',
  fontFamily: SYSTEM_FONT_FAMILY,
  textAlign: 'center',
  color: 'inherit',
})
