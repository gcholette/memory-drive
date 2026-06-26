import { useEffect, useRef, useState } from "react"
import { ImgMetadata } from "../types/archiveMetadata"
import { invoke } from "@tauri-apps/api/core"
import { useOnScreen } from "../hooks/useOnScreen"

type Props = {
    ressource: ImgMetadata
}

export const GalleryItem = ({ressource}: Props) => {
    if (ressource.mime !== "Jpg") return null
    const [thumb, setThumb] = useState<undefined | string>(undefined)
    const ref = useRef<HTMLDivElement>(null)
    const isVisible = useOnScreen(ref)

    useEffect(() => {
        const run = async () => {
            const bytes = await invoke<number[]>("load_image", { path: ressource.thumb_img_path });
            const blob = new Blob([new Uint8Array(bytes)], { type: "image/jpeg" });
            const url = URL.createObjectURL(blob);
            setThumb(url)
        }

        if (isVisible) {
            run()
        }
    }, [isVisible, setThumb])

    return (
        <div className="field-border gallery-item" ref={ref}>
            {isVisible && thumb && <img src={thumb} />}
        </div>
    )
}