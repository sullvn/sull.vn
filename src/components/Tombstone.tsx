import * as React from 'react'
import glamorous from 'glamorous'

import { green, red, purple, quadrupleFade, Theme } from '../style'

export default function Tombstone(): JSX.Element {
  return (
    <StyledSVG width="100" height="30" viewBox="0 -1 45 12">
      <polygon points="0,10 5,0 10,10" />
      <rect x="0" y="0" width="10" height="10" transform="translate(16,0)" />
      <circle cx="5" cy="5" r="5.4" transform="translate(32,0)" />
    </StyledSVG>
  )
}

const StyledSVG = glamorous.svg(
  {
    opacity: 1 - quadrupleFade,

    ':hover': {
      opacity: 1,

      '& polygon': {
        fill: green,
      },
      '& rect': {
        fill: red,
      },
      '& circle': {
        fill: purple,
      },
    },
  },
  ({ theme }: { theme: Theme }) => ({
    fill: theme.fgColor,
  }),
)
