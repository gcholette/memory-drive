import { ImgMetadata } from "../types/archiveMetadata"

type Props = {
    ressource: ImgMetadata
}

export const GalleryItem = ({ressource}: Props) => {
    if (ressource.mime !== "Jpg") return null

    return (
        <div className="status-field-border gallery-item" >
            Dynamic content
        </div>
    )
}