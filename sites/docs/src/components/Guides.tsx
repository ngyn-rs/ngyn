import { Button } from "@site/src/components/Button";
import { Heading } from "@site/src/components/Heading";

const guides = [
	{
		href: "/basics",
		name: "Basics",
		description:
			"Learn how to write superpowered sync and async routes and handlers.",
	},
	{
		href: "/gates",
		name: "Authentication & Authorization",
		description:
			"Learn how to restrict access to your endpoints based on specific requirements.",
	},
	{
		href: "/context",
		name: "Context API",
		description:
			"Understand how to work with ngyn at it's core with the context API.",
	},
	{
		href: "/errors",
		name: "Error Handling",
		description: "Empower your apps with a fluid error handling mechanism.",
	},
];

export function Guides() {
	return (
		<div className="my-16 xl:max-w-none">
			<Heading level={2} id="guides">
				Quickstart Guides
			</Heading>
			<div className="not-prose mt-4 grid grid-cols-1 gap-8 border-t border-zinc-900/5 pt-10 sm:grid-cols-2 xl:grid-cols-4 dark:border-white/5">
				{guides.map((guide) => (
					<div key={guide.href}>
						<h3 className="text-sm font-semibold light:text-zinc-900 dark:text-white">
							{guide.name}
						</h3>
						<p className="mt-1 text-sm light:text-zinc-600 dark:text-zinc-400">
							{guide.description}
						</p>
						<p className="mt-4">
							<Button href={`/docs/${guide.href}`} variant="text" arrow="right">
								Read more
							</Button>
						</p>
					</div>
				))}
			</div>
		</div>
	);
}
