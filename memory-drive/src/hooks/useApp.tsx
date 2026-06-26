import { getCurrentWindow } from "@tauri-apps/api/window";
import { useEffect } from "react";

export const useApp = () => {
    const appWindow = getCurrentWindow();
    const minimize = () => appWindow.minimize();
    const maximize = () => appWindow.toggleMaximize();
    const close = () => appWindow.close();

    useEffect(() => {
        console.log("Useeffected")
        document.getElementById('drag-region')?.addEventListener('mousedown', (e) => {
            if (e.buttons === 1) {
                // Primary (left) button
                e.detail === 2
                    ? appWindow.toggleMaximize() // Maximize on double click
                    : appWindow.startDragging(); // Else start dragging
            }
        });

    }, [])



    return { minimize, maximize, close }
}
