import * as d3 from "d3"
import {get_code_to_station, get_path_to_link} from "./global.js"
import {leadZeros,array_to_string} from "./util.js"

const svg = d3.select("#trip-times");
const margin = {top: 40, right: 40, bottom: 30, left: 65};
let width = svg.node().clientWidth - margin.left - margin.right;
let height = 700 - margin.top - margin.bottom;
const barWidth = 4;
let main = svg
    .append("g")
    .attr("transform", `translate(${margin.left},${margin.top})`);

let scroll_point = -10;

let scroll = main.append("g")
    .attr("transform",`translate(0,${-scroll_point})`);

svg.on("wheel", function() {
        scroll_point -= d3.event.deltaY;
        scroll_point = Math.max(-10, scroll_point);
        //scroll_point = Math.min(scroll_point, svg.node().clientHeight - height)
        scroll.attr("transform",`translate(0,${-scroll_point})`)
    })

let delay_bars = scroll.append("g");
let modified_bars = scroll.append("g");
let bars = scroll.append("g");

let maxDuration = 0;
let xScale = d3
    .scaleLinear()
    .range([0, width])
    .domain([0, maxDuration]);

let cover_up = main
    .append("rect")
    .attr("x",-margin.left)
    .attr("y",-margin.top)
    .attr("width",width + margin.top + margin.bottom)
    .attr("height",margin.top)
    .attr("fill","white")

let yScale = d3
    .scaleLinear()
    .range([0, 3*height])
    .domain([0,24]);

let yAxis = d3.axisLeft(yScale)
    .ticks(24 * 2)
    .tickFormat(function (d) {
        let hours = Math.floor(d) + ""
        let minutes = Math.round(60*(d%1)) + "";
        if (hours.length < 2) {
            hours = "0" + hours
        }
        if (minutes.length < 2) {
            minutes = "0" + minutes
        }
        return `${hours}:${minutes}`;
    });

let xAxis = d3.axisTop(xScale);

let y_label = main
    .append("text")
    .text("departure time")
    .attr("transform",`translate(${-margin.left + 15},${(svg.node().getBoundingClientRect().height - margin.top)/2}) rotate(-90)`)
    .attr("style","text-anchor:middle")

let x_label = main
    .append("text")
    .text("trip duration[hours]")
    .attr("transform",`translate(${width/2},-20)`)
    .attr("style","text-anchor:middle")

scroll
    .append("g")
    .attr("class", "yaxis")
    .call(yAxis);

main
    .append("g")
    .attr("class", "xaxis")
    .call(xAxis);

window.addEventListener('resize', function() {
    width = svg.node().clientWidth - margin.left - margin.right;
    xScale.range([0, width]);
    svg.select(".xaxis").call(xAxis);
    cover_up.attr("width",width + margin.top + margin.bottom)
    y_label.attr("transform",`translate(${-margin.left + 15},${(svg.node().getBoundingClientRect().height - margin.top)/2}) rotate(-90)`);
    x_label.attr("transform",`translate(${width/2},-20)`)
}, true);


d3
.select("#show-both")
.on("click",function() {
    bars.attr("opacity",1).attr("pointer-events","visiblePainted");
    modified_bars.attr("opacity",1).attr("pointer-events","visiblePainted");
    delay_bars.attr("opacity",1).attr("pointer-events","visiblePainted");
    x_label.text("trip duration[hours]")
    d3.selectAll(".bar.selected").remove()
    scroll.selectAll(".transition").remove();
})

d3
.select("#show-original")
.on("click",function() {
    bars.attr("opacity",1).attr("pointer-events","visiblePainted");
    modified_bars.attr("opacity",0).attr("pointer-events","none");
    delay_bars.attr("opacity",0).attr("pointer-events","none");
    x_label.text("trip duration[hours]")
    d3.selectAll(".bar.selected").remove()
    scroll.selectAll(".transition").remove();
})

