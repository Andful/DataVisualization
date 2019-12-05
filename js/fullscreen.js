import * as screenfull from 'screenfull';

window.fullscreen = function(elem,event) {
    console.log(event.key)
    if (!screenfull.isFullscreen && event.key == "f" && screenfull.isEnabled) {
        screenfull.request(elem);
    } else {
        screenfull.exit()
    }
}
