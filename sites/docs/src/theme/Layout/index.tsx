import clsx from "clsx";
import ErrorBoundary from "@docusaurus/ErrorBoundary";
import {
	PageMetadata,
	SkipToContentFallbackId,
	ThemeClassNames,
} from "@docusaurus/theme-common";
import { useKeyboardNavigation } from "@docusaurus/theme-common/internal";
import SkipToContent from "@theme/SkipToContent";
import AnnouncementBar from "@theme/AnnouncementBar";
import Navbar from "@theme/Navbar";
// import Footer from "@theme/Footer";
import LayoutProvider from "@theme/Layout/Provider";
import ErrorPageContent from "@theme/ErrorPageContent";
import type { Props } from "@theme/Layout";
import styles from "./styles.module.css";
import { SectionProvider } from "@site/src/components/SectionProvider";
import { Footer } from "@site/src/components/Footer";
import { Toaster } from "react-hot-toast";

export default function Layout(props: Props): JSX.Element {
	const {
		children,
		noFooter,
		wrapperClassName,
		// Not really layout-related, but kept for convenience/retro-compatibility
		title,
		description,
	} = props;

	useKeyboardNavigation();

	return (
		<LayoutProvider>
			<SectionProvider sections={[]}>
				<PageMetadata title={title} description={description} />

				<SkipToContent />

				<AnnouncementBar />

				<Navbar />

				<div
					id={SkipToContentFallbackId}
					className={clsx(
						ThemeClassNames.wrapper.main,
						styles.mainWrapper,
						wrapperClassName,
					)}
				>
					<ErrorBoundary
						fallback={(params) => <ErrorPageContent {...params} />}
					>
						{children}
					</ErrorBoundary>
				</div>

				{!noFooter && <Footer />}
			</SectionProvider>
			<Toaster
				position="bottom-center"
				toastOptions={{
					className: "text-xs",
					style: {
						background: "rgb(251 146 60 / var(--tw-text-opacity, 1))",
						color: "white",
					},
					duration: 3000,
				}}
			/>
		</LayoutProvider>
	);
}
