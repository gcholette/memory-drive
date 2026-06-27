import { useRef } from "react"
import { useArchiveStore } from "../store/archiveStore"
import { useStatusStore } from "../store/statusStore"


export const Settings = () => {
    const { settingsIsOpen, closeSettings } = useStatusStore()
    const {archivePath, setArchivePath} = useArchiveStore()
    const archivePathValue = useRef<string>(archivePath || "")

    if (!settingsIsOpen) return null

    const onSave = () => {
        setArchivePath(archivePathValue.current)
    }

    return (
        <div className="window dialog" style={{ "width": "500px" }}>
            <div className="title-bar">
                <div className="title-bar-text">Settings</div>
                <div className="title-bar-controls">
                    <button aria-label="Close" onClick={closeSettings}></button>
                </div>
            </div>
            <div className="window-body" style={{ padding: "1px" }}>
                <div className="field-row-stacked" style={{ "width": "100%" }}>
                    <label>Archive Path</label>
                    <input type="text" defaultValue={archivePathValue.current} onChange={(e) => { archivePathValue.current = e.target.value }}/>
                </div>
                <button className="save-button" onClick={onSave}>Save</button>
            </div>
        </div>
    )
}