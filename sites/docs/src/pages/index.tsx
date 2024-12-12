import { useState } from "react";
import { Dialog, DialogPanel } from "@headlessui/react";
import { Bars3Icon, XMarkIcon } from "@heroicons/react/24/outline";
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

const navigation = [];

export default function Home(): JSX.Element {
	const { siteConfig } = useDocusaurusContext();
	const [mobileMenuOpen, setMobileMenuOpen] = useState(false);
	return (
		<Layout
			title={siteConfig.title}
			description="Craft Scalable Web Applications with Ease in Rust!"
		>
			<div className="bg-white">
				<header className="absolute inset-x-0 top-0 z-50">
					<nav
						aria-label="Global"
						className="flex items-center justify-between p-6 lg:px-8"
					>
						<div className="flex lg:flex-1">
							<a href="#" className="-m-1.5 p-1.5">
								<span className="sr-only">Your Company</span>
								<img
									alt=""
									src="https://tailwindui.com/plus/img/logos/mark.svg?color=indigo&shade=600"
									className="h-8 w-auto"
								/>
							</a>
						</div>
						<div className="flex lg:hidden">
							<button
								type="button"
								onClick={() => setMobileMenuOpen(true)}
								className="-m-2.5 inline-flex items-center justify-center rounded-md p-2.5 text-gray-700"
							>
								<span className="sr-only">Open main menu</span>
								<Bars3Icon aria-hidden="true" className="size-6" />
							</button>
						</div>
						<div className="hidden lg:flex lg:gap-x-12">
							{navigation.map((item) => (
								<a
									key={item.name}
									href={item.href}
									className="text-sm/6 font-semibold text-gray-900"
								>
									{item.name}
								</a>
							))}
						</div>
						<div className="hidden lg:flex lg:flex-1 lg:justify-end">
							<a href="#" className="text-sm/6 font-semibold text-gray-900">
								Log in <span aria-hidden="true">&rarr;</span>
							</a>
						</div>
					</nav>
					<Dialog
						open={mobileMenuOpen}
						onClose={setMobileMenuOpen}
						className="lg:hidden"
					>
						<div className="fixed inset-0 z-50" />
						<DialogPanel className="fixed inset-y-0 right-0 z-50 w-full overflow-y-auto bg-white px-6 py-6 sm:max-w-sm sm:ring-1 sm:ring-gray-900/10">
							<div className="flex items-center justify-between">
								<a href="#" className="-m-1.5 p-1.5">
									<span className="sr-only">Your Company</span>
									<img
										alt=""
										src="https://tailwindui.com/plus/img/logos/mark.svg?color=indigo&shade=600"
										className="h-8 w-auto"
									/>
								</a>
								<button
									type="button"
									onClick={() => setMobileMenuOpen(false)}
									className="-m-2.5 rounded-md p-2.5 text-gray-700"
								>
									<span className="sr-only">Close menu</span>
									<XMarkIcon aria-hidden="true" className="size-6" />
								</button>
							</div>
							<div className="mt-6 flow-root">
								<div className="-my-6 divide-y divide-gray-500/10">
									<div className="space-y-2 py-6">
										{navigation.map((item) => (
											<a
												key={item.name}
												href={item.href}
												className="-mx-3 block rounded-lg px-3 py-2 text-base/7 font-semibold text-gray-900 hover:bg-gray-50"
											>
												{item.name}
											</a>
										))}
									</div>
									<div className="py-6">
										<a
											href="#"
											className="-mx-3 block rounded-lg px-3 py-2.5 text-base/7 font-semibold text-gray-900 hover:bg-gray-50"
										>
											Log in
										</a>
									</div>
								</div>
							</div>
						</DialogPanel>
					</Dialog>
				</header>

				<div className="relative isolate px-6 pt-14 lg:px-8">
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
					<div className="mx-auto max-w-2xl py-32 sm:py-48 lg:py-56">
						<div className="hidden sm:mb-8 sm:flex sm:justify-center">
							<div className="relative rounded-full px-3 py-1 text-sm/6 text-gray-600 ring-1 ring-gray-900/10 hover:ring-gray-900/20">
								Announcing our next round of funding.{" "}
								<a href="#" className="font-semibold text-indigo-600">
									<span aria-hidden="true" className="absolute inset-0" />
									Read more <span aria-hidden="true">&rarr;</span>
								</a>
							</div>
						</div>
						<div className="text-center">
							<h1 className="text-balance text-5xl font-semibold tracking-tight text-gray-900 sm:text-7xl">
								Data to enrich your online business
							</h1>
							<p className="mt-8 text-pretty text-lg font-medium text-gray-500 sm:text-xl/8">
								Anim aute id magna aliqua ad ad non deserunt sunt. Qui irure qui
								lorem cupidatat commodo. Elit sunt amet fugiat veniam occaecat.
							</p>
							<div className="mt-10 flex items-center justify-center gap-x-6">
								<a
									href="#"
									className="rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
								>
									Get started
								</a>
								<a href="#" className="text-sm/6 font-semibold text-gray-900">
									Learn more <span aria-hidden="true">→</span>
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
			</div>
		</Layout>
	);
}
