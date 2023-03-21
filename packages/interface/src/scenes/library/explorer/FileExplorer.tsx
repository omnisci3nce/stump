import { Text } from '@stump/components'
import type { DirectoryListingFile } from '@stump/types'

interface FileExplorerProps {
	files: DirectoryListingFile[]
}

// TODO: this needs to be virtualized, as I am not paginating it like other lists/grids throughout Stump.
// Look into -> https://tanstack.com/virtual/v3, doesn't look too bad
// TODO: this needs to have grid and list layout options
export default function FileExplorer({ files }: FileExplorerProps) {
	return (
		<div className="grid grid-cols-4 items-start justify-center gap-6 sm:grid-cols-5 md:grid-cols-6 md:justify-start lg:grid-cols-8">
			{files.map((file) => (
				<ExplorerFile key={file.path} {...file} />
			))}
		</div>
	)
}

// Lol the name is just reversed...
function ExplorerFile({ name, path, is_directory }: DirectoryListingFile) {
	function getIconSrc() {
		const archivePattern = new RegExp(/^.*\.(cbz|zip|rar|cbr)$/gi)

		if (is_directory) {
			return '/assets/icons/folder.png'
		} else if (archivePattern.test(path)) {
			// TODO: no lol, I want to try and render a small preview still
			// will have to create a new endpoint to try and grab a thumbnail by path
			return '/assets/icons/archive.svg'
		} else if (path.endsWith('.epub')) {
			return '/assets/icons/epub.svg'
		} else {
			return ''
		}
	}

	return (
		<button className="flex flex-col items-center justify-center space-y-2">
			{/* FIXME: don't use images for svg fallbacks... or, just set color of images... */}
			<img src={getIconSrc()} className="h-20 w-20 active:scale-[.99]" />
			<Text className="max-w-[5rem]" size="xs" noOfLines={2}>
				{name}
			</Text>
		</button>
	)
}