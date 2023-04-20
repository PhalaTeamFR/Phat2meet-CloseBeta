import { styled } from 'goober'

export const Wrapper = styled('div')`
	margin: 30px 0;

	${props => props.inline && `
		margin: 0;
	`}
`;

export const StyledLabel = styled('label')`
	display: block;
	padding-bottom: 4px;
	font-size: 18px;

	${props => props.inline && `
		font-size: 16px;
	`}
`;

export const StyledSubLabel = styled('label')`
	display: block;
	padding-bottom: 6px;
	font-size: 13px;
	opacity: .6;
`;

export const StyledSelect = styled('select')`
	width: 100%;
	box-sizing: border-box;
	font: inherit;
	background: #000;
	color: inherit;
	padding: 10px 14px;
	border: 1px solid var(--secondary);
	box-shadow: inset 0 0 0 0 var(--secondary);
	border-radius: 3px;
	outline: none;
	transition: border-color .15s, box-shadow .15s;

	&:focus {
		border: 1px solid var(--secondary);
		box-shadow: inset 0 -3px 0 0 var(--secondary);
	}
`;