d3
.select("#show-modified")
.on("click",function() {
    bars.attr("opacity",0).attr("pointer-events","none");
    modified_bars.attr("opacity",1).attr("pointer-events","visiblePainted");
    delay_bars.attr("opacity",0).attr("pointer-events","none");
    x_label.text("trip duration[hours]")
    d3.selectAll(".bar.selected").remove()
    scroll.selectAll(".transition").remove();
})

d3
.select("#show-delay")
.on("click",function() {
    bars.attr("opacity",0).attr("pointer-events","none");
    modified_bars.attr("opacity",0).attr("pointer-events","none");
    delay_bars.attr("opacity",1).attr("pointer-events","visiblePainted");
    x_label.text("delay[hours]")
    d3.selectAll(".bar.selected").remove()
    scroll.selectAll(".transition").remove();
})


function array_to_hours(e) {
    return 24 * e[0] + e[1] + e[2]/60;
}

function get_transitions(trip) {
    return trip
        .slice(0,-1)
        .filter((e,i) => trip[i].line != trip[i+1].line)
        .map(e => {return {"station": e.arrival_station, "time": e.arrival_time}});
}

function highlight_trip(trip) {
    d3.selectAll(".bar.selected").remove()
    d3.selectAll(".selected").classed("selected",false);
    d3.selectAll(".transition").remove();
    let to_clone = d3.select(this);
    scroll
    .append("rect")
    .attr("x",to_clone.attr("x"))
    .attr("y",to_clone.attr("y"))
    .attr("width",to_clone.attr("width"))
    .attr("height",to_clone.attr("height"))
    .attr("class",to_clone.attr("class"))
    .classed("selected", true)
    .on("mouseleave",unhighlight_trip);
    let transitions = get_transitions(trip);
    let element = d3.select(this);
    trip.forEach(function(d) {
        let path_to_link = get_path_to_link();
        let links = path_to_link[d.departure_station < d.arrival_station ? `${d.departure_station}-${d.arrival_station}` : `${d.arrival_station}-${d.departure_station}`]
        links.forEach((function(d) {d3.select(`#${d}`).classed("selected",true).classed("modified",element.classed("modified"))}));
    })
    transitions.forEach(function(d) {
        d3.select(`#${d.station}`).classed("selected",true);
    })
    transitions.unshift({"station": trip[0].departure_time, "time": trip[0].departure_time})
    transitions.push({"station": trip[trip.length-1].arrival_station, "time": trip[trip.length-1].arrival_time})
    scroll.selectAll(".transition")
    .data(transitions)
    .enter()
    .append("circle")
    .attr("class", "transition")
    .attr("cx",e => xScale(array_to_hours(e.time) - array_to_hours(trip[0].departure_time)))
    .attr("cy",e => yScale(array_to_hours(trip[0].departure_time)))
    .attr("r",4)
    .attr("fill",(d,i) => {if (i==0){return "red"} else if (i==transitions.length-1){return "green"} else {return "yellow"}})

    show_trip_detail(trip)
}

function unhighlight_trip(trip) {
    //d3.select(this).remove();
}

function show_trip_detail(trip) {
    let last_index = 0;
    let line = trip[0].line;
    let transitions = [];

    let code_to_station = get_code_to_station();

    transitions.push({"station": code_to_station[trip[0].departure_station], "arrival_time": "-", "departure_time": array_to_string(trip[0].departure_time)})
    let last_line = trip[0].line;
    trip.slice(1).forEach( (d,i) => {
        if (last_line != d.line) {
            transitions.push({"station": code_to_station[d.departure_station], "arrival_time": array_to_string(trip[i].arrival_time), "departure_time": array_to_string(d.departure_time)})
            last_line = d.line
        }
    })
    transitions.push({"station": code_to_station[trip[trip.length - 1].arrival_station], "arrival_time": array_to_string(trip[trip.length - 1].arrival_time), "departure_time": "-"})
    d3.select("#trip-data")
        .selectAll("tr")
        .remove()

    d3.select("#trip-data")
        .selectAll("tr")
        .data(transitions)
        .enter()
        .append("tr")
        .html(d => `
            <th>${d.station}</th>
            <th>${d.arrival_time}</th>
            <th>${d.departure_time}</th>
            <th class="table-ball">●</th>
            `)
}

