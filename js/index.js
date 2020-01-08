import * as d3 from "d3";
import './fullscreen.js';
import WorkerInterface from "./WorkerInterface.js"
import {show_suggestion, removed_focus} from "./suggestion.js"
import {add_to_plot, reset_plot, reset_modification_plot} from "./plot_times.js"
import {set_code_to_station, set_path_to_link} from "./global.js"
import {show_city,remove_city} from "./show_city.js"

let worker = new WorkerInterface("./worker.js");

let svg = d3.select("#map")

async function draw() {
    //get data
    const [pairs, stations] = await Promise.all([d3.json("data/pairs.json"),d3.json("data/stations.json")]);
    let map = await d3.xml("svg/railway_map.svg");
    let svg = d3.select("#map")
    let main = d3.select("#main");
    let width = svg.node().clientWidth;
    let height = svg.node().clientHeight;

    main.append("g").node().outerHTML = map.getElementById("network").outerHTML
    main.append("g").node().outerHTML = map.getElementById("contour").outerHTML

    const mar = 30
    const zoom = d3.zoom()
    .translateExtent([
        [-250, 0],
        [1000, 800]
    ])
    .scaleExtent([0.5, 10])
    .on('zoom', zoomed);


    svg.call(zoom);


    function zoomed() {
        d3.select("#main").attr("transform", d3.event.transform);
    }

    d3.select("#departure-station")
    .on("focus", show_suggestion(stations))
    .on("input", show_suggestion(stations))
    .on("focusout", removed_focus(stations));
    d3.select("#arrival-station")
    .on("focus", show_suggestion(stations))
    .on("input", show_suggestion(stations))
    .on("focusout", removed_focus(stations));

    let code_to_station = {};
    stations.payload.forEach(d => code_to_station[d.code] = d.namen.lang);
    set_code_to_station(code_to_station)
    worker.generate_links({"pairs":JSON.stringify(pairs)},function(data) {
        console.log("data:",data)
        set_path_to_link(data);
    })

    d3.select("#compute_time")
    .on("click", function() {
        d3.event.preventDefault()
        reset_plot();
        let from = d3.select("#departure-station").attr("data-selection")
        let to = d3.select("#arrival-station").attr("data-selection")
        if (from && to) {
            worker.compute_paths({"from": from, "to": to,"day":"0","modified":"false"},data => add_to_plot(data, false))
        } else {
            console.error("no selection")
        }
    })

    d3.select("#compute_modification")
    .on("click", function() {
        d3.event.preventDefault()
        reset_modification_plot();
        let from = d3.select("#departure-station").attr("data-selection")
        let to = d3.select("#arrival-station").attr("data-selection")
        if (from && to) {
            worker.compute_paths({"from": from, "to": to,"day":"0","modified":"true"},data => add_to_plot(data, true))
        } else {
            console.error("no selection")
        }
    })

    d3.select("#reset")
    .on("click", function() {
        d3.event.preventDefault();
        d3.selectAll(".removed").classed("removed",false);
    })

    d3.selectAll(".station")
    .on("click", function() {
        if (d3.select(this).classed("removed")) {
            d3.select(this).classed("removed",false);
            worker.remove_station({"station": this.id},() => {})
        } else {
            d3.select(this).classed("removed",true);
            worker.remove_station({"station": this.id},() => {})
        }
    })
    .on("mouseover", function(){show_city(this.id)})
    .on("mouseout", function(){remove_city()})

    d3.selectAll(".rails")
    .on("click", function() {
        if (d3.select(this).classed("removed")) {
            d3.select(this).classed("removed",false);
            worker.add_link({"link": this.id},() => {})
        } else {
            d3.select(this).classed("removed",true);
            worker.remove_link({"link": this.id},() => {})
        }
    })
}

draw();
