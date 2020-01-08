import json
import numpy as np

with open("static/data/timetable.json") as timetable:
    timetable = json.loads(timetable.read())

lines = {}

for e in timetable:
    if e["line"] not in lines:
        lines[e["line"]] = []

    lines[e["line"]].append(e)

latest_arrival = [l[-1] for _,l in lines.items()]

latest_arrival = sorted(latest_arrival, key=lambda x:x["arrival_time"].split(":"))
