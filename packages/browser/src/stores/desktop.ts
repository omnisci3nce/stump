import { useDesktopAppContext } from '@stump/client'
import { DesktopAppStore, SavedServer } from '@stump/sdk'
import { useCallback, useEffect } from 'react'
import { create } from 'zustand'

/**
 * The type which represents the local store, which is a superset of the desktop application store
 * with a 'set' method to update the store as a sort of 'patch' operation
 */
type LocalStore = DesktopAppStore & {
	set: (args: Partial<DesktopAppStore>) => void
}

/**
 * A 'local' zustand store to sync the desktop application store with components
 * across the application
 */
const useLocalStore = create<LocalStore>()((set) => ({
	connected_servers: [],
	run_bundled_server: true,
	set: (partial) => set((state) => ({ ...state, ...partial })),
}))

/**
 * A hook to interact with the desktop application store. There is a little bit of a
 * synchronization dance here:
 *
 * Zustand is used to provide easier access to the store's data throughout the application,
 * but the actual store is persisted to disk and managed by Tauri. This hook will load the
 * store on mount and manually sync the two when changes are made. Since this is all local
 * to a single desktop application, there isn't really any need for a more complex solution.
 */
export const useTauriStore = () => {
	const { store } = useDesktopAppContext()

	const localStore = useLocalStore()

	/**
	 * An effect to save the store to disk whenever it changes. This is a bit overly
	 * cautious because of the following snippet from the docs:
	 *
	 * > As the store is only persisted to disk before the apps exit, changes might be lost in a crash.
	 * > This method lets you persist the store to disk whenever you deem necessary.
	 *
	 * @see {@link Store.save}
	 */
	useEffect(() => {
		store.save().catch((e) => console.error('Failed to save store', e))
	}, [store, localStore])

	/**
	 * An effect to load the store from disk on mount and sync it with the local store
	 */
	useEffect(
		() => {
			const init = async () => {
				try {
					const storeEntries = await store.entries()
					const loadedStore = storeEntries.reduce(
						(acc, [key, value]) => {
							acc[key] = value
							return acc
						},
						{} as Record<string, unknown>,
					)
					// TODO: smarter type assertions here
					if ('connected_servers' in loadedStore) {
						localStore.set(loadedStore as DesktopAppStore)
					}
				} catch (e) {
					console.error('Failed to load store', e)
				}
			}

			init()
		},
		// eslint-disable-next-line react-hooks/exhaustive-deps
		[],
	)

	/**
	 * Add a server to the list of connected servers. If this is the first server added,
	 * it will also be set as the active server
	 *
	 * @param server The server to add
	 */
	const addServer = useCallback(
		async (server: SavedServer) => {
			const isEmpty = localStore.connected_servers.length === 0
			await store.set('connected_servers', [...localStore.connected_servers, server])
			if (isEmpty) {
				await store.set('active_server', server)
			}
			localStore.set({
				connected_servers: [...localStore.connected_servers, server],
				...(isEmpty ? { active_server: server } : {}),
			})
			return server
		},
		[store, localStore],
	)

	/**
	 * Edit a server in the list of connected servers
	 *
	 * @param originalName The original name of the server, used to find the server to edit
	 * @param server The new server data
	 */
	const editServer = useCallback(
		async (originalName: string, server: SavedServer) => {
			const existingServer = localStore.connected_servers.find((s) => s.name === originalName)
			if (!existingServer) {
				return
			}
			const isActiveServer = localStore.active_server?.name === originalName
			const newServers = localStore.connected_servers.map((s) =>
				s.name === originalName ? server : s,
			)
			await store.set('connected_servers', newServers)
			localStore.set({
				connected_servers: newServers,
				...(isActiveServer ? { active_server: server } : {}),
			})
		},
		[store, localStore],
	)

	/**
	 * Remove a server from the list of connected servers
	 *
	 * @param name The name of the server to remove
	 */
	const removeServer = useCallback(
		async (name: string) => {
			const newServers = localStore.connected_servers.filter((server) => server.name !== name)
			await store.set('connected_servers', newServers)
			localStore.set({
				connected_servers: newServers,
			})
		},
		[store, localStore],
	)

	/**
	 * Reset the store, deleting all connected servers and the active server.
	 */
	const resetStore = useCallback(async () => {
		await store.clear()
		localStore.set({
			active_server: undefined,
			connected_servers: [],
		})
	}, [store, localStore])
	/**
	 * Set the active server
	 *
	 * @param name The name of the server to set as active
	 */
	const setActiveServer = useCallback(
		async (name: string) => {
			const activeServer = localStore.connected_servers.find((server) => server.name === name)
			if (activeServer) {
				await store.set('active_server', activeServer)
			}
		},
		[store, localStore],
	)
	/**
	 * Set whether to run the bundled server or not
	 */
	const setRunBundledServer = useCallback(
		async (value: boolean) => {
			await store.set('run_bundled_server', value)
			localStore.set({
				run_bundled_server: value,
			})
		},
		[store, localStore],
	)

	return {
		...localStore,
		addServer,
		editServer,
		removeServer,
		resetStore,
		setActiveServer,
		setRunBundledServer,
	}
}
