import React from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { getMediaPage } from '~api/query/media';

// TODO: handle redirects I will *probably* add for when a user
// comes here trying to read pages of an epub.
export default function ReadBook() {
	const navigate = useNavigate();

	const { id, page } = useParams();

	if (!id) {
		throw new Error('Media id is required');
	} else if (!page) {
		navigate(`/books/${id}/pages/1`);

		// TODO: do I need this?
		return null;
	}

	return (
		<div className="h-full w-full flex justify-center items-center">
			<img
				// Note: Comic book ratio is -> 663 : 1024
				className="object-scale-down max-h-full"
				src={getMediaPage(id, parseInt(page, 10))}
				onError={(err) => {
					// @ts-ignore
					err.target.src = '/src/favicon.png';
				}}
			/>
		</div>
	);
}