let trips = [];
let modified_trips = [];
let trips_departure = {};
let delays = []

export function add_to_plot(trip,modified) {
    if (modified) {
        modified_trips.push(trip)
        if (trips_departure[array_to_hours(trip[0].departure_time)]){
            let delay = array_to_hours(trip[trip.length-1].arrival_time) - trips_departure[array_to_hours(trip[0].departure_time)]
            delays.push(delay)
        }

    } else {
        trips.push(trip);
        trips_departure[array_to_hours(trip[0].departure_time)] = array_to_hours(trip[trip.length-1].arrival_time);
    }
    let newMaxTripDuration = Math.max(...trip.map(d => array_to_hours(trip[trip.length - 1].arrival_time) - array_to_hours(trip[0].departure_time)));
    if (maxDuration < newMaxTripDuration) {
        maxDuration = newMaxTripDuration;
        xScale.domain([0, maxDuration]);
        svg.select(".xaxis")
                    .transition()
                    .duration(100)  // https://github.com/mbostock/d3/wiki/Transitions#wiki-d3_ease
                    .call(xAxis);
        //scroll.selectAll(".bar").remove()
    }

    let g = modified ? modified_bars : bars;
    g.selectAll(".bar")
        .data(modified ? modified_trips : trips)
        .enter()
        .append("rect")
        .attr("class","bar")
        .classed("modified",modified)
        .attr("x", () => xScale(0))
        .attr("y", d => yScale(array_to_hours(d[0].departure_time)) - barWidth/2)
        .attr("width", d => xScale(array_to_hours(d[d.length - 1].arrival_time) - array_to_hours(d[0].departure_time)))
        .attr("height", barWidth)
        .on("mouseenter", highlight_trip)
        //.on("mouseleave", unhighlight_trip)
        .on("click", show_trip_detail)

    scroll.selectAll(".bar")
        .data(modified_trips.concat(trips))
        .transition()
        .duration(100)
        .attr("x", () => xScale(0))
        .attr("y", d => yScale(array_to_hours(d[0].departure_time)) - barWidth/2)
        .attr("width", d => xScale(array_to_hours(d[d.length - 1].arrival_time) - array_to_hours(d[0].departure_time)))
        .attr("height", barWidth)

    if (modified) {
        delay_bars
        .selectAll(".delay")
        .data(delays)
        .transition()
        .duration(100)
        .attr("x", () => xScale(0))
        .attr("y", (d, i) => yScale(array_to_hours(modified_trips[i][0].departure_time)) - barWidth/2)
        .attr("width", d => xScale(d))
        .attr("height", barWidth)

        delay_bars
        .selectAll(".delay")
        .data(delays)
        .enter()
        .append("rect")
        .attr("class","delay")
        .attr("x", () => xScale(0))
        .attr("y", (d, i) => yScale(array_to_hours(modified_trips[i][0].departure_time)) - barWidth/2)
        .attr("width", d => xScale(d))
        .attr("height", barWidth)
    }

    if (trips.length == 1) {
        scroll_point = yScale(array_to_hours(trips[0][0].departure_time)) - barWidth/2
        console.log(`new scroll point ${scroll_point}`)
        scroll
        .transition()
        .duration(750)
        .attr("transform", `translate(0,${-scroll_point})`)
    }
}

export function reset_plot() {
    bars.selectAll("*").remove()
    modified_bars.selectAll("*").remove()
    delay_bars.selectAll("*").remove()

    trips = [];
    modified_trips = [];
    trips_departure = {};modified_trips = [];
    delays = []
    let maxDuration = 0;
}

export function reset_modification_plot() {
    modified_bars.selectAll("*").remove()
    delay_bars.selectAll("*").remove()

    modified_trips = [];
    delays = []
}
