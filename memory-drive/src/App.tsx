import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [imgState, setImgState] = useState<string>("");

  async function load_archive() {
    const archiveData = await invoke("load_archive", { archivePath: '/home/gcholette/Pictures/mdrive_test/' })
    console.log(archiveData)
  }

  return (
    <main className="container">
      <button type="submit" onClick={load_archive}>Load Archive</button>
      {imgState ? <img src={imgState}></img> : <></> }
    </main>
  )
}

export default App;
