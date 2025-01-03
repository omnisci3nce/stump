import { useEmailerQuery, useEmailersQuery, useUpdateEmailer } from '@stump/client'
import { useEffect, useMemo } from 'react'
import { useNavigate, useParams } from 'react-router'

import { ContentContainer, SceneContainer } from '@/components/container'
import paths from '@/paths'

import { useEmailerSettingsContext } from './context'
import { CreateOrUpdateEmailerForm, CreateOrUpdateEmailerSchema } from './emailers'

export default function EditEmailerScene() {
	const navigate = useNavigate()

	const { id: rawId } = useParams<{ id: string }>()
	const id = useMemo(() => parseInt(rawId || '', 10), [rawId])

	const { canEditEmailer } = useEmailerSettingsContext()
	const { emailer } = useEmailerQuery({
		enabled: !isNaN(id),
		id,
		suspense: true,
	})
	const { emailers } = useEmailersQuery({ suspense: true })
	const { updateAsync: updateEmailer } = useUpdateEmailer({
		id,
	})

	useEffect(() => {
		if (isNaN(id) || !emailer) {
			navigate(paths.notFound())
		} else if (!canEditEmailer) {
			navigate('..', { replace: true })
		}
	}, [id, emailer, navigate, canEditEmailer])

	const onSubmit = async ({ name, is_primary, ...config }: CreateOrUpdateEmailerSchema) => {
		try {
			await updateEmailer({
				config: {
					...config,
					host: config.smtp_host,
					max_attachment_size_bytes: config.max_attachment_size_bytes ?? null,
					// TODO: support configuring this
					max_num_attachments: null,
					password: config.password?.length ? config.password : null,
					port: config.smtp_port,
				},
				is_primary,
				name,
			})
			navigate(paths.settings('server/email'))
		} catch (error) {
			console.error(error)
			// TODO:toast
		}
	}

	if (!emailer || !canEditEmailer) {
		return null
	}

	return (
		<SceneContainer>
			<ContentContainer>
				<CreateOrUpdateEmailerForm
					emailer={emailer}
					existingNames={emailers?.map((e) => e.name) || []}
					onSubmit={onSubmit}
				/>
			</ContentContainer>
		</SceneContainer>
	)
}
