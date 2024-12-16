import useDocusaurusContext from "@docusaurus/useDocusaurusContext";
import Layout from "@theme/Layout";
import { CodeBlock } from "../components/CodeWrap";
import NavbarLogo from "@theme/Logo";
import { Guides } from "@site/src/components/Guides";
import { EcosystemFeatures } from "../components/Features";
import Link from "@docusaurus/Link";
import { Button } from "../components/Button";
import toast from "react-hot-toast";
import { useEvent } from "react-use";

const homePageCode = `use ngyn::prelude::*;

#[handler]
fn echo_hello() -> String {
    "Hello World, Ngyn!".to_string()
}

#[tokio::main]
async fn main() {

    let mut app = HyperApplication::default();

    // echo hello for every route and http post verb
    app.any("*", echo_hello);

    let _ = app.listen("127.0.0.1:3000").await;
}`;

export default function Home(): JSX.Element {
	const { siteConfig } = useDocusaurusContext();

	async function onKeyDown(event: KeyboardEvent) {
		if (event.key === "c" && (event.metaKey || event.ctrlKey)) {
			event.preventDefault();
			await navigator.clipboard.writeText("cargo add ngyn");
			toast.success("Copied to clipboard successfully");
		}
	}

	useEvent("onKeyDown", onKeyDown);

	return (
		<Layout
			title={siteConfig.title}
			description="Craft Scalable Web Applications with Ease in Rust!"
		>
			<div className="relative isolate px-6 pt-2 lg:px-8 container">
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
				<div className="flex flex-col md:flex-row justify-between items-center">
					<div className="py-32 sm:py-24 lg:py-32">
						<div className="hidden sm:mb-8 sm:flex">
							<div className="relative rounded-full px-3 py-1 text-sm/6 md:text-base/6 text-neutral-300 bg-neutral-400/10 hover:bg-gray-400/20 flex items-center gap-2">
								<NavbarLogo
									titleClassName="hidden"
									imageClassName="w-5"
									className="leading-[0]"
								/>
								Latest version release:
								<Link
									href="/docs"
									className="font-semibold text-orange-400 pl-2"
								>
									v0.5.x <span aria-hidden="true">&rarr;</span>
								</Link>
							</div>
						</div>
						<div className="md:max-w-[60%] 2xl:max-w-[50%] text-center md:text-left">
							<h1 className="text-balance text-5xl font-semibold tracking-tight text-neutral-300 sm:text-7xl">
								ngyn{" "}
								<code className="text-2xl rounded-full font-medium">
									(en·jn)
								</code>
							</h1>
							<p className="mt-8 text-pretty text-lg md:text-xl font-medium text-gray-500 dark:text-neutral-50 sm:text-xl/8">
								The next-generation web framework for building ergonomic,
								lightning-fast, type-safe, reliable backend services in Rust.
							</p>
							<div className="mt-10 flex justify-center md:justify-start items-center gap-6 flex-wrap">
								<Button href="/docs" variant="filled">
									Get started
								</Button>
								<Button href="https://docs.rs/ngyn" variant="primary">
									API Reference <span aria-hidden="true">→</span>
								</Button>
								<Button
									variant="secondary"
									onClick={async () => {
										await navigator.clipboard.writeText("cargo add ngyn");
										toast.success("Copied to clipboard successfully");
									}}
								>
									cargo add ngyn
									<kbd className="pl-1 flex-none text-xs font-semibold font-sans">
										⌘C
									</kbd>
								</Button>
							</div>
						</div>
					</div>
					<div className="max-w-[98%]">
						<CodeBlock className="mt-8" code={homePageCode} hideLineNumber />
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
