import * as d3 from 'd3'
import {get_code_to_station} from "./global.js"

export function show_city(city) {
    let citypoint = d3.select("#map").node().createSVGPoint();
    let elem = d3.select(`#${city}`);
    let matrix = elem.node().getCTM();
    citypoint.x = elem.attr("x1");
    citypoint.y = elem.attr("y1");

    let position = citypoint.matrixTransform(matrix);
    let tooltip = d3.select("#tooltip");
    tooltip.select("text").text(get_code_to_station()[city])
    tooltip.attr("transform",`translate(${position.x},${position.y - 10})`)

    let bBox = tooltip.select("text").node().getBBox();
    tooltip
        .select("rect")
        .attr("x",-bBox.width/2 - 3)
        .attr("y",-bBox.height - 3)
        .attr("width",bBox.width + 6)
        .attr("height", bBox.height + 6)
        .attr("fill", "white")

    tooltip.attr("opacity",`1`)
}

export function remove_city() {
    d3.select("#tooltip").attr("opacity",`0`)
}
