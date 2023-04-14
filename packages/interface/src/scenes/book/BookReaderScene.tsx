import { getMediaPage } from '@stump/api'
import { useMediaByIdQuery, useUpdateMediaProgress } from '@stump/client'
import { Navigate, useNavigate, useParams, useSearchParams } from 'react-router-dom'

import ImageBasedReader, {
	AnimatedImageBasedReader,
} from '../../components/readers/ImageBasedReader'
import ImageBasedScrollReader from '../../components/readers/ImageBasedScrollReader'
import paths from '../../paths'
import { ARCHIVE_EXTENSION, EBOOK_EXTENSION } from '../../utils/patterns'

export default function BookReaderScene() {
	const [search] = useSearchParams()
	const navigate = useNavigate()

	const { id, page } = useParams()
	if (!id) {
		throw new Error('You must provide a book ID for the reader.')
	}

	const { isLoading: fetchingBook, media } = useMediaByIdQuery(id)
	const { updateReadProgress } = useUpdateMediaProgress(id, {
		onError(err) {
			console.error(err)
		},
	})

	function handleChangePage(newPage: number) {
		updateReadProgress(newPage)
		// search.set('page', newPage.toString())
		// setSearch(search)
		navigate(paths.bookReader(id!, { page: newPage }))
	}

	if (fetchingBook) {
		return null
	} else if (!media) {
		return <Navigate to={paths.notFound()} />
	}

	if (media.extension.match(EBOOK_EXTENSION)) {
		return <Navigate to={paths.bookReader(id, { isEpub: true })} />
	}

	// TODO: I cannot do this anymore... I think page should just become a search param,
	// and then if scroll is not set then these can be applied... Therefore, I think the route
	// in the app router could be: reader?page=1 OR reader?scroll=vertical&page=3
	else if (!page || parseInt(page, 10) <= 0) {
		return <Navigate to={paths.bookReader(id, { page: 1 })} />
	} else if (parseInt(page, 10) > media.pages) {
		return <Navigate to={paths.bookReader(id, { page: media.pages })} />
	}

	if (media.extension.match(ARCHIVE_EXTENSION)) {
		const scroll = search.get('scroll')
		const initialPage = page ? parseInt(page, 10) : media.current_page || undefined

		if (scroll) {
			return (
				<ImageBasedScrollReader
					media={media}
					initialPage={initialPage}
					//! FIXME: don't be lazy aaron...
					orientation={scroll as 'horizontal' | 'vertical'}
					getPageUrl={(pageNumber) => getMediaPage(id, pageNumber)}
					onProgressUpdate={updateReadProgress}
				/>
			)
		}

		const animated = search.get('animated')
		// TODO: this will be merged under ImageBasedReader once animations get stable. animation will become a prop
		// eventually. This is just a debug tool for me right now, and will not remain as separate components in the future.
		const Component = animated ? AnimatedImageBasedReader : ImageBasedReader

		return (
			<Component
				media={media}
				currentPage={parseInt(page, 10)}
				getPageUrl={(pageNumber) => getMediaPage(id, pageNumber)}
				onPageChange={handleChangePage}
			/>
		)
	}

	return <div>Not a supported book or i just can&rsquo;t do that yet! :)</div>
}
