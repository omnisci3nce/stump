import React from 'react'

import { Label } from '../form'
import { Text } from '../text'
import { cn } from '../utils'
import { RawInput, RawInputProps } from './raw'

// TODO: size prop
export type InputProps = {
	/** The label for the input. */
	label?: string
	/** The optional props for the label. */
	labelProps?: Omit<React.ComponentPropsWithoutRef<typeof Label>, 'children'>
	/** The optional description for the input. */
	description?: string
	/** The optional props for the description. */
	descriptionProps?: Omit<React.ComponentPropsWithoutRef<typeof Text>, 'children'>
	/** The optional class name for the container. */
	containerClassName?: string
	/** An optional right icon to display inset the input */
	icon?: React.ReactNode
} & RawInputProps

export const Input = React.forwardRef<HTMLInputElement, InputProps>(
	(
		{ label, description, labelProps, descriptionProps, containerClassName, icon, ...props },
		ref,
	) => {
		const renderIcon = () => {
			if (icon) {
				return <div className="absolute inset-y-0 right-0 flex items-center pr-3">{icon}</div>
			}

			return null
		}

		return (
			<div className={cn('grid w-full max-w-sm items-center gap-1.5', containerClassName)}>
				{label && (
					<Label htmlFor={props.id} {...(labelProps || {})}>
						{label}
					</Label>
				)}
				<div className="relative w-full">
					<RawInput {...props} ref={ref} />
					{renderIcon()}
				</div>

				{description && (
					<Text variant="muted" size="sm" {...(descriptionProps || {})}>
						{description}
					</Text>
				)}
			</div>
		)
	},
)
Input.displayName = 'Input'
