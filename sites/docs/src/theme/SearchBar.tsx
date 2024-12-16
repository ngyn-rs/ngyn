// By default, the classic theme does not provide any SearchBar implementation
// If you swizzled this, it is your responsibility to provide an implementation
// Tip: swizzle the SearchBar from the Algolia theme for inspiration:
// npm run swizzle @docusaurus/theme-search-algolia SearchBar

import { useHistory, useLocation } from "@docusaurus/router";
import { Dialog, DialogBackdrop, DialogPanel } from "@headlessui/react";
import clsx from "clsx";
import Highlighter from "react-highlight-words";
import {
	useState,
	useEffect,
	Suspense,
	useRef,
	useId,
	Fragment,
	useCallback,
} from "react";
import useDocusaurusContext from "@docusaurus/useDocusaurusContext";

import {
	type AutocompleteApi,
	type AutocompleteCollection,
	type AutocompleteState,
	createAutocomplete,
} from "@algolia/autocomplete-core";
import type { Result } from "@site/src/mdx/search.mjs";

type EmptyObject = Record<string, never>;

type Autocomplete = AutocompleteApi<
	Result,
	React.SyntheticEvent,
	React.MouseEvent,
	React.KeyboardEvent
>;

function useAutocomplete({ close }: { close: () => void }) {
	const id = useId();
	const router = useHistory();
	const [autocompleteState, setAutocompleteState] = useState<
		AutocompleteState<Result> | EmptyObject
	>({});

	const {
		siteConfig: { customFields },
	} = useDocusaurusContext();

	function navigate({ itemUrl }: { itemUrl?: string }) {
		if (!itemUrl) {
			return;
		}

		router.push(itemUrl);

		if (
			itemUrl ===
			window.location.pathname + window.location.search + window.location.hash
		) {
			close();
		}
	}

	const [autocomplete] = useState<Autocomplete>(() =>
		createAutocomplete<
			Result,
			React.SyntheticEvent,
			React.MouseEvent,
			React.KeyboardEvent
		>({
			id,
			placeholder: "Search documentation...",
			defaultActiveItemId: 0,
			onStateChange({ state }) {
				setAutocompleteState(state);
			},
			shouldPanelOpen({ state }) {
				return state.query !== "";
			},
			navigator: {
				navigate,
			},
			getSources({ query }) {
				return new Promise((resolve) =>
					resolve({
						search: async (query: string, { limit }: { limit?: number }) => {
							const response = await fetch(
								`${customFields.chipSearchUrl}/search?query=${query}&limit=${limit}`,
							);
							return response
								.json()
								.then(({ data }) => data ?? [])
								.catch(() => []);
						},
					}),
				).then(({ search }) => {
					return [
						{
							sourceId: "documentation",
							getItems() {
								return search(query, { limit: 5 });
							},
							getItemUrl({ item }) {
								return item.url;
							},
							onSelect: navigate,
						},
					];
				});
			},
		}),
	);

	return { autocomplete, autocompleteState };
}

function useSearchProps() {
	const buttonRef = useRef<React.ElementRef<"button">>(null);
	const [open, setOpen] = useState(false);

	return {
		buttonProps: {
			ref: buttonRef,
			onClick() {
				setOpen(true);
			},
		},
		dialogProps: {
			open,
			setOpen: useCallback((open: boolean) => {
				const { width = 0, height = 0 } =
					buttonRef.current?.getBoundingClientRect() ?? {};
				if (!open || (width !== 0 && height !== 0)) {
					setOpen(open);
				}
			}, []),
		},
	};
}

function SearchIcon(props: React.ComponentPropsWithoutRef<"svg">) {
	return (
		<svg viewBox="0 0 20 20" fill="none" aria-hidden="true" {...props}>
			<path
				strokeLinecap="round"
				strokeLinejoin="round"
				d="M12.01 12a4.25 4.25 0 1 0-6.02-6 4.25 4.25 0 0 0 6.02 6Zm0 0 3.24 3.25"
			/>
		</svg>
	);
}

function NoResultsIcon(props: React.ComponentPropsWithoutRef<"svg">) {
	return (
		<svg viewBox="0 0 20 20" fill="none" aria-hidden="true" {...props}>
			<path
				strokeLinecap="round"
				strokeLinejoin="round"
				d="M12.01 12a4.237 4.237 0 0 0 1.24-3c0-.62-.132-1.207-.37-1.738M12.01 12A4.237 4.237 0 0 1 9 13.25c-.635 0-1.237-.14-1.777-.388M12.01 12l3.24 3.25m-3.715-9.661a4.25 4.25 0 0 0-5.975 5.908M4.5 15.5l11-11"
			/>
		</svg>
	);
}

