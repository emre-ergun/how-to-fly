import * as sim from "lib-simulation-wasm";
import { useEffect } from "react";

const simulation = new sim.Simulation();
const world = simulation.world();

function App() {
  useEffect(() => {
    for (const animal of world.animals) {
      console.log(animal.x, animal.y);
    }
  }, [])

  return <div>Hello</div>

}

export default App
