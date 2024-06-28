import clsx from "clsx";
import { Highlight, themes } from "prism-react-renderer";
import React from "react";
import { useColorMode } from "@docusaurus/theme-common";

type Props = {
    children: React.ReactNode;
    className?: string;
};

export function CodeBlock({
    code,
    language = "rust",
    className,
}: {
    code: string;
    language?: string;
    className?: string;
}): JSX.Element {
    const { colorMode } = useColorMode();
    return (
        <CodeWrap className={className}>
            <Highlight
                theme={colorMode === "dark" ? themes.oneDark : themes.oneLight}
                code={code}
                language={language}
            >
                {({
                    className,
                    style,
                    tokens,
                    getLineProps,
                    getTokenProps,
                }) => (
                    <pre
                        className={clsx(className, "block  flex-auto mb-0")}
                        style={style}
                    >
                        {tokens.map((line, i) => (
                            <div key={i} {...getLineProps({ line })}>
                                <span className="inline-block w-6 text-right text-slate-500 dark:text-slate-400 pr-4 text-xs">
                                    {i + 1}
                                </span>
                                {line.map((token, key) => (
                                    <span
                                        key={key}
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
                "relative overflow-hidden shadow-xl flex bg-white border border-black sm:max-h-[none] sm:rounded-xl lg:h-[34.6875rem] xl:h-[31.625rem] dark:bg-zinc-900/70 dark:backdrop-blur dark:ring-1 dark:ring-inset dark:ring-white/10 !h-auto max-h-[none] mb-4",
                className
            )}
        >
            <div className="relative w-full flex flex-col">
                <div className="flex-none border-b border-slate-500/30">
                    <div className="flex items-center h-8 space-x-1.5 px-3">
                        <div className="w-2.5 h-2.5 bg-red-600 rounded-full"></div>
                        <div className="w-2.5 h-2.5 bg-orange-600 rounded-full"></div>
                        <div className="w-2.5 h-2.5 bg-green-600 rounded-full"></div>
                    </div>
                </div>
                <div className="relative min-h-0 flex-auto flex flex-col [&_>_div]:mb-0">
                    {children}
                </div>
            </div>
        </div>
    );
}
