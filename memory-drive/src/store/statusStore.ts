import { create } from 'zustand'

type Year = number


export type StatusStore = {
  selectedYear: Year | null
  setSelectedYear: (year: Year | null) => void

  loadingStatus: number | null,
  setLoadingStatus: (loadingStatus: number | null) => void

  batchesToProcess: number | null,
  setBatchesToProcess: (batchesToProcess: number | null) => void

  settingsIsOpen: boolean
  openSettings: () => void
  closeSettings: () => void
}

export const useStatusStore = create<StatusStore>((set) => ({
  selectedYear: null,
  setSelectedYear: (year: Year | null) => set({ selectedYear: year }),

  loadingStatus: null,
  setLoadingStatus: (loadingStatus) => set({loadingStatus}),

  batchesToProcess: null,
  setBatchesToProcess: (batchesToProcess) => set({batchesToProcess}),

  settingsIsOpen: false,
  openSettings: () => set({ settingsIsOpen: true }),
  closeSettings: () => set({ settingsIsOpen: false })
}))