function LoadingIcon(props: React.ComponentPropsWithoutRef<"svg">) {
	const id = useId();

	return (
		<svg viewBox="0 0 20 20" fill="none" aria-hidden="true" {...props}>
			<circle cx="10" cy="10" r="5.5" strokeLinejoin="round" />
			<path
				stroke={`url(#${id})`}
				strokeLinecap="round"
				strokeLinejoin="round"
				d="M15.5 10a5.5 5.5 0 1 0-5.5 5.5"
			/>
			<defs>
				<linearGradient
					id={id}
					x1="13"
					x2="9.5"
					y1="9"
					y2="15"
					gradientUnits="userSpaceOnUse"
				>
					<stop stopColor="currentColor" />
					<stop offset="1" stopColor="currentColor" stopOpacity="0" />
				</linearGradient>
			</defs>
		</svg>
	);
}

function HighlightQuery({ text, query }: { text: string; query: string }) {
	return (
		<Highlighter
			highlightClassName="underline bg-transparent text-orange-500"
			searchWords={[query]}
			autoEscape={true}
			textToHighlight={text}
		/>
	);
}

function SearchResult({
	result,
	resultIndex,
	autocomplete,
	collection,
	query,
}: {
	result: Result;
	resultIndex: number;
	autocomplete: Autocomplete;
	collection: AutocompleteCollection<Result>;
	query: string;
}) {
	const id = useId();

	const sectionTitle = [].find((section) =>
		section.links.find((link) => link.href === result.url.split("#")[0]),
	)?.title;
	const hierarchy = [sectionTitle, result.pageTitle].filter(
		(x): x is string => typeof x === "string",
	);

	return (
		<li
			className={clsx(
				"group block cursor-default px-4 py-3 light:aria-selected:bg-zinc-50 aria-selected:bg-zinc-800/50",
				resultIndex > 0 && "border-t light:border-zinc-100 border-zinc-800",
			)}
			aria-labelledby={`${id}-hierarchy ${id}-title`}
			{...autocomplete.getItemProps({
				item: result,
				source: collection.source,
			})}
		>
			<div
				id={`${id}-title`}
				aria-hidden="true"
				className="text-sm font-medium light:text-zinc-900 group-aria-selected:text-orange-500 text-white"
			>
				<HighlightQuery text={result.title} query={query} />
			</div>
			{hierarchy.length > 0 && (
				<div
					id={`${id}-hierarchy`}
					aria-hidden="true"
					className="mt-1 truncate whitespace-nowrap text-2xs text-zinc-500"
				>
					{hierarchy.map((item, itemIndex, items) => (
						// biome-ignore lint/suspicious/noArrayIndexKey: <explanation>
						<Fragment key={itemIndex}>
							<HighlightQuery text={item} query={query} />
							<span
								className={
									itemIndex === items.length - 1
										? "sr-only"
										: "mx-2 light:text-zinc-300 text-zinc-700"
								}
							>
								/
							</span>
						</Fragment>
					))}
				</div>
			)}
		</li>
	);
}

function SearchResults({
	autocomplete,
	query,
	collection,
}: {
	autocomplete: Autocomplete;
	query: string;
	collection: AutocompleteCollection<Result>;
}) {
	if (collection.items.length === 0) {
		return (
			<div className="p-6 text-center">
				<NoResultsIcon className="mx-auto h-5 w-5 light:stroke-zinc-900 stroke-zinc-600" />
				<p className="mt-2 text-xs light:text-zinc-700 text-zinc-400">
					Nothing found for{" "}
					<strong className="break-words font-semibold light:text-zinc-900 text-white">
						&lsquo;{query}&rsquo;
					</strong>
					. Please try again.
				</p>
			</div>
		);
	}

	return (
		<ul {...autocomplete.getListProps()}>
			{collection.items.map((result, resultIndex) => (
				<SearchResult
					key={result.url}
					result={result}
					resultIndex={resultIndex}
					autocomplete={autocomplete}
					collection={collection}
					query={query}
				/>
			))}
		</ul>
	);
}

function SearchInput({
	ref,
	autocomplete,
	autocompleteState,
	onClose,
}: React.ComponentPropsWithRef<"input"> & {
	autocomplete: Autocomplete;
	autocompleteState: AutocompleteState<Result> | EmptyObject;
	onClose: () => void;
}) {
	const inputProps = autocomplete.getInputProps({ inputElement: null });

	return (
		<div className="group relative flex h-12">
			<SearchIcon className="pointer-events-none absolute left-3 top-0 h-full w-5 stroke-zinc-500" />
			<input
				ref={ref}
				data-autofocus
				className={clsx(
					"flex-auto appearance-none bg-transparent pl-10 light:text-zinc-900 outline-none placeholder:text-zinc-500 focus:w-full focus:flex-none sm:text-sm text-white [&::-webkit-search-cancel-button]:hidden [&::-webkit-search-decoration]:hidden [&::-webkit-search-results-button]:hidden [&::-webkit-search-results-decoration]:hidden",
					autocompleteState.status === "stalled" ? "pr-11" : "pr-4",
				)}
				{...inputProps}
				onKeyDown={(event) => {
					if (
						event.key === "Escape" &&
						!autocompleteState.isOpen &&
						autocompleteState.query === ""
					) {
						// In Safari, closing the dialog with the escape key can sometimes cause the scroll position to jump to the
						// bottom of the page. This is a workaround for that until we can figure out a proper fix in Headless UI.
						if (document.activeElement instanceof HTMLElement) {
							document.activeElement.blur();
						}

						onClose();
					} else {
						inputProps.onKeyDown(event);
					}
				}}
			/>
			{autocompleteState.status === "stalled" && (
				<div className="absolute inset-y-0 right-3 flex items-center">
					<LoadingIcon className="h-5 w-5 animate-spin light:stroke-zinc-200 light:text-zinc-900 stroke-zinc-800 text-orange-400" />
				</div>
			)}
		</div>
	);
}

