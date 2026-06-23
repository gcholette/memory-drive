import { create } from 'zustand'
import { ArchiveMetadata } from '../types/archiveMetadata'

export type ArchiveStore = {
  archiveMetadata: ArchiveMetadata | null
  setArchiveMetadata: (archiveMetadata: ArchiveMetadata) => void
}

export const useArchiveStore = create<ArchiveStore>((set) => ({
  archiveMetadata: null,
  setArchiveMetadata: (archiveMetadata: ArchiveMetadata) => set({ archiveMetadata }),
}))
