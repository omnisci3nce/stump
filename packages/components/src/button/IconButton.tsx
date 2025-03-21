import { cva, VariantProps } from 'class-variance-authority'
import { forwardRef } from 'react'

import { cn } from '../utils'
import { BUTTON_BASE_CLASSES, BUTTON_ROUNDED_VARIANTS, BUTTON_VARIANTS } from './Button'

const iconButtonVariants = cva(BUTTON_BASE_CLASSES, {
	defaultVariants: {
		rounded: 'default',
		size: 'sm',
		variant: 'default',
	},
	variants: {
		rounded: BUTTON_ROUNDED_VARIANTS,
		size: {
			lg: 'p-3',
			md: 'p-2',
			sm: 'p-2',
			xs: 'p-1.5 md:p-1',
			xxs: 'p-0.5',
		},
		variant: BUTTON_VARIANTS,
	},
})

export type IconButtonProps = React.ButtonHTMLAttributes<HTMLButtonElement> &
	VariantProps<typeof iconButtonVariants> & {
		pressEffect?: boolean
		primaryFocus?: boolean
	}

const IconButton = forwardRef<HTMLButtonElement, IconButtonProps>(
	(
		{ className, variant, size, rounded, pressEffect = true, primaryFocus = true, ...props },
		ref,
	) => {
		return (
			<button
				className={cn(
					iconButtonVariants({ className, rounded, size, variant }),
					{
						'active:scale-95': pressEffect,
						'cursor-not-allowed': props.disabled,
						'focus:ring-edge-brand dark:focus:ring-edge-brand': primaryFocus,
					},
					className,
				)}
				ref={ref}
				type="button"
				{...props}
			/>
		)
	},
)
IconButton.displayName = 'IconButton'

export { IconButton, iconButtonVariants }
