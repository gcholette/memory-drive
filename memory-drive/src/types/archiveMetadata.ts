type Totals = {
    total_imgs: number,
    total_vids: number,
}

export type ImgMetadata = {
    full_img_path: string,
    thumb_img_path: string,
    img_name: string,
    mime: string,
    year: number,
    month: number,
}

export type ArchiveLeafMetadata = Totals & {
    imgs: ImgMetadata[],
}

export type ArchiveLeafMap = Record<number, ArchiveLeafMetadata>;

export type ArchiveYearMetadata = Totals & {
    year_months: ArchiveLeafMap,
}

export type ArchiveYearMap = Record<number, ArchiveYearMetadata>;

export type ArchiveMetadata = Totals & {
    years: ArchiveYearMap,
}

