import * as screenfull from 'screenfull';

window.fullscreen = function(elem,event) {
    event.preventDefault();
    if (!screenfull.isFullscreen && event.key.toLowerCase() == "f" && event.shiftKey && screenfull.isEnabled) {
        screenfull.request(elem);
        console.log("yes");
    }
    return false;
}
