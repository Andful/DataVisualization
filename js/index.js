import * as d3 from "d3";
import './fullscreen.js';

function get_bounding_box(geojson,projection) {
    let result = {}
    let points = geojson
    .features
    .flatMap(x => x
        .geometry
        .coordinates
        .flatMap(y => y[0]
            .map(k => projection(k))
        )
    )
    result.x = Math.min(...(points).map(x => x[0]))
    result.y = Math.min(...(points).map(x => x[1]))
    result.width = Math.max(...(points).map(x => x[0])) - result.x
    result.height = Math.max(...(points).map(x => x[1])) - result.y
    result.center = {x:result.x + result.width/2, y:result.y + result.height/2}
    return result;
}

async function draw()
{
    let svg = d3.select("svg");
    let width = svg.node().clientWidth;
    let height = svg.node().clientHeight;

    let [netherlands, stations, rails, trains] = await Promise.all([
        d3.json("data/the-netherlands.geojson"),
        d3.json("data/stations.json"),
        d3.json("data/rails.json"),
        d3.json("data/trains.json")
    ]);

    let projection = d3.geoMercator()

    let bbox = get_bounding_box(netherlands,projection)
    projection = projection
    .center(projection.invert([bbox.center.x,bbox.center.y]))
    .translate([width/2, height/2])
    .scale(projection.scale()*Math.min(width/bbox.width, height/bbox.height))

        //.scale(100)                       // This is like the zoom
        //.translate([ width/2, height/2 ])

    const zoom = d3.zoom()
      .scaleExtent([1, 10])
      .translateExtent([[0,0],[width,height]])
      .on('zoom', zoomed);

    const station_radius = 1
    const train_radius = 1

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
        .data(stations.payload)
        .enter()
        .append("circle")
        .attr("class","station")
        .attr("fill", "red")
        .attr("cx", d => projection([d.lng, d.lat])[0])
        .attr("cy", d => projection([d.lng, d.lat])[1])
        .attr("r",`${Math.exp(station_radius)}px`)
        .style("stroke", "none")
        .on("mouseover", showTooltip)
        .on("mousemove", showTooltip)
        .on("mouseout", removeTooltip);

    let tooltip = svg
        .append("g")
        .attr("opacity",1)

    tooltip.append("rect")
    tooltip
        .append("text")
        .attr("x",20)
        .attr("y",0)
        .style("pointer-events", "none")

    function showTooltip(d,i) {
        let transform = this.getCTM();
        var position = svg.node().createSVGPoint();
        let selected = d3.select(this)
        position.x = selected.attr("cx");
        position.y = selected.attr("cy");
        position = position.matrixTransform(transform);
        tooltip.attr("transform",`translate(${position.x},${position.y})`)
        tooltip.select('text').text(d.namen.lang)
        tooltip.attr("opacity",1)
    }

    function removeTooltip(d,i) {
        tooltip.attr("opacity",0)
    }


    /*map
        .selectAll(".train")
        .data(trains.payload.treinen.map(d => projection([d.lng, d.lat])))
        .enter()
        .append("circle")
        .attr("class","train")
        .attr("fill", "blue")
        .attr("cx", d => d[0])
        .attr("cy", d => d[1])
        .attr("r",`${train_radius}px`)
        .style("stroke", "none")*/

    function zoomed() {
        map.attr('transform', d3.event.transform);

        /*map
            .selectAll(".train")
            .attr("r", `${Math.exp(train_radius/d3.event.transform.k)}px`)*/
        map
            .selectAll(".station")
            .attr("r", `${Math.exp(station_radius/d3.event.transform.k)}px`)
    }
}

draw();
