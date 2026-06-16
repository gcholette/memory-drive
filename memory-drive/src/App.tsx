import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [imgState, setImgState] = useState<string>("");

  async function load_img() {
    const imgData = new Uint8Array(await invoke("load_imgs_in_path", { filePath: '/home/gcholette/Pictures/' }))
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
