import { invoke } from "@tauri-apps/api/core";
import { useArchiveStore } from "../store/archiveStore"
import "./Sidebar.css";
import { ArchiveMetadata } from "../types/archiveMetadata";
import { useSelectionStore } from "../store/selectionStore";

export const Sidebar = () => {
    const { archiveMetadata, setArchiveMetadata } = useArchiveStore()
    const { setSelectedYear } = useSelectionStore()

    const load_archive = async () => {
        const archiveData = (await invoke("load_archive", { archivePath: '/home/gcholette/Pictures/mdrive_test/' })) as ArchiveMetadata

        setArchiveMetadata(archiveData)
        console.log(archiveData)
    }


    const years = Object.keys(archiveMetadata?.years || {})

    return (
        <div className="sidebar">
            {years.map(yr => <div>
                <button key={yr} className="year" onClick={() => setSelectedYear(Number(yr))}>
                    {yr.split('').map(x => `${x} `).join('')}
                </button>
            </div>
            )}
            <div>
                <button type="submit" onClick={load_archive}>Load Archive</button>
            </div>
        </div>
    )
}