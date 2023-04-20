import { useEffect, forwardRef } from 'react'

import {
  Wrapper,
  StyledLabel,
  StyledSubLabel,
  StyledInput,
} from './TextField.styles'

const TextField = forwardRef(({
  label,
  subLabel,
  id,
  inline = false,
  setValue,
  ...props
}, ref) => {

  return (
    <Wrapper $inline={inline}>
      {label && <StyledLabel htmlFor={id} $inline={inline}>{label}</StyledLabel>}
      {subLabel && <StyledSubLabel htmlFor={id}>{subLabel}</StyledSubLabel>}
      <StyledInput id={id} ref={ref} {...props} />
    </Wrapper>
  )

})

export default TextField
