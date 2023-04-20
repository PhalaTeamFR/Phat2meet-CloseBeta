import { styled } from 'goober'

export const StyledMain = styled('div')`
  width: 800px;
  margin: 20px auto;
  max-width: calc(100% - 60px);
  magin-bottom : 20px

  * {
    text-align: left;
  }
`

export const StyledMatrix = styled('div')`
  min-height: 40rem;
`


export const StyledMainTest = styled('div')`
  width: 800px;
  margin: 20px auto;
  max-width: calc(100% - 60px);
  border: 1px solid #a52a2a3b;
  border-radius: 3px;
  padding: 10px 14px;
  background-color: #a52a2a3b;

  * {
    text-align: left;
  }
`

export const CreateForm = styled('form')`
  margin: 0 0 60px;
`

export const TitleSmall = styled('span')`
  display: block;
  margin: 0;
  font-size: 3rem;
  text-align: center;
  color: var(--secondary);
  line-height: 1em;
  text-transform: uppercase;
  text-align: left;

  ${props => !props.$altChars && `
    font-family: sans-serif;
    font-size: 2rem;
    font-weight: 600;
    line-height: 1.2em;
    padding-top: .3em;
  `}
`

export const TitleLarge = styled('h1')`
  margin: 0;
  font-size: 4rem;
  text-align: center;
  color: var(--primary);
  font-family: 'Lato-black', sans-serif;
  font-weight: 900;
  line-height: 1em;
  text-transform: uppercase;
  text-align: left;

  @media (max-width: 350px) {
    font-size: 3.5rem;
  }
`
export const Links = styled('nav')`
  text-align: center;
  margin: 20px 0;
`

export const AboutSection = styled('section')`
  margin: 30px 0 0;
  background-color: var(--surface);
  padding: 20px 0;

  & a {
    color: var(--secondary);
  }
`

export const P = styled('p')`
  font-weight: 500;
  line-height: 1.6em;
`

export const Stats = styled('div')`
  display: flex;
  justify-content: space-around;
  align-items: flex-start;
  flex-wrap: wrap;
`

export const Stat = styled('div')`
  text-align: center;
  padding: 0 6px;
  min-width: 160px;
  margin: 10px 0;
`

export const StatNumber = styled('span')`
  display: block;
  font-weight: 900;
  color: var(--secondary);
  font-size: 2em;
`

export const StatLabel = styled('span')`
  display: block;
`

export const OfflineMessage = styled('div')`
  text-align: center;
  margin: 50px 0 20px;
`

export const ButtonArea = styled('div')`
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: center;
  gap: 12px;
  margin: 30px 0;
`

export const VideoWrapper = styled('div')`
  margin: 0 auto;
  position: relative;
  padding-bottom: 56.4%;
  width: 100%;

  iframe {
    position: absolute;
    width: 100%;
    height: 100%;
    border-radius: 10px;
  }
`

export const VideoLink = styled('a')`
  display: block;
  text-decoration: none;
  position: relative;
  width: 100%;
  max-width: 400px;
  margin: 0 auto;
  transition: transform .15s;

  &:hover, &:focus {
    transform: translateY(-2px);
  }
  &:active {
    transform: translateY(-1px);
  }

  img {
    width: 100%;
    display: block;
    border-radius: 10px;
    background-color: #CCC;
  }
  span {
    color: #FFFFFF;
    position: absolute;
    top: 50%;
    font-size: 1.5rem;
    text-align: center;
    width: 100%;
    display: block;
    transform: translateY(-50%);
    text-shadow: 0 0 20px rgba(0,0,0,.8);
    user-select: none;

    &::before {
      content: '';
      display: block;
      height: 2em;
      width: 2em;
      background: currentColor;
      border-radius: 100%;
      margin: 0 auto .4em;
      background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='24' height='24' viewBox='0 0 24 24' fill='none' stroke='%23F79E00' stroke-width='2' stroke-linecap='round' stroke-linejoin='round' class='feather feather-play'%3E%3Cpolygon points='5 3 19 12 5 21 5 3'%3E%3C/polygon%3E%3C/svg%3E");
      background-position: center;
      background-repeat: no-repeat;
      background-size: 1em;
      box-shadow: 0 0 20px 0 rgba(0,0,0,.3);
    }
  }
`