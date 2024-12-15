import Link from "@docusaurus/Link";
import clsx from "clsx";

function ArrowIcon(props: React.ComponentPropsWithoutRef<"svg">) {
	return (
		<svg viewBox="0 0 20 20" fill="none" aria-hidden="true" {...props}>
			<path
				stroke="currentColor"
				strokeLinecap="round"
				strokeLinejoin="round"
				d="m11.5 6.5 3 3.5m0 0-3 3.5m3-3.5h-9"
			/>
		</svg>
	);
}

const variantStyles = {
	primary:
		"rounded-full light:bg-zinc-900 py-1 px-3 text-white light:hover:bg-zinc-700 bg-orange-400/10 text-orange-400 ring-1 ring-inset ring-orange-400/20 hover:bg-orange-400/10 hover:text-orange-300 hover:ring-orange-300",
	secondary:
		"rounded-full light:bg-zinc-100 py-1 px-3 light:text-zinc-900 light:hover:bg-zinc-200 bg-zinc-800/40 text-zinc-400 ring-1 ring-inset ring-zinc-800 hover:bg-zinc-800 hover:text-zinc-300",
	filled:
		"rounded-full light:bg-zinc-900 py-1 px-3 text-white light:hover:bg-zinc-700 bg-orange-500 hover:text-neutral-50 hover:bg-orange-400",
	outline:
		"rounded-full py-1 px-3 light:text-zinc-700 ring-1 ring-inset ring-zinc-900/10 hover:bg-zinc-900/2.5 hover:text-zinc-900 text-zinc-400 ring-white/10 hover:bg-white/5 hover:text-white",
	text: "text-orange-500 hover:text-orange-600 text-orange-400 hover:text-orange-500",
};

type ButtonProps = {
	variant?: keyof typeof variantStyles;
	arrow?: "left" | "right";
} & (
	| React.ComponentPropsWithoutRef<typeof Link>
	| (React.ComponentPropsWithoutRef<"button"> & { href?: undefined })
);

export function Button({
	variant = "primary",
	className,
	children,
	arrow,
	type: buttonType = "button",
	...props
}: ButtonProps) {
	className = clsx(
		"inline-flex gap-0.5 justify-center overflow-hidden text-sm font-medium transition border-0",
		variantStyles[variant],
		className,
	);

	const arrowIcon = (
		<ArrowIcon
			className={clsx(
				"mt-0.5 h-5 w-5",
				variant === "text" && "relative top-px",
				arrow === "left" && "-ml-1 rotate-180",
				arrow === "right" && "-mr-1",
			)}
		/>
	);

	const inner = (
		<>
			{arrow === "left" && arrowIcon}
			{children}
			{arrow === "right" && arrowIcon}
		</>
	);

	if (typeof props.href === "undefined") {
		return (
			<button
				type={buttonType as HTMLButtonElement["type"]}
				className={className}
				{...(props as React.ComponentPropsWithoutRef<"button">)}
			>
				{inner}
			</button>
		);
	}

	return (
		<Link
			className={className}
			{...(props as React.ComponentPropsWithoutRef<typeof Link>)}
		>
			{inner}
		</Link>
	);
}
