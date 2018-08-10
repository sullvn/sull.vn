import * as React from 'react'
import styled from 'react-emotion'
import Helmet from 'react-helmet'

import Base from '../templates/Base'
import Cube3D from '../components/Cube3D'
import Disc3D from '../components/Disc3D'
import TextLink from '../components/TextLink'
import { articleTextWidth, contentStartSpacing } from '../style'

const waveTexture = require('../assets/wave-texture.jpg')
const portraitImage = require('../assets/portrait.png')

export default function IndexPage(): JSX.Element {
  return (
    <Base>
      <Helmet>
        <meta property="og:type" content="website" />
        <meta property="og:image" content={waveTexture} />
      </Helmet>
      <Section>
        <Texture>
          <Disc3D
            style={{
              position: 'absolute',
              left: '12%',
              bottom: '25%',
            }}
          />
          <Cube3D
            style={{
              position: 'absolute',
              left: '30%',
              bottom: '40%',
            }}
          />
          <Portrait src={portraitImage} />
        </Texture>
        <P>Oh hey there! I'm Kevin Sullivan.</P>
        <P>
          I do stuff at the intersection of technology and creativity.{' '}
          <TextLink href="/babbles">Sometimes I write about that</TextLink>.
        </P>
        <P>
          You can find me on{' '}
          <TextLink href="https://github.com/awfulaxolotl">GitHub</TextLink>,{' '}
          <TextLink href="https://instagram.com/awfulaxolotl/">
            Instagram
          </TextLink>
          , <TextLink href="https://dev.to/awfulaxolotl">DEV</TextLink>,{' '}
          <TextLink href="https://keybase.io/awfulaxolotl">Keybase</TextLink>,{' '}
          <TextLink href="https://twitter.com/awfulaxolotl">Twitter</TextLink>,
          or{' '}
          <TextLink href="mailto:kevin.sullivan@axolotl.industries">
            email
          </TextLink>
          . Say hello!
        </P>
        <P>
          What you can't find here are any cave salamanders. No refunds – sorry,
          not sorry!
        </P>
      </Section>
    </Base>
  )
}

const Section = styled('section')({
  marginTop: `calc(${contentStartSpacing} / 2)`,
})

const P = styled('p')({
  fontSize: '1.5em',

  lineHeight: 1.45,

  margin: '0 auto 2em',
  maxWidth: articleTextWidth,
})

// A direct parent is needed on the texture for it to blend correctly
const Texture = styled('div')({
  position: 'relative',
  height: '210px',
  padding: '0 30px 0 0',
  marginBottom: contentStartSpacing,

  background: `no-repeat left/cover url(${waveTexture})`,
  backgroundClip: 'content-box',
})

const Portrait = styled('img')({
  position: 'absolute',
  right: 0,
  bottom: '-30px',

  height: '300px',
  margin: 0,
})
