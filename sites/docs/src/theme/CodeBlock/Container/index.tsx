import Container from "@theme-original/CodeBlock/Container";
import type ContainerType from "@theme/CodeBlock/Container";
import type { WrapperProps } from "@docusaurus/types";
import CodeWrap from "@site/src/components/CodeWrap";

type Props = WrapperProps<typeof ContainerType>;

export default function ContainerWrapper(props: Props): JSX.Element {
	return (
		<CodeWrap>
			<Container {...props} />
		</CodeWrap>
	);
}
