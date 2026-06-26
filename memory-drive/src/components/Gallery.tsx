import { useRef } from "react";
import { useArchiveStore } from "../store/archiveStore";
import { useStatusStore } from "../store/statusStore";
import "./Gallery.css";
import { GalleryItem } from "./GalleryItem";

export const Gallery = () => {
    const ref = useRef<HTMLDivElement>(null)
    const { selectedYear, setSelectedYear } = useStatusStore()
    const { archiveMetadata } = useArchiveStore()

    if (selectedYear === null) return null
    if (archiveMetadata === null) return null

    const { total_imgs, total_vids, year_months } = archiveMetadata?.years[selectedYear]
    const months = Object.keys(year_months)

    return (
        <div className="window">
            <div className="title-bar">
                <div className="title-bar-text">{selectedYear}</div>
                <div className="title-bar-controls">
                    <button aria-label="Minimize"></button>
                    <button aria-label="Maximize"></button>
                    <button aria-label="Close" onClick={() => setSelectedYear(null)}></button>
                </div>
            </div>

            <div className="window-body" ref={ref}>
                {months.map(m => <div className="gallery-month">
                    <h4>{m}</h4>
                    {year_months[Number(m)].imgs.map(img => <GalleryItem ressource={img} key={img.thumb_img_path} />)}

                </div>)}

            </div>
            <div className="status-bar">
                <p className="status-bar-field">{total_imgs} pictures</p>
                <p className="status-bar-field">{total_vids} videos</p>
                <p className="status-bar-field">{months.length} months</p>
            </div>
        </div>
    )
}