import clsx from "clsx";
import { Highlight, themes } from "prism-react-renderer";
import { useColorMode } from "@docusaurus/theme-common";

type Props = {
	children: React.ReactNode;
	className?: string;
};

export function CodeBlock({
	code,
	language = "rust",
	className,
	hideLineNumber = false,
}: {
	code: string;
	language?: string;
	className?: string;
	hideLineNumber?: boolean;
}): JSX.Element {
	const { colorMode } = useColorMode();
	return (
		<CodeWrap className={className}>
			<Highlight
				theme={colorMode === "dark" ? themes.oneDark : themes.oneLight}
				code={code}
				language={language}
			>
				{({ className, style, tokens, getLineProps, getTokenProps }) => (
					<pre
						className={clsx(className, "block  flex-auto mb-0 rounded-t-none")}
						style={style}
					>
						{tokens.map((line, i) => (
							// biome-ignore lint/suspicious/noArrayIndexKey: <explanation>
							<div key={i} {...getLineProps({ line })}>
								{!hideLineNumber && (
									<span className="inline-block w-6 text-right text-slate-500 dark:text-slate-400 pr-4 text-xs">
										{i + 1}
									</span>
								)}
								{line.map((token) => (
									<span
										key={token.content}
										{...getTokenProps({
											token,
										})}
									/>
								))}
							</div>
						))}
					</pre>
				)}
			</Highlight>
		</CodeWrap>
	);
}

export default function CodeWrap({ children, className }: Props): JSX.Element {
	return (
		<div
			className={clsx(
				"relative overflow-hidden shadow-xl flex sm:max-h-[none] sm:rounded-xl lg:h-[34.6875rem] xl:h-[31.625rem] bg-zinc-900/50 backdrop-blur ring-1 ring-inset ring-white/10 !h-auto max-h-[none] mb-4 text-base",
				className,
			)}
		>
			<div className="relative w-full flex flex-col">
				<div className="flex-none border-b border-slate-500/30">
					<div className="flex items-center h-8 space-x-1.5 px-3">
						<span className="w-3 h-3 bg-red-600 rounded-full" />
						<span className="w-3 h-3 bg-orange-600 rounded-full" />
						<span className="w-3 h-3 bg-green-600 rounded-full" />
					</div>
				</div>
				<div className="relative min-h-0 flex-auto flex flex-col [&_>_div]:mb-0">
					{children}
				</div>
			</div>
		</div>
	);
}
