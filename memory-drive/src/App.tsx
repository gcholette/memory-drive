import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [imgState, setImgState] = useState<string>("");

  async function load_img() {
    const imgData = new Uint8Array(await invoke("load_img", { filePath: '/home/gcholette/Pictures/459149823_931915115651346_6145531330672003157_n.jpg' }))
    const img = new Blob([imgData.buffer], { type: 'image/png' } )
    setImgState(URL.createObjectURL(img))
  }

  return (
    <main className="container">
      <button type="submit" onClick={load_img}>Load img</button>
      {imgState ? <img src={imgState}></img> : <></> }
    </main>
  )
}

export default App;
