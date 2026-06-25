import { create } from 'zustand'

type Year = number

export type SelectionStore = {
  selectedYear: Year | null
  setSelectedYear: (year: Year | null) => void
}

export const useSelectionStore = create<SelectionStore>((set) => ({
  selectedYear: null,
  setSelectedYear: (year: Year | null) => set({ selectedYear: year }),
}))

