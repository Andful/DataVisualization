importScripts("./pkg/index.js");
importScripts("https://d3js.org/d3.v5.min.js");
async function run() {
    await self.wasm_bindgen("./pkg/index_bg.wasm");
    const e=await d3.json("data/timetable.json.real");
    console.log("loading started");
    let pathFinder=self.wasm_bindgen.PathFinder.load_data(self,JSON.stringify(e));
    console.log("loading finished");
    onmessage = function(e) {
        pathFinder.onmessage(e.data)
    };
    console.log("worker online");
    postMessage("ready")
}

run();
