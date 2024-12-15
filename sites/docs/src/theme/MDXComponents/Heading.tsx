import { Heading } from "@site/src/components/Heading";
import type { Props } from "@theme/MDXComponents/Heading";

export default function MDXHeading({ as, ...props }: Props): JSX.Element {
	return (
		<Heading level={Number.parseInt(as.replace("h", "")) as never} {...props} />
	);
}
