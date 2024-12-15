"use client";

import Link from "@docusaurus/Link";
import {
	type MotionValue,
	motion,
	useMotionTemplate,
	useMotionValue,
} from "framer-motion";
import {
	Code,
	Server,
	Zap,
	Shield,
	Lock,
	FormInput,
	Webhook,
	AudioLines,
} from "lucide-react";

import { GridPattern } from "@site/src/components/GridPattern";
import { Heading } from "@site/src/components/Heading";
import { Button } from "./Button";

interface Feature {
	href: string;
	name: string;
	description: string;
	icon: React.ComponentType<{ className?: string }>;
	pattern: Omit<
		React.ComponentPropsWithoutRef<typeof GridPattern>,
		"width" | "height" | "x"
	>;
}

const features: Array<Feature> = [
	{
		href: "/",
		name: "High Performance",
		description:
			"Ngyn is built for speed, leveraging Rust's blazing-fast runtime and zero-cost abstractions to deliver unparalleled backend performance.",
		icon: Zap,
		pattern: {
			y: -6,
			squares: [
				[-1, 2],
				[1, 3],
			],
		},
	},
	{
		href: "/",
		name: "Type Safe",
		description:
			"Benefit from Rust's robust type system and compile-time guarantees, ensuring fewer runtime errors and more reliable backend services.",
		icon: Shield,
		pattern: {
			y: -6,
			squares: [
				[0, 2],
				[1, 4],
			],
		},
	},
	{
		href: "/",
		name: "Scalable Architecture",
		description:
			"Designed for microservices and large-scale applications, with built-in support for async programming and concurrent request handling.",
		icon: Server,
		pattern: {
			y: -6,
			squares: [
				[-1, 2],
				[1, 3],
			],
		},
	},
	{
		href: "/",
		name: "Developer Ergonomic",
		description:
			"Clean, expressive APIs and comprehensive documentation make building complex backend services simpler and more enjoyable.",
		icon: Code,
		pattern: {
			y: 22,
			squares: [[0, 1]],
		},
	},
	{
		href: "/",
		name: "Gates and Middlewares",
		description:
			"Learn about the contact model and how to create, retrieve, update, delete, and list contacts.",
		icon: Lock,
		pattern: {
			y: 16,
			squares: [
				[0, 1],
				[1, 3],
			],
		},
	},
	{
		href: "/",
		name: "Websockets and Streams",
		description:
			"Learn about the conversation model and how to create, retrieve, update, delete, and list conversations.",
		icon: Webhook,
		pattern: {
			y: -6,
			squares: [
				[-1, 2],
				[1, 3],
			],
		},
	},
	{
		href: "/",
		name: "Form Handling",
		description:
			"Learn about the conversation model and how to create, retrieve, update, delete, and list conversations.",
		icon: FormInput,
		pattern: {
			y: -6,
			squares: [
				[-1, 2],
				[1, 3],
			],
		},
	},
	{
		href: "/",
		name: "Multiple Platforms",
		description:
			"Learn about the message model and how to create, retrieve, update, delete, and list messages.",
		icon: AudioLines,
		pattern: {
			y: 32,
			squares: [
				[0, 2],
				[1, 4],
			],
		},
	},
];

function FeatureIcon({ icon: Icon }: { icon: Feature["icon"] }) {
	return (
		<div className="flex h-7 w-7 items-center justify-center rounded-full light:bg-zinc-900/5 ring-1 light:ring-zinc-900/25 backdrop-blur-[2px] transition duration-300 light:group-hover:bg-white/50 light:group-hover:ring-zinc-900/25 bg-white/7.5 ring-white/15 group-hover:bg-orange-300/10 group-hover:ring-orange-500">
			<Icon className="h-5 w-5 light:fill-zinc-700/10 light:stroke-zinc-700 transition-colors duration-300 light:group-hover:stroke-zinc-900 fill-white/10 stroke-zinc-400" />
		</div>
	);
}

