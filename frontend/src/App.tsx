import { useState } from "react";
import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";
import TextEditor from "./components/TextEditor";
import "./App.css";

function App() {
  const [count, setCount] = useState(0);

  return (
    <>
      <TextEditor />
    </>
  );
}

export default App;
