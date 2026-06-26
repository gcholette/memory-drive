import { RefObject, useEffect, useMemo, useState } from "react"

// Source - https://stackoverflow.com/a/65008608
// Posted by GuCier, modified by community.
// Retrieved 2026-06-25, License - CC BY-SA 4.0
export const useOnScreen = (ref: RefObject<HTMLElement>) => {

    const [isIntersecting, setIntersecting] = useState(false)

    const observer = useMemo(() => new IntersectionObserver(
        ([entry]) => {
            setIntersecting(entry.isIntersecting)
        }
    ), [ref])


    useEffect(() => {
        if (ref.current) {
            observer.observe(ref.current)
        }
        return () => observer.disconnect()
    }, [])

    return isIntersecting
}
