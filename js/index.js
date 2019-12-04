import * as d3 from "d3";

let svg = d3.select("svg");
let width = +svg.attr("width");
let height = +svg.attr("height");

// Map and projection
let projection = d3.geoMercator()
    .center([5.2913, 52.1326])                // GPS of location to zoom on
    .scale(4000)                       // This is like the zoom
    .translate([ width/2, height/2 ])

async function draw()
{
    let netherlands = await d3.json("data/the-netherlands.geojson");
    let stations = await d3.json("data/stations.json");
    let rails = await d3.json("data/rails.json");
    let trains = await d3.json("data/trains.json");

    const station_radius = 3
    const train_radius = 3

    const zoom = d3.zoom()
      .scaleExtent([1, 8])
      .on('zoom', zoomed);

    svg.call(zoom);

    let map = svg.append("g")

    map
        .selectAll(".country")
        .data(netherlands.features)
        .enter()
        .append("path")
        .attr("class","country")
        .attr("fill", "grey")
        .attr("d", d3.geoPath()
            .projection(projection)
        )
        .style("stroke", "none");

    map
        .selectAll(".rail")
        .data(rails.payload.features)
        .enter()
        .append("path")
        .attr("class","rail")
        .attr("fill", "none")
        .attr("d", d3.geoPath()
            .projection(projection)
        )
        .attr("vector-effect","non-scaling-stroke")
        .style("stroke", "black")


    map
        .selectAll(".station")
        .data(stations.payload.map(d => projection([d.lng, d.lat])))
        .enter()
        .append("circle")
        .attr("class","station")
        .attr("fill", "red")
        .attr("cx", d => d[0])
        .attr("cy", d => d[1])
        .attr("r",`${station_radius}px`)
        .style("stroke", "none")

    map
        .selectAll(".train")
        .data(trains.payload.treinen.map(d => projection([d.lng, d.lat])))
        .enter()
        .append("circle")
        .attr("class","train")
        .attr("fill", "blue")
        .attr("cx", d => d[0])
        .attr("cy", d => d[1])
        .attr("r",`${train_radius}px`)
        .style("stroke", "none")

    navigator.geolocation.getCurrentPosition(showPosition, errorHandler);
    function errorHandler(err) {
       if(err.code == 1) {
          alert("Error: Access is denied!");
       } else if( err.code == 2) {
          alert("Error: Position is unavailable!");
       }
    }
    function showPosition(position) {
        let xy = projection([position.coords.longitude, position.coords.latitude])
        map
            .append("circle")
            .attr("id","your_location")
            .attr("fill", "green")
            .attr("cx", d => xy[0])
            .attr("cy", d => xy[1])
            .attr("r",`${train_radius}px`)
            .style("stroke", "none")
    }

    function zoomed() {
        map.attr('transform', d3.event.transform);

        map
            .selectAll(".train")
            .attr("r", `${train_radius/d3.event.transform.k}px`)
        map
            .selectAll(".station")
            .attr("r", `${station_radius/d3.event.transform.k}px`)
    }
}

draw();
