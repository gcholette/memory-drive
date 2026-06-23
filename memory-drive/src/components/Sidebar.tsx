import { useArchiveStore } from "../store/archiveStore"
import "./Sidebar.css";

export const Sidebar = () => {
    const { archiveMetadata } = useArchiveStore()

    const years = Object.keys(archiveMetadata?.years || {})

    return <div className="sidebar">
        {years.map(yr => <div>
            <button key={yr}>
                {yr}
            </button>
        </div>
        )}
    </div>
}