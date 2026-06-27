import "./App.css"
import "./98.css"
import { Sidebar } from "./components/Sidebar"
import { Gallery } from "./components/Gallery"
import { useApp } from "./hooks/useApp"
import { Settings } from "./components/Settings"

export const App = () => {
  const { close, maximize, minimize } = useApp()
  return (
    <main>
      <div className="title-bar app-bar">
        <div className="title-bar-text" id="drag-region" style={{ paddingLeft: "4px"}}>
          m&nbsp;&nbsp;e&nbsp;&nbsp;m&nbsp;&nbsp;o&nbsp;&nbsp;r&nbsp;&nbsp;y&nbsp;&nbsp;&nbsp;&nbsp;d&nbsp;&nbsp;r&nbsp;&nbsp;i&nbsp;&nbsp;v&nbsp;&nbsp;e
        </div>
        <div className="title-bar-controls">
          <button aria-label="Minimize" onClick={minimize}></button>
          <button aria-label="Maximize" onClick={maximize}></button>
          <button aria-label="Close" onClick={close}></button>
        </div>
      </div>
      <div>
      </div>
      <div className="container">
        <Sidebar />
        <Gallery />
      </div>
      <Settings />
    </main>
  )
}
