import "./App.css"
import "./98.css"
import { Sidebar } from "./components/Sidebar"
import { Gallery } from "./components/Gallery"

export const App = () =>
  <main className="container">
    <Sidebar />
    <Gallery />
  </main>

