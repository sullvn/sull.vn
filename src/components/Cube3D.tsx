import * as React from 'react'
import { keyframes } from 'glamor'
import glamorous, { CSSProperties } from 'glamorous'
import { darken } from 'polished'

import { purple } from '../style'


interface Cube3DProps {
  css?: CSSProperties 
  style?: React.CSSProperties
  className?: string
}

const faceWidth = '3em'


export default function Cube3D( props: Cube3DProps ): JSX.Element {
  return (
    <Container { ...props }>
      <Cube>
        <Face
          color={ purple }
        />
        <Face
          color={ darken( 0.05, purple ) }
          transform="rotateY(.25turn)"
        />
        <Face
          color={ darken( 0.1, purple ) }
          transform="rotateY(.5turn)"
        />
        <Face
          color={ darken( 0.05, purple ) }
          transform="rotateY(.75turn)"
        />
        <Face
          color={ darken( 0.15, purple ) }
          transform="rotateX(.25turn)"
        />
        <Face
          color={ darken( 0.15, purple ) }
          transform="rotateX(-.25turn)"
        />
      </Cube>
    </Container>
  )
}


const Container = glamorous.div({
  position: 'relative',

  width: `calc( 2 * ${ faceWidth })`,
  height: `calc( 2 * ${ faceWidth })`,

  perspective: '12em',
})


const Cube = glamorous.div({
  position: 'absolute',
  top: '50%',
  left: '50%',

  transformStyle: 'preserve-3d',

  animation: `60s linear 0s infinite normal ${ keyframes({
    from: {
      transform: 'rotateZ(.13turn) rotateX(.4turn) rotateY(.8turn)',
    },
    to: {
      transform: 'rotateZ(1.13turn) rotateX(.4turn) rotateY(1.8turn)',
    },
  }) }`,
})


const Face = glamorous.div({
  margin: `calc( -.5 * ${ faceWidth })`,
  marginBottom: `-${ faceWidth }`,
  width: faceWidth,
  height: faceWidth,

  backfaceVisibility: 'hidden',
}, ({ color, transform }) => ({
  backgroundColor: color || 'white',
  transform: `${ transform || '' } translateZ( calc(.5 * ${ faceWidth }))`,
}))
