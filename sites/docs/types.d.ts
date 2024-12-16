import type { SearchOptions } from "flexsearch";

declare module "@site/src/mdx/search.mjs" {
	export type Result = {
		url: string;
		title: string;
		pageTitle?: string;
	};

	export function search(query: string, options?: SearchOptions): Array<Result>;
}
