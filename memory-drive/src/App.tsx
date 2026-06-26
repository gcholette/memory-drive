import "./App.css"
import "./98.css"
import { Sidebar } from "./components/Sidebar"
import { Gallery } from "./components/Gallery"
import { useApp } from "./hooks/useApp"



export const App = () => {
  const { close, maximize, minimize } = useApp()
  return (
    <main>
      <div className="title-bar app-bar">
        <div className="title-bar-text" id="drag-region">Memory Drive</div>
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
    </main>
  )

}
