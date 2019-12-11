import json
import numpy as np

with open("static/data/timetable.json") as timetable:
    timetable = json.loads(timetable.read())

timetable[0]["timetable"][0].keys()


timetable[0]["timetable"][0]

set([e[] for e in timetable for t in e["timetable"] if t["train_code"]=='30906'])

station1 = list(e['station'] for e in timetable)
station2 = list(set(f['arrival_station'] for e in timetable for f in e['timetable']))

station1 = sorted(station1)
station2 = sorted(station2)

with open("station1.csv","w") as out:
    for e in station1:
        out.write(e + "\n")

with open("station2.csv","w") as out:
    for e in station2:
        out.write(e + "\n")

len(station1)
len(station2)

station1 = set(station1)
station2 = set(station2)
station2.difference(station1)
len(station2)

import os

home_dir = os.popen('git rev-parse --show-toplevel').read().strip()
with open(os.path.join(home_dir, 'static/data/stations.json'),'r') as infile:
    data = json.loads(infile.read())

data['payload'][0]['code']
