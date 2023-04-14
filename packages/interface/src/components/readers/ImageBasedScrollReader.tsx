import type { Media } from '@stump/types'
import { defaultRangeExtractor, Range, useVirtualizer } from '@tanstack/react-virtual'
import { useCallback, useEffect, useRef, useState } from 'react'
import { useWindowSize } from 'rooks'

type Props = {
	media: Media
	initialPage?: number
	orientation?: 'horizontal' | 'vertical'
	getPageUrl(page: number): string
	onProgressUpdate(page: number): void
}

// TODO: support both horizontal and vertical scrolling
export default function ImageBasedScrollReader({
	initialPage,
	media,
	// orientation = 'vertical',
	getPageUrl,
	onProgressUpdate,
}: Props) {
	const parentRef = useRef<HTMLDivElement>(null)
	const [imageSizes, setImageSizes] = useState<Record<number, number>>({})

	const { innerWidth, innerHeight } = useWindowSize()

	const estimateSize = useCallback(
		(index: number) => {
			const loadedSize = imageSizes[index]
			return loadedSize ?? (innerHeight || 35)
		},
		[imageSizes, innerHeight],
	)

	const visibleRef = useRef([0, 0])
	const virtualizer = useVirtualizer({
		count: media.pages,
		estimateSize,
		getScrollElement: () => parentRef.current,
		overscan: 5,
		rangeExtractor: useCallback((range: Range) => {
			visibleRef.current = [range.startIndex, range.endIndex]
			return defaultRangeExtractor(range)
		}, []),
	})

	useEffect(
		() => {
			if (initialPage) {
				virtualizer.scrollToIndex(initialPage - 1)
			}
		},
		// eslint-disable-next-line react-hooks/exhaustive-deps
		[initialPage],
	)

	useEffect(
		() => {
			virtualizer.measure()
		},
		// eslint-disable-next-line react-hooks/exhaustive-deps
		[innerWidth, innerHeight, imageSizes],
	)

	//* I take the lower bound as the current page so we don't eagarly update the progress
	//* right when the edge of the next page is visible. It isn't perfect, but it's better.
	const currentPage = visibleRef.current[0] ?? 0
	useEffect(
		() => {
			//! FIXME: important!: debounce this API call...
			onProgressUpdate(currentPage + 1)
		},
		// eslint-disable-next-line react-hooks/exhaustive-deps
		[currentPage],
	)

	// FIXME: does not handle resize well....
	return (
		<div
			ref={parentRef}
			className="w-full overflow-y-scroll"
			style={{
				height: innerHeight || undefined,
			}}
		>
			<div
				className="relative inline-flex h-full w-full text-white"
				style={{
					height: `${virtualizer.getTotalSize()}px`,
				}}
			>
				<div className="fixed bottom-2 left-2 z-50 rounded-lg bg-black bg-opacity-75 px-2 py-1 text-white">
					{(visibleRef.current[0] ?? 0) + 1}
				</div>
				{virtualizer.getVirtualItems().map((virtualRow) => {
					const virtualPage = virtualRow.index + 1
					const imageUrl = getPageUrl(virtualPage)

					return (
						<div
							key={virtualRow.key}
							data-index={virtualRow.index}
							style={{
								// height: `${estimateSize(virtualRow.index)}px`,
								position: 'absolute',
								transform: `translateY(${virtualRow.start}px)`,
								width: '100%',
							}}
							className="flex items-start justify-center"
							// ref={virtualizer.measureElement}
						>
							<img
								className="max-h-full w-full select-none object-scale-down md:w-auto"
								src={imageUrl}
								style={{
									maxHeight: innerHeight || undefined,
								}}
								onLoad={(e) => {
									const height = e.currentTarget.height
									setImageSizes((prev) => ({
										...prev,
										[virtualRow.index]: height,
									}))
								}}
							/>
						</div>
					)
				})}
			</div>
		</div>
	)
}
