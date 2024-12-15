import useDocusaurusContext from "@docusaurus/useDocusaurusContext";
import Layout from "@theme/Layout";
import { CodeBlock } from "../components/CodeWrap";
import NavbarLogo from "@theme/Logo";
import { Guides } from "@site/src/components/Guides";
import { EcosystemFeatures } from "../components/Features";
import Link from "@docusaurus/Link";

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
			<div className="relative isolate px-6 pt-2 lg:px-8">
				<div
					aria-hidden="true"
					className="absolute inset-x-0 -top-40 -z-10 transform-gpu overflow-hidden blur-3xl sm:-top-80"
				>
					<div
						style={{
							clipPath:
								"polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)",
						}}
						className="relative left-[calc(50%-11rem)] aspect-[1155/678] w-[36.125rem] -translate-x-1/2 rotate-[30deg] bg-gradient-to-tr from-[#ff80b5] to-[#9089fc] opacity-30 sm:left-[calc(50%-30rem)] sm:w-[72.1875rem]"
					/>
				</div>
				<div className="mx-auto max-w-2xl py-32 sm:py-24 lg:py-32">
					<div className="hidden sm:mb-8 sm:flex sm:justify-center">
						<div className="relative rounded-full px-3 py-1 text-sm/6 md:text-base/6 text-neutral-300 bg-neutral-400/10 hover:bg-gray-400/20">
							Latest version release:
							<Link href="/docs" className="font-semibold text-orange-400 pl-2">
								v0.5.x <span aria-hidden="true">&rarr;</span>
							</Link>
						</div>
					</div>
					<div className="text-center">
						<h1 className="text-balance text-5xl font-semibold tracking-tight text-orange-600 sm:text-7xl">
							<NavbarLogo titleClassName="hidden" />
						</h1>
						<p className="mt-8 text-pretty text-lg md:text-2xl font-medium text-gray-500 dark:text-neutral-50 sm:text-xl/8">
							A next-generation web framework for building lightning-fast,
							reliable backend services in Rust
						</p>
						<div className="mt-10 flex items-center justify-center gap-x-6">
							<a
								href="/"
								className="rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
							>
								Get started
							</a>
							<a href="/" className="text-sm/6 font-semibold text-orange-900">
								Github <span aria-hidden="true">→</span>
							</a>
						</div>
					</div>
				</div>
				<div
					aria-hidden="true"
					className="absolute inset-x-0 top-[calc(100%-13rem)] -z-10 transform-gpu overflow-hidden blur-3xl sm:top-[calc(100%-30rem)]"
				>
					<div
						style={{
							clipPath:
								"polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)",
						}}
						className="relative left-[calc(50%+3rem)] aspect-[1155/678] w-[36.125rem] -translate-x-1/2 bg-gradient-to-tr from-[#ff80b5] to-[#9089fc] opacity-30 sm:left-[calc(50%+36rem)] sm:w-[72.1875rem]"
					/>
				</div>
			</div>
			<div className="container">
				<Guides />
				<EcosystemFeatures />
			</div>
		</Layout>
	);
}
