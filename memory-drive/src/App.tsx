import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { ArchiveMetadata } from "./types/archiveMetadata";
import { useArchiveStore } from "./store/archiveStore";
import { Sidebar } from "./components/Sidebar";

export const App = () => {
  const { setArchiveMetadata } = useArchiveStore()

  async function load_archive() {
    const archiveData = (await invoke("load_archive", { archivePath: '/home/gcholette/Pictures/mdrive_test/' })) as ArchiveMetadata

    setArchiveMetadata(archiveData)
    console.log(archiveData)
  }

  return (
    <main className="container">
      <Sidebar />
      <div>
        <button type="submit" onClick={load_archive}>Load Archive</button>
      </div>
    </main>
  )
}

