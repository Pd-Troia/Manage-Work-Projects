import * as React from "react"
import "./App.css";
import ProjectList from "./components/project_list";
import { checkForUpdate } from "./updater"

function App() {
  React.useEffect(() => { checkForUpdate() }, [])

    return (
    <main className="container">
      <ProjectList/>
    </main>
  );
}

export default App;
