import { styled } from 'goober'
import { forwardRef } from 'react'

export const Wrapper = styled('div')`
  margin: 30px 0;
`

export const StyledLabel = styled('label')`
  display: block;
  padding-bottom: 4px;
  font-size: 18px;
`

export const StyledSubLabel = styled('label')`
  display: block;
  padding-bottom: 6px;
  font-size: 13px;
  opacity: .6;
`

export const Range = styled('div', forwardRef)`
  user-select: none;
  background-color: var(--surface);
  border: 1px solid var(--primary);
  border-radius: 3px;
  height: 50px;
  position: relative;
  margin: 38px 6px 18px;
`

export const Handle = styled('div')`
  height: calc(100% + 20px);
  width: 20px;
  border: 1px solid var(--primary);
  background-color: var(--primary);
  border-radius: 3px;
  position: absolute;
  top: -10px;
  left: calc(${props => props.$value * 4.166}% - 11px);
  cursor: ew-resize;
  touch-action: none;
  transition: left .1s;
  @media (prefers-reduced-motion: reduce) {
    transition: none;
  }

  &:after {
    content: '|||';
    font-size: 8px;
    position: absolute;
    top: 0;
    left: 0;
    height: 100%;
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--shadow);
  }

  &:before {
    content: '${props => props.label}';
    position: absolute;
    bottom: calc(100% + 8px);
    text-align: center;
    left: 50%;
    transform: translateX(-50%);
    white-space: nowrap;
    ${props => props.$extraPadding}
  }
`

export const Selected = styled('div')`
  position: absolute;
  height: 100%;
  left: ${props => props.$start * 4.166}%;
  right: calc(100% - ${props => props.$end * 4.166}%);
  top: 0;
  background-color: var(--highlight);
  border-radius: 2px;
  transition: left .1s, right .1s;
  @media (prefers-reduced-motion: reduce) {
    transition: none;
  }
`
