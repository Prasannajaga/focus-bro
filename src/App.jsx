import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [items, setItems] = useState([]);

  useEffect(() => {
    async function load() {
      const raw = await invoke("read_file");
      setItems(raw);
    }

    load();
  }, []);

  function isUrl(str) {
    try {
      new URL(str);
      return true;
    } catch {
      return false;
    }
  }

  return (
    <div className="container">
      <h2 className="header">This is what matters</h2>

      <div className="items-grid">
        {items.map((item, i) => (
          <div key={i} className="item-card">
            {isUrl(item) ? (
              <a
                href={item}
                onClick={(e) => {
                  e.preventDefault();
                  invoke("open_url", { url: item });
                }}
                className="item-link"
              >
                {item}
              </a>
            ) : (
              <div className="item-text">{item}</div>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}

export default App;
