import * as d3 from "d3";

let svg = d3.select("#main")
let tooltip = d3.select("#tooltip")
let text = tooltip.select('text')
let rect = tooltip.select('rect')

export function showTooltip(d,i) {
    let transform = this.getCTM();
    var position = svg.node().createSVGPoint();
    let selected = d3.select(this)
    position.x = selected.attr("cx");
    position.y = selected.attr("cy");
    position = position.matrixTransform(transform);
    tooltip.attr("transform",`translate(${position.x},${position.y - 15})`)
    text.text(d.code)
    //d3.select(`#${d.code}`).attr("r","10").attr("stroke","red");
    let bBox = text.node().getBBox();
    rect
        .attr("x",-bBox.width/2 - 3)
        .attr("y",-bBox.height - 3)
        .attr("width",bBox.width + 6)
        .attr("height", bBox.height + 6)
        .attr("fill", "white")
    tooltip.attr("opacity",1)
}

export function removeTooltip(d,i) {
    tooltip.attr("opacity",0)
}
