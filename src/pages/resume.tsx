import * as React from 'react'
import Helmet from 'react-helmet'
import styled from '@emotion/styled'

import Base from '../templates/Base'
import {
  articleTextWidth,
  baseWidth,
  contentStartSpacing,
  largeHeaderSize,
} from '../style'

export default function ResumePage(): JSX.Element {
  return (
    <Base>
      <Helmet title="Resume" />
      <Resume>
        <Name>Kevin Sullivan</Name>
        <Purpose>Because data and design deserve each other.</Purpose>

        <Job>
          <Company>Symbiote</Company>
          <Duration>2020 - 2022</Duration>
        </Job>
        <Task>
          <Role>Co-founder.</Role> Helped start a plant sensor business.
          Worked on app, cloud services, embedded software, manufacturing,
          packaging design, online shop, and media.
        </Task>

        <Job>
          <Company>Uber ATG</Company>
          <Duration>2018 - 2020</Duration>
        </Job>
        <Task>
          Built tools for artificial intelligence research in autonomous
          driving.
        </Task>

        <Job>
          <Company>Aclima</Company>
          <Duration>2015 - 2018</Duration>
        </Job>
        <Task>
          Created interactive geospatial interfaces for hyper-dense
          environmental data. Featured work at the United Nations and New York
          Times: Cities for Tomorrow.
        </Task>

        <Job>
          <Company>Amazon</Company>
          <Duration>2014 - 2015</Duration>
        </Job>
        <Task>
          Worked on a generative testing system for a point-of-sales mobile app
          and hardware.
        </Task>

        <Job>
          <Company>
            Popular
            <wbr />
            Pays
          </Company>
          <Duration>2014</Duration>
        </Job>
        <Task>
          <Role>Consulting.</Role> Started a social graph web crawler and
          complementary fraud detection system using PostgreSQL.
        </Task>

        <Job>
          <Company>
            Study
            <wbr />
            Cloud
          </Company>
          <Duration>2013</Duration>
        </Job>
        <Task>
          <Role>Intern.</Role> Frontend engineering for a new learning
          management system design.
        </Task>

        <Job>
          <Company>
            Wolfram
            <wbr />
            Alpha
          </Company>
          <Duration>2011 - 2012</Duration>
        </Job>
        <Task>
          <Role>Intern.</Role> Frontend engineering, prototyping, and
          authentication system.
        </Task>

        <Education>
          <Company>University of Illinois<br />Urbana-Champaign</Company>
          <Major>B.S. in Computer Science</Major>
          <Duration>Class of 2014</Duration>
        </Education>

        <Links>
          <Link href="mailto:kevin@sull.vn">kevin@sull.vn</Link>
          <Link href="https://github.com/sullvn">github.com/sullvn</Link>
          <Link href="https://sull.vn">sull.vn</Link>
        </Links>
      </Resume>
    </Base>
  )
}

const SYSTEM_FONT_FAMILY =
  '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen-Sans, Ubuntu, Cantarell, "Helvetica Neue", sans-serif'

const Resume = styled('article')({
  display: 'grid',
  grid: 'repeat(6, 1fr) / repeat(2, 3fr 4fr)',
  gridGap: '2rem 1rem',

  margin: `calc(${contentStartSpacing} / 2) 0`,

  [`@media(max-width: ${baseWidth})`]: {
    gridTemplateRows: 'repeat(12, auto)',
    gridTemplateColumns: 'repeat(2, auto)',

    maxWidth: articleTextWidth,
    marginLeft: 'auto',
    marginRight: 'auto',
  },
})

const Name = styled('h1')({
  gridColumn: '1 / 4',

  fontSize: largeHeaderSize,
  margin: 0,
  lineHeight: 0.8,

  [`@media(max-width: ${baseWidth})`]: {
    gridColumn: '1 / 3',
  },
})

const Purpose = styled('p')({
  fontSize: '14px',
  fontFamily: SYSTEM_FONT_FAMILY,
  lineHeight: 1.45,
  margin: 0,

  [`@media(max-width: ${baseWidth})`]: {
    gridColumn: '1 / 3',
  },
})

const Job = styled('div')({
  display: 'flex',
  flexDirection: 'column',
})

const Company = styled('h2')({
  textAlign: 'right',
  margin: 0,
})

const Duration = styled('time')({
  textAlign: 'right',
  margin: 0,
})

const Task = styled('p')({
  fontSize: '14px',
  fontFamily: SYSTEM_FONT_FAMILY,
  lineHeight: 1.45,
  margin: 0,
})

const Role = styled('b')({})

const Major = styled('b')({
  textAlign: 'right',
  margin: 0,
})

const Education = styled('div')({
  display: 'flex',
  flexDirection: 'column',

  gridColumnEnd: 'span 2',

  [`@media(max-width: ${baseWidth})`]: {
    alignItems: 'center',
  },
})

const Links = styled('nav')({
  display: 'flex',
  justifyContent: 'space-around',

  gridColumn: '1/5',

  [`@media(max-width: ${baseWidth})`]: {
    flexDirection: 'column',
    gridColumn: '1 / 3',
  },
})

const Link = styled('a')({
  display: 'block',

  color: 'inherit',
  fontSize: '14px',
  fontFamily: SYSTEM_FONT_FAMILY,
  textAlign: 'center',
  textDecoration: 'none',
})
