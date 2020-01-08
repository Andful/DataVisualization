import * as d3 from "d3";

//highlight station when a suggestion is hovered
function highlight_station(text_box, code) {
    if (text_box.id == "departure-station") {
        d3.selectAll(".departure-station-point").classed("departure-station-point", false)
        d3.select(`#${code}`).classed("departure-station-point", true)
    } else if (text_box.id == "arrival-station") {
        d3.selectAll(".arrival-station-point").classed("arrival-station-point", false)
        d3.select(`#${code}`).classed("arrival-station-point", true)
    }
}

//select the suggestion
function selected_suggestion(text_box) {
    return function(d) {
        text_box.value = d.namen.lang;
        d3.select(text_box)
            .attr("data-selection", d.code)
            .classed("error_input",false);
        d3.selectAll(".suggestion-container")
            .remove();
        highlight_station(text_box, d.code);
    }
}


export function removed_focus(stations) {
    return function() {
        d3.selectAll(".suggestion-container").remove();
        let input = this.value.toLowerCase().trim();
        let valids = stations
            .payload
            .filter(d => d.namen.lang.toLowerCase()==input)

        if (valids.length > 0) {
            this.value = valids[0].namen.lang
            d3.select(this).attr("data-selection", valids[0].code);

        } else {
            let elem = d3.select(this)
                .attr("data-selection", null)
            elem.classed("error_input",this.value != "");
        }
    }
}

//show the suggestions
export function show_suggestion(stations) {
    return function() {
        let bbox = this.getBoundingClientRect();
        let input = this.value.toLowerCase();
        let station_names = stations
            .payload
            .filter(d => d.namen.lang.toLowerCase().includes(input))
        let text_box = this;
        d3.select(".suggestion-container").remove();
        d3.select("body")
        .append("div")
        .attr("class","suggestion-container")
        .attr("style",`
            width:${bbox.width}px;
            top:${bbox.top}px;
            left:${bbox.right}px;
        `)
        .selectAll(".suggestion")
        .data(station_names)
        .enter()
        .append("div")
        .attr("class","suggestion")
        .html(d => {
            let s = d.namen.lang
            let start = s.toLowerCase().indexOf(input);
            return `
                ${s.slice(0,start)}<strong>${s.slice(start,start+input.length)}</strong>${s.slice(start+input.length)}
            `;
        })
        .on("mousedown",selected_suggestion(text_box))
        .on("mouseenter", d => {
            highlight_station(text_box, d.code);
        })
        .on("mouseleave", d => {
            if (text_box.id == "departure-station") {
                d3.select(`#${d.code}`).classed("departure-station-point", false)
            } else if (text_box.id == "arrival-station") {
                d3.select(`#${d.code}`).classed("arrival-station-point", false)
            }
        })
    }
}
