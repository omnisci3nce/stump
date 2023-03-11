# Components

> :warning: **This package is very early in development and is not actually ready for usage in Stump.**
> Once it is ready, Chakra UI will be phased out in favor of this package.

This package is the component library used throughout the Stump web and desktop interface(s). A vast majority of the foundational components come from [shadcn/ui](https://ui.shadcn.com/docs), a wonderful free and open source collection of React components build with [radix-ui](https://radix-ui.com/) and [TailwindCSS](https://tailwindcss.com/). Huge thanks to [shadcn](https://github.com/shadcn) for their work!

## Usage

When developing components within this package, it is really useful to have the Storybook running. To do so, run the following command:

```bash
# Using moon from the project root directory
moon run components:storybook

# Using PNPM from the package root directory
pnpm run storybook
```

## Structure

The components are organized and grouped into folders based on their most abstracted use case / category. For example, all variants of a button are grouped together in the `button` folder. This is to make it easier to find the component you are looking for, as well as to group the Storybook stories together in the same way you would find them in the Storybook UI.