import { Heading } from "@site/src/components/Heading";
import type { Props } from "@theme/MDXComponents/Heading";

export default function MDXHeading({ as, ...props }: Props): JSX.Element {
	// eslint-disable-next-line @typescript-eslint/no-unused-vars
	const _cls = "text-5xl text-4xl text-3xl text-2xl text-xl";
	return (
		// @ts-expect-error types
		<Heading level={Number.parseInt(as.replace("h", "")) as never} {...props} />
	);
}
