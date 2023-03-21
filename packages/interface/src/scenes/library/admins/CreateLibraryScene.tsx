import { Heading, Link, Text } from '@stump/components'

import { useLocaleContext } from '../../../i18n'

export default function CreateLibraryScene() {
	const { t } = useLocaleContext()

	return (
		<div className="flex flex-col gap-4">
			<header className="flex flex-col gap-2">
				<Heading size="lg">{t('createLibraryScene.heading')}</Heading>
				<Text size="sm" variant="muted">
					{t('createLibraryScene.subtitle')}{' '}
					<Link href="https://stumpapp.dev/guides/libraries">
						{t('createLibraryScene.subtitleLink')}.
					</Link>
				</Text>
			</header>
		</div>
	)
}