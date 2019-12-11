importScripts("/pkg/index.js");
async function run(){
    await self.wasm_bindgen("/pkg/index_bg.wasm");
    pathFinder = self.wasm_bindgen.PathFinder.new(self)

    onmessage = function(e) {
        pathFinder.onmessage(e.data);
    }
    console.log("worker online")
}
run()
//onmessage = function(x) {
//    console.log(x);
//}
