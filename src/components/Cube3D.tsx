import * as React from 'react'
import styled, { keyframes } from 'react-emotion'
import { darken } from 'polished'

import { purple } from '../style'

interface Cube3DProps {
  style?: React.CSSProperties
  className?: string
}

const faceWidth = '4em'

export default function Cube3D(props: Cube3DProps): JSX.Element {
  return (
    <Container {...props}>
      <Cube>
        <Face color={purple} />
        <Face color={darken(0.03, purple)} transform="rotateY(.25turn)" />
        <Face color={darken(0.06, purple)} transform="rotateY(.5turn)" />
        <Face color={darken(0.03, purple)} transform="rotateY(.75turn)" />
        <Face color={darken(0.09, purple)} transform="rotateX(.25turn)" />
        <Face color={darken(0.09, purple)} transform="rotateX(-.25turn)" />
      </Cube>
    </Container>
  )
}

const Container = styled('div')({
  position: 'relative',

  width: `calc( 2 * ${faceWidth})`,
  height: `calc( 2 * ${faceWidth})`,

  perspective: '20em',
})

const Cube = styled('div')({
  position: 'absolute',
  top: '50%',
  left: '50%',

  transformStyle: 'preserve-3d',

  animation: `121s linear 0s infinite normal ${keyframes({
    from: {
      transform: 'rotateZ(.13turn) rotateX(.4turn) rotateY(.8turn)',
    },
    to: {
      transform: 'rotateZ(1.13turn) rotateX(.4turn) rotateY(1.8turn)',
    },
  })}`,
})

interface FaceProps {
  color: string
  transform?: string
}

const Face = styled<FaceProps, 'div'>('div')(
  ({ color, transform }: FaceProps) => ({
    backgroundColor: color || 'white',
    transform: `${transform || ''} translateZ( calc(.5 * ${faceWidth}))`,
  }),
  {
    margin: `calc( -.5 * ${faceWidth})`,
    marginBottom: `-${faceWidth}`,
    width: faceWidth,
    height: faceWidth,

    backfaceVisibility: 'hidden',
  },
)
