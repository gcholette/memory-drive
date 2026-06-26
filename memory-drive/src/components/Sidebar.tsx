import { invoke } from "@tauri-apps/api/core";
import { useArchiveStore } from "../store/archiveStore"
import "./Sidebar.css";
import { ArchiveMetadata } from "../types/archiveMetadata";
import { useStatusStore } from "../store/statusStore";
import { useState } from "react";

export const Sidebar = () => {
    const { archiveMetadata, setArchiveMetadata } = useArchiveStore()
    const { setSelectedYear } = useStatusStore()
    const [loadingStatus, setLoadingStatus] = useState<number | null>(null)
    const [batchesToProcess, setBatchesToProcess] = useState<number | null>(null)

    const load_archive = async () => {
        const archiveData = (await invoke("load_archive", { archivePath: '/home/gcholette/Pictures/mdrive_test/' })) as ArchiveMetadata
        console.log(archiveData)
        await invoke("cache_all_thumbnails", { archivePath: '/home/gcholette/Pictures/mdrive_test/' })

        setArchiveMetadata(archiveData)
    }

    const compress_archive = async () => {
        const archiveData = (await invoke("load_archive", { archivePath: '/home/gcholette/Pictures/mdrive_test/' })) as ArchiveMetadata
        const total = archiveData.total_imgs + archiveData.total_vids
        const pageSize = 40
        const totalPages = Math.ceil(total / pageSize)

        setLoadingStatus(0)
        setBatchesToProcess(totalPages)
        for (let i = 0; i <= totalPages; i++) {
            await invoke("compress_batch", { archivePath: '/home/gcholette/Pictures/mdrive_test/', amount: pageSize, page: i })
            setLoadingStatus(i)
        }

        setLoadingStatus(null)
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
            <div>

                <button type="submit" onClick={compress_archive}>Compress Archive</button>
            </div>
            {loadingStatus !== null &&
                <div className="window loading-window" style={{"width": "300px"}}>
                    <div className="title-bar">
                        <div className="title-bar-text">Processing...</div>
                    </div>
                    <div className="window-body">
                        <div className="progress-indicator">
                            <span className="progress-indicator-bar" style={{ "width": `${Math.floor((loadingStatus / (batchesToProcess ?? 1)) * 100)}%` }} />
                        </div>
                    </div>
                </div>

            }
        </div>
    )
}