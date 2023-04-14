import type { Media } from '@stump/types'
import { useVirtualizer, useWindowVirtualizer } from '@tanstack/react-virtual'
import { useCallback, useEffect, useLayoutEffect, useRef, useState } from 'react'
import { useMediaMatch, useWindowSize } from 'rooks'

type Props = {
	media: Media
	// TODO: progress loaded from before reading session, will scroll to offset of page
	// initialPage: number
	getPageUrl(page: number): string
}

export default function ImageBasedScrollReader({ media, getPageUrl }: Props) {
	const parentRef = useRef<HTMLDivElement>(null)
	const [imageSizes, setImageSizes] = useState<Record<number, number>>({})

	const isAtLeastSmall = useMediaMatch('(min-width: 640px)')

	const { innerWidth, innerHeight } = useWindowSize()

	const estimateSize = useCallback(
		(index: number) => {
			const loadedSize = imageSizes[index]
			return loadedSize ?? (innerHeight || 35)
		},
		[imageSizes, innerHeight],
	)

	const rowVirtualizer = useVirtualizer({
		count: media.pages,
		estimateSize,
		getScrollElement: () => parentRef.current,
		overscan: 5,
	})

	useEffect(
		() => {
			rowVirtualizer.measure()
		},
		// eslint-disable-next-line react-hooks/exhaustive-deps
		[innerWidth, innerHeight, isAtLeastSmall],
	)

	// return <RowVirtualizerDynamicWindow count={media.pages} getPageUrl={getPageUrl} />

	// // TODO: scrollbar width
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
					height: `${rowVirtualizer.getTotalSize()}px`,
				}}
			>
				{rowVirtualizer.getVirtualItems().map((virtualRow) => {
					const virtualPage = virtualRow.index + 1
					const imageUrl = getPageUrl(virtualPage)

					return (
						<div
							key={virtualRow.key}
							style={{
								position: 'absolute',
								transform: `translateY(${virtualRow.start}px)`,
								width: '100%',
							}}
							className="flex items-start justify-center"
						>
							<img
								className="max-h-full w-full select-none object-scale-down md:w-auto"
								src={imageUrl}
								style={{
									maxHeight: innerHeight,
								}}
								ref={rowVirtualizer.measureElement}
								onLoad={(e) => {
									const height = e.currentTarget.naturalHeight
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

const RowVirtualizerDynamicWindow = ({
	count,
	getPageUrl,
}: {
	count: number
	getPageUrl(page: number): string
}) => {
	const parentRef = useRef<HTMLDivElement>(null)
	const [imageSizes, setImageSizes] = useState<Record<number, number>>({})

	const parentOffsetRef = useRef(0)

	const isAtLeastSmall = useMediaMatch('(min-width: 640px)')

	const { innerWidth, innerHeight } = useWindowSize()

	// const estimateSize = useCallback(() => {
	// 	if (!isAtLeastSmall) {
	// 		return innerWidth || 0
	// 	} else {
	// 		return innerHeight || 0
	// 	}
	// }, [innerHeight, innerWidth, isAtLeastSmall])

	const estimateSize = useCallback(
		(index: number) => {
			const loadedSize = imageSizes[index]
			return loadedSize ?? (innerHeight || 35)
		},
		[imageSizes, innerHeight],
	)

	useLayoutEffect(() => {
		parentOffsetRef.current = parentRef.current?.offsetTop ?? 0
	}, [])

	const virtualizer = useWindowVirtualizer({
		count,
		// estimateSize: () => 100,
		estimateSize,
		overscan: 0,
		scrollMargin: parentOffsetRef.current,
	})
	const items = virtualizer.getVirtualItems()

	useEffect(() => {
		virtualizer.measure()
	}, [virtualizer])

	console.log('items', items)
	console.log('count', count)

	return (
		<div ref={parentRef} className="max-w-full">
			<div
				style={{
					height: virtualizer.getTotalSize(),
					position: 'relative',
					width: '100%',
				}}
			>
				<div
					style={{
						left: 0,
						position: 'absolute',
						top: 0,
						transform: `translateY(${items[0]!.start - virtualizer.options.scrollMargin}px)`,
						width: '100%',
					}}
				>
					{items.map((virtualRow) => {
						const virtualPage = virtualRow.index + 1
						const imageUrl = getPageUrl(virtualPage)
						return (
							<div
								key={virtualRow.key}
								data-index={virtualRow.index}
								className="flex justify-center"
							>
								<img
									className="max-h-full w-full select-none object-scale-down md:w-auto"
									style={{
										maxHeight: innerHeight,
									}}
									src={imageUrl}
									ref={virtualizer.measureElement}
									onLoad={(e) => {
										const height = e.currentTarget.naturalHeight
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
		</div>
	)
}