function FeaturePattern({
	mouseX,
	mouseY,
	...gridProps
}: Feature["pattern"] & {
	mouseX: MotionValue<number>;
	mouseY: MotionValue<number>;
}) {
	const maskImage = useMotionTemplate`radial-gradient(180px at ${mouseX}px ${mouseY}px, white, transparent)`;
	const style = { maskImage, WebkitMaskImage: maskImage };

	return (
		<div className="pointer-events-none">
			<div className="absolute inset-0 rounded-2xl transition duration-300 [mask-image:linear-gradient(white,transparent)] group-hover:opacity-50">
				<GridPattern
					width={72}
					height={56}
					x="50%"
					className="absolute inset-x-0 inset-y-[-30%] h-[160%] w-full skew-y-[-18deg] fill-black/[0.02] stroke-black/5 dark:fill-white/1 dark:stroke-white/2.5 rounded-full"
					{...gridProps}
				/>
			</div>
			<motion.div
				className="absolute inset-0 rounded-2xl bg-gradient-to-r from-[#D7EDEA] to-[#F4FBDF] opacity-0 transition duration-300 group-hover:opacity-100 dark:from-[#202D2E] dark:to-[#303428]"
				style={style}
			/>
			<motion.div
				className="absolute inset-0 rounded-2xl opacity-0 mix-blend-overlay transition duration-300 group-hover:opacity-100"
				style={style}
			>
				<GridPattern
					width={72}
					height={56}
					x="50%"
					className="absolute inset-x-0 inset-y-[-30%] h-[160%] w-full skew-y-[-18deg] fill-black/50 stroke-black/70 dark:fill-white/2.5 dark:stroke-white/10 rounded-full"
					{...gridProps}
				/>
			</motion.div>
		</div>
	);
}

function Feature({ feature }: { feature: Feature }) {
	const mouseX = useMotionValue(0);
	const mouseY = useMotionValue(0);

	function onMouseMove({
		currentTarget,
		clientX,
		clientY,
	}: React.MouseEvent<HTMLDivElement>) {
		const { left, top } = currentTarget.getBoundingClientRect();
		mouseX.set(clientX - left);
		mouseY.set(clientY - top);
	}

	return (
		<div
			key={feature.href}
			onMouseMove={onMouseMove}
			className="group relative flex rounded-2xl light:bg-zinc-50 transition-shadow hover:shadow-md light:hover:shadow-zinc-900/5 bg-white/2.5 hover:shadow-black/5"
		>
			<FeaturePattern {...feature.pattern} mouseX={mouseX} mouseY={mouseY} />
			<div className="absolute inset-0 rounded-2xl ring-1 ring-inset ring-zinc-900/7.5 group-hover:ring-zinc-900/10 dark:ring-white/10 dark:group-hover:ring-white/20" />
			<div className="relative rounded-2xl px-4 pb-4 pt-16">
				<FeatureIcon icon={feature.icon} />
				<h3 className="mt-4 text-sm font-semibold leading-7 text-zinc-900 dark:text-white">
					<Link href={feature.href}>
						<span className="absolute inset-0 rounded-2xl" />
						{feature.name}
					</Link>
				</h3>
				<p className="mt-1 text-sm light:text-zinc-600 dark:text-zinc-400">
					{feature.description}
				</p>
				<Button variant="filled">Learn more</Button>
			</div>
		</div>
	);
}

export function EcosystemFeatures() {
	return (
		<div className="my-16 xl:max-w-none">
			<Heading level={2} id="ecosystem">
				The Ecosystem
			</Heading>
			<div className="not-prose mt-4 grid grid-cols-1 gap-8 border-t light:border-zinc-900/5 pt-10 sm:grid-cols-2 xl:grid-cols-4 dark:border-white/5">
				{features.map((feature) => (
					<Feature key={feature.name} feature={feature} />
				))}
			</div>
		</div>
	);
}
