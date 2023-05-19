import * as React from 'react'
import Helmet from 'react-helmet'
import styled from '@emotion/styled'

import Base from '../templates/Base'
import {
  articleTextWidth,
  baseWidth,
  contentStartSpacing,
  foreground,
  largeHeaderSize,
  quadrupleFade,
} from '../style'

export default function ResumePage(): JSX.Element {
  return (
    <Base>
      <Helmet title="Resume" />
      <Resume>
        <Name>Kevin Sullivan</Name>
        <HilbertMark />

        <Job>
          <Company>Symbiote</Company>
          <Duration>2020 - 2022</Duration>
        </Job>
        <Task>
          <Role>Co-founder.</Role> Helped create plant soil sensor.
          product. Worked on app, cloud services, embedded software,
          manufacturing, packaging, shop, and media.
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
          Interactive geospatial interfaces for hyper-dense
          environmental data. Featured at the U.N. and New
          York Times: Cities for Tomorrow.
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

  [`@media print`]: {
    marginTop: `calc(${contentStartSpacing} / 4)`
  },
})

const Name = styled('h1')({
  gridColumn: '1 / 4',

  fontSize: largeHeaderSize,
  margin: 0,
  lineHeight: 0.8,
  justifySelf: 'end',

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

function Hilbert(props: React.HTMLAttributes<SVGSVGElement>): JSX.Element {
  return (
    <svg
      width="186"
      height="186"
      viewBox="0 0 186 186"
      xmlns="http://www.w3.org/2000/svg"
      {...props}
    >
    <path
      d="M0.600006 12.8857H25.1714V25.1714H0.600006V55.8857V86.6H37.4571V62.0286H49.7429V86.6H86.6V55.8857V25.1714H62.0286V12.8857H80.4572H98.8857V37.4571H135.743V12.8857H154.171H172.6V25.1714H148.029V62.0286H172.6V74.3143H154.171H135.743V49.7429H98.8857V80.4572V105.029V135.743H135.743V111.171H154.171H172.6V123.457H148.029V160.314H172.6V172.6H154.171H135.743V148.029H98.8857V172.6H80.4572H62.0286V160.314H86.6V129.6V98.8857H49.7429V123.457H37.4571V98.8857H0.600006V129.6V160.314H25.1714V172.6H0.600006V184.886H37.4571V148.029H12.8857V129.6V111.171H25.1714V135.743H62.0286V111.171H74.3143V129.6V148.029H49.7429V184.886H80.4572H111.171V160.314H123.457V184.886H154.171H184.886V148.029H160.314V135.743H184.886V98.8857H154.171H123.457V123.457H111.171V105.029V80.4572V62.0286H123.457V86.6H154.171H184.886V49.7429H160.314V37.4571H184.886V0.600006H154.171H123.457V25.1714H111.171V0.600006H80.4572H49.7429V37.4571H74.3143V55.8857V74.3143H62.0286V49.7429H25.1714V74.3143H12.8857V55.8857V37.4571H37.4571V0.600006H0.600006V12.8857Z"
    />
    </svg>
  )
}

const HilbertMark = styled(Hilbert)({
  height: '5.75rem',
  width: '5.75rem',
  justifySelf: 'end',
  marginTop: '0.25rem',
  fill: foreground,
  opacity: 1 - quadrupleFade,

  [`@media(max-width: ${baseWidth})`]: {
    display: 'none',
  },
})
