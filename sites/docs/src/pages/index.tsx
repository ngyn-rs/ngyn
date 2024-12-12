import useDocusaurusContext from "@docusaurus/useDocusaurusContext";
import Layout from "@theme/Layout";
import { CodeBlock } from "../components/CodeWrap";

const homePageCode = `#[handler]
fn echo_hello() -> String {
    "Hello World!".to_string()
}`;

function HomepageHeader() {
	return (
		<header className="flex justify-between">
			<div className="relative max-w-5xl mx-auto px-4 py-24 sm:py-40 lg:py-48 min-h-full">
				<h1 className="text-zinc-900 font-extrabold text-4xl sm:text-5xl lg:text-6xl tracking-tight text-center dark:text-white">
					Craft Scalable Web Applications with Ease in Rust!
				</h1>
				<p className="mt-6 text-lg text-slate-600 text-center max-w-3xl mx-auto dark:text-slate-400">
					Ngyn is where simplicity meets sophistication in Rust web development.
					It provides a set of {""}
					<span className="font-bold text-orange-500 dark:text-orange-400">
						macros
					</span>
					, {}
					<span className="font-bold text-orange-500 dark:text-orange-400">
						traits
					</span>
					, and {}
					<span className="font-medium text-orange-500 dark:text-orange-400">
						utilities
					</span>{" "}
					that help create efficient platform-agnostic web apps progressively.
				</p>
				<div className="mt-6 sm:mt-10 flex justify-center space-x-4 text-sm">
					<a
						className="btn btn-primary text-center py-3 w-full md:w-fit"
						href="/docs"
					>
						Get started
					</a>
					<button
						type="button"
						className="border-0 hidden lg:flex items-center text-sm leading-6 text-zinc-400 rounded-md ring-1 ring-zinc-900/10 shadow-sm py-1.5 px-3 hover:ring-zinc-300 dark:bg-zinc-800 dark:highlight-white/5 dark:hover:bg-zinc-700"
						onClick={() => navigator.clipboard.writeText("cargo add ngyn")}
					>
						<span className="text-neutral-200 font-bold">cargo add ngyn</span>
						<span className="ml-auto pl-3 flex-none text-xs font-semibold">
							⌘C
						</span>
					</button>
				</div>
			</div>
			<div>
				<CodeBlock className="mt-8" code={homePageCode} hideLineNumber />
			</div>
		</header>
	);
}

export default function Home(): JSX.Element {
	const { siteConfig } = useDocusaurusContext();
	return (
		<Layout
			title={siteConfig.title}
			description="Craft Scalable Web Applications with Ease in Rust!"
		>
			<HomepageHeader />
		</Layout>
	);
}
