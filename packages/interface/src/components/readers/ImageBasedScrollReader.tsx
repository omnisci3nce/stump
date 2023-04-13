import type { Media } from '@stump/types'
import {
	elementScroll,
	useVirtualizer,
	useWindowVirtualizer,
	VirtualizerOptions,
} from '@tanstack/react-virtual'
import React, { useCallback, useEffect, useMemo, useRef, useState } from 'react'
import { useMediaMatch, useWindowSize } from 'rooks'

function easeInOutQuint(t: number) {
	return t < 0.5 ? 16 * t * t * t * t * t : 1 + 16 * --t * t * t * t * t
}
type Props = {
	media: Media
	// TODO: progress loaded from before reading session, will scroll to offset of page
	// initialPage: number
	onPageChange: (page: number) => void
	getPageUrl(page: number): string
}

export default function ImageBasedScrollReader({
	media,
	// currentPage,
	onPageChange,
	getPageUrl,
}: Props) {
	const parentRef = useRef<HTMLDivElement>(null)
	const visibleRef = useRef(0)

	const { innerWidth, innerHeight } = useWindowSize()

	const isAtLeastSmall = useMediaMatch('(min-height: 640px)')
	const isAtLeastMedium = useMediaMatch('(min-height: 768px)')

	// const scrollToFn: VirtualizerOptions<any, any>['scrollToFn'] = React.useCallback(
	// 	(offset, canSmooth, instance) => {
	// 		const duration = 1000
	// 		const start = parentRef.current.scrollTop
	// 		const startTime = (visibleRef.current = Date.now())

	// 		const run = () => {
	// 			if (visibleRef.current !== startTime) return
	// 			const now = Date.now()
	// 			const elapsed = now - startTime
	// 			const progress = easeInOutQuint(Math.min(elapsed / duration, 1))
	// 			const interpolated = start + (offset - start) * progress

	// 			if (elapsed < duration) {
	// 				elementScroll(interpolated, canSmooth, instance)
	// 				requestAnimationFrame(run)
	// 			} else {
	// 				elementScroll(interpolated, canSmooth, instance)
	// 			}
	// 		}

	// 		requestAnimationFrame(run)
	// 	},
	// 	[],
	// )

	const rows = Array.from({ length: 400 }, () => Math.floor(20 + Math.random() * 10))
	const rowVirtualizer = useVirtualizer({
		count: rows.length,
		estimateSize: () => 25,
		getScrollElement: () => parentRef.current,
		overscan: 5,
	})

	useEffect(
		() => {
			rowVirtualizer.measure()
		},
		// eslint-disable-next-line react-hooks/exhaustive-deps
		[innerWidth, innerHeight],
	)

	return (
		<div
			ref={parentRef}
			className=""
			style={{
				height: innerHeight || undefined,
				width: innerWidth || undefined,
			}}
		>
			<div
				className="relative inline-flex h-full"
				style={{
					height: `${rowVirtualizer.getTotalSize()}px`,
				}}
			>
				{rowVirtualizer.getVirtualItems().map((virtualRow) => {
					const row = rows[virtualRow.index]

					return (
						<div
							key={virtualRow.key}
							style={{
								height: `${row}px`,
								position: 'absolute',
								transform: `translateY(${virtualRow.start}px)`,
								width: '100%',
							}}
						>
							{virtualRow.index}
						</div>
					)
				})}
			</div>
		</div>
	)

	return (
		<div
			style={{
				height: `${rowVirtualizer.getTotalSize()}px`,
				position: 'relative',
				width: '100%',
			}}
			className="text-center"
		>
			{rowVirtualizer.getVirtualItems().map((virtualRow) => {
				const row = rows[virtualRow.index]

				return (
					<div
						key={virtualRow.key}
						style={{
							height: `${row}px`,
							position: 'absolute',
							transform: `translateY(${virtualRow.start}px)`,
							width: '100%',
						}}
						ref={virtualRow.measureElement}
					>
						{virtualRow.index}
					</div>
				)
			})}
		</div>
	)

	// return (
	// 	<div
	// 		ref={parentRef}
	// 		className="List"
	// 		style={{
	// 			height: innerHeight || '100%',
	// 			overflow: 'auto',
	// 			width: innerWidth || '100%',
	// 		}}
	// 	>
	// 		<div
	// 			style={{
	// 				height: `${rowVirtualizer.getTotalSize()}px`,
	// 				position: 'relative',
	// 				width: '100%',
	// 			}}
	// 		>
	// 			{rowVirtualizer.getVirtualItems().map((virtualItem) => {
	// 				const virtualPage = virtualItem.index + 1
	// 				const imageUrl = getPageUrl(virtualPage)

	// 				return (
	// 					<div
	// 						key={virtualItem.key}
	// 						style={{
	// 							left: 0,
	// 							position: 'relative',
	// 							top: 0,
	// 							transform: `translateY(${virtualItem.start}px)`,
	// 							width: '100%',
	// 						}}
	// 					>
	// 						{/* <img src={imageUrl} /> */}
	// 						{virtualPage}
	// 					</div>
	// 				)
	// 			})}
	// 		</div>
	// 	</div>
	// )
}
