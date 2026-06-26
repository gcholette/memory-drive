import { create } from 'zustand'

type Year = number


export type SelectionStore = {
  selectedYear: Year | null
  isScrolling: boolean
  setSelectedYear: (year: Year | null) => void
  setIsScrolling: (isScrolling: boolean) => void
}

export const useStatusStore = create<SelectionStore>((set) => ({
  selectedYear: null,
  isScrolling: false,
  setSelectedYear: (year: Year | null) => set({ selectedYear: year }),
  setIsScrolling: (isScrolling: boolean) => set({ isScrolling }),
}))
