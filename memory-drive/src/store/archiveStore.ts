import { create } from 'zustand'
import { ArchiveMetadata } from '../types/archiveMetadata'

export type ArchiveStore = {
  archivePath: string | null
  setArchivePath: (archivePath: string | null) => void

  archiveMetadata: ArchiveMetadata | null
  setArchiveMetadata: (archiveMetadata: ArchiveMetadata) => void
}

export const useArchiveStore = create<ArchiveStore>((set) => ({
  archivePath: '/home/gcholette/Pictures/mdrive_test/',
  setArchivePath: (archivePath: string | null) => set({archivePath}),

  archiveMetadata: null,
  setArchiveMetadata: (archiveMetadata: ArchiveMetadata) => set({ archiveMetadata }),
}))