function SearchDialog({
	open,
	setOpen,
	className,
}: {
	open: boolean;
	setOpen: (open: boolean) => void;
	className?: string;
}) {
	const formRef = useRef<React.ElementRef<"form">>(null);
	const panelRef = useRef<React.ElementRef<"div">>(null);
	const inputRef = useRef<React.ElementRef<typeof SearchInput>>(null);
	const { autocomplete, autocompleteState } = useAutocomplete({
		close() {
			setOpen(false);
		},
	});
	const { pathname, search } = useLocation();

	// biome-ignore lint/correctness/useExhaustiveDependencies: <explanation>
	useEffect(() => {
		setOpen(false);
	}, [pathname, search, setOpen]);

	useEffect(() => {
		if (open) {
			return;
		}

		function onKeyDown(event: KeyboardEvent) {
			if (event.key === "k" && (event.metaKey || event.ctrlKey)) {
				event.preventDefault();
				setOpen(true);
			}
		}

		window.addEventListener("keydown", onKeyDown);

		return () => {
			window.removeEventListener("keydown", onKeyDown);
		};
	}, [open, setOpen]);

	return (
		<Dialog
			open={open}
			onClose={() => {
				setOpen(false);
				autocomplete.setQuery("");
			}}
			className={clsx("fixed inset-0 z-50", className)}
		>
			<DialogBackdrop
				transition
				className="fixed inset-0 light:bg-zinc-400/25 backdrop-blur-sm data-[closed]:opacity-0 data-[enter]:duration-300 data-[leave]:duration-200 data-[enter]:ease-out data-[leave]:ease-in bg-black/40"
			/>

			<div className="fixed inset-0 overflow-y-auto px-4 py-4 sm:px-6 sm:py-20 md:py-32 lg:px-8 lg:py-[15vh]">
				<DialogPanel
					transition
					className="mx-auto transform-gpu overflow-hidden rounded-lg light:bg-zinc-50 shadow-xl ring-1 ring-zinc-900/7.5 data-[closed]:scale-95 data-[closed]:opacity-0 data-[enter]:duration-300 data-[leave]:duration-200 data-[enter]:ease-out data-[leave]:ease-in sm:max-w-xl bg-zinc-900 ring-zinc-800"
				>
					<div {...autocomplete.getRootProps({})}>
						<form
							ref={formRef}
							{...autocomplete.getFormProps({
								inputElement: inputRef.current,
							})}
						>
							<SearchInput
								ref={inputRef}
								autocomplete={autocomplete}
								autocompleteState={autocompleteState}
								onClose={() => setOpen(false)}
							/>
							<div
								ref={panelRef}
								className="border-t light:border-zinc-200 light:bg-white empty:hidden border-zinc-100/5 bg-white/2.5"
								{...autocomplete.getPanelProps({})}
							>
								{autocompleteState.isOpen && (
									<SearchResults
										autocomplete={autocomplete}
										query={autocompleteState.query ?? ""}
										collection={autocompleteState.collections[0]}
									/>
								)}
							</div>
						</form>
					</div>
				</DialogPanel>
			</div>
		</Dialog>
	);
}

export default function Search() {
	const { buttonProps, dialogProps } = useSearchProps();

	return (
		<div className="hidden lg:block lg:max-w-md lg:flex-auto -ml-2">
			<button
				type="button"
				className="hidden h-8 w-full items-center gap-2 rounded-full bg-white pl-2 pr-3 text-sm light:text-zinc-500 ring-1 light:ring-zinc-900/10 transition light:hover:ring-zinc-900/20 ui-not-focus-visible:outline-none lg:flex bg-white/5 text-zinc-400 ring-inset ring-white/10 hover:ring-white/20"
				{...buttonProps}
			>
				<SearchIcon className="h-5 w-5 stroke-current" />
				Search documentation...
				<kbd className="ml-auto text-2xs light:text-zinc-400 text-neutral-500">
					<kbd className="font-sans text-neutral-500">
						{/(Mac|iPhone|iPod|iPad)/i.test(navigator.platform) ? "⌘" : "Ctrl "}
					</kbd>
					<kbd className="font-sans text-neutral-500">K</kbd>
				</kbd>
			</button>
			<Suspense fallback={null}>
				<SearchDialog className="hidden lg:block" {...dialogProps} />
			</Suspense>
		</div>
	);
}
