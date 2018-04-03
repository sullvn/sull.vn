import { CSSProperties } from 'react'
import styled from 'react-emotion'
import { transparentize } from 'polished'

import { purple, fade, halfFade } from '../style'

export const textLinkCSS: CSSProperties = {
  color: purple,
  fontWeight: 'bold',
  borderBottom: `.15em solid ${transparentize(fade, purple)}`,
  textDecoration: 'none',

  '&:hover': {
    borderBottomColor: transparentize(halfFade, purple),
  },
}

const TextLink = styled('a')(textLinkCSS)

export default TextLink
