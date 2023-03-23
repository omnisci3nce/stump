import { Check, ChevronsUpDown } from 'lucide-react'
import { Fragment, useState } from 'react'

import { Button, Command, Label, Popover, Text } from '..'
import { cn } from '../utils'

// TODO: customize multi display value, e.g. "2 items selected"
export type ComboBoxOption = {
	label: string
	value: string
}

type SingleSelectComboBoxProps = {
	isMultiSelect?: false
	value?: string
	onChange?: (value?: string) => void
}
type MultiSelectComboBoxProps = {
	isMultiSelect: true
	value?: string[]
	onChange?: (value?: string[]) => void
}

const SIZE_VARIANTS = {
	default: 'w-[200px]',
	full: 'w-full',
	lg: 'w-[300px]',
	md: 'w-[250px]',
	sm: 'w-[150px]',
}

export type ComboBoxProps = {
	label?: string
	description?: string
	options: ComboBoxOption[]
	size?: keyof typeof SIZE_VARIANTS
	triggerClassName?: string
	wrapperClassName?: string
	placeholder?: string
	filterable?: boolean
	filterPlaceholder?: string
	filterEmptyMessage?: string
} & (SingleSelectComboBoxProps | MultiSelectComboBoxProps)

export function ComboBox({
	label,
	description,
	isMultiSelect,
	options,
	value,
	onChange,
	size = 'default',
	triggerClassName,
	wrapperClassName,
	placeholder = 'Select...',
	filterable = false,
	filterPlaceholder = 'Filter...',
	filterEmptyMessage = 'No results found',
}: ComboBoxProps) {
	const [open, setOpen] = useState(false)

	const handleChange = (selected: string) => {
		if (isMultiSelect) {
			if (value?.includes(selected)) {
				onChange?.(value.filter((item) => item !== selected))
			} else if (value) {
				onChange?.([...value, selected])
			} else {
				onChange?.([selected])
			}
		} else {
			onChange?.(selected)
			setOpen(false)
		}
	}

	const renderSelected = () => {
		if (!value) {
			return placeholder
		}

		if (isMultiSelect) {
			return (
				value
					?.map((selected) => {
						const option = options.find((option) => option.value === selected)
						return option?.label
					})
					.join(', ') || placeholder
			)
		} else {
			return options.find((option) => option.value === value)?.label || placeholder
		}
	}

	const Container = label || description ? 'div' : Fragment
	const containerProps = {
		...((label || description) && { className: 'flex flex-col gap-1.5' }),
	}

	return (
		<Container {...containerProps}>
			{label && <Label>{label}</Label>}
			<Popover open={open} onOpenChange={setOpen}>
				<Popover.Trigger asChild>
					<Button
						variant="outline"
						role="combobox"
						aria-expanded={open}
						className={cn(SIZE_VARIANTS[size], 'justify-between truncate', triggerClassName)}
					>
						{renderSelected()}
						<ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
					</Button>
				</Popover.Trigger>
				<Popover.Content className={cn(SIZE_VARIANTS[size], 'p-0', wrapperClassName)}>
					<Command>
						{filterable && (
							<>
								<Command.Input placeholder={filterPlaceholder} />
								<Command.Empty>{filterEmptyMessage}</Command.Empty>
							</>
						)}
						<Command.Group>
							{options.map((option) => {
								const isSelected = value?.includes(option.value) || false

								return (
									<Command.Item
										key={option.value}
										onSelect={handleChange}
										className={cn('transition-all duration-75', { 'text-brand': isSelected })}
										value={option.value}
									>
										<Check
											className={cn('mr-2 h-4 w-4', isSelected ? 'opacity-100' : 'opacity-0')}
										/>
										{option.label}
									</Command.Item>
								)
							})}
						</Command.Group>
					</Command>
				</Popover.Content>
			</Popover>
			{description && (
				<Text size="sm" variant="muted">
					{description}
				</Text>
			)}
		</Container>
	)
}