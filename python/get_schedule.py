import requests
import os
import json
import urllib.request
from datetime import date
from datetime import timedelta
from datetime import time
from bs4 import BeautifulSoup
from multiprocessing import Pool

example_url = "900524"

url = "https://www.rijdendetreinen.nl/en/train-archive/%s/%d"

home_dir = os.popen('git rev-parse --show-toplevel').read().strip()

fractions = {
    "½": 1/2,
    "⅓": 1/3,
    "⅔": 2/3,
    "¼": 1/4,
    "¾": 3/4,
    "⅕": 1/5,
    "⅖": 2/5,
    "⅗": 3/5,
    "⅘": 4/5,
    "⅙": 1/6,
    "⅚": 5/6,
    "⅐": 1/7,
    "⅜": 3/8,
    "⅝": 5/8,
    "⅞": 7/8,
    "⅑": 1/9,
    "⅒": 1/10
}

def get_delay(time):
    if time == "—":
        return None, 0

    if "+" in time:
        time, delayStr = time.split("+")
        delay = 0
        for i in range(len(delayStr)):
            if "0" <= delayStr[i] <= "9":
                delay = 10*delay + int(delayStr[i])
            else:
                delay += fractions[delayStr[i]]

        return time, delay
    else:
        return time, 0

def station_to_url(station):
    return station \
        .lower() \
        .replace("ë","e") \
        .replace("â","a") \
        .replace(" ","-") \
        .replace("/","-") \
        .replace(".","") \
        .replace("'","")

def get_time_and_delay(div):
    text = div.text
    text = text.strip()
    if text == "—":
        return None, 0
    if "+" in text:
        t, delayStr = text.split("+")
        delayStr = delayStr.strip()
        delay = 0
        for i in range(len(delayStr)):
            if "0" <= delayStr[i] <= "9":
                delay = 10*delay + int(delayStr[i])
            else:
                delay += fractions[delayStr[i]]
        return t.strip(), delay
    else:
        return text.strip(), 0

#cached = set();
def get_timetable_data(ride_number : int, d : date, station_map):
    #if ride_number not in cached:
    #    cached.add(ride_number)
    #else:
    #    return

    print("loading date %s ride %d" % (d,ride_number))

    response = requests.get(url % (d,ride_number))
    soup = BeautifulSoup(response.text, "html.parser")
    services = soup.find_all(class_='service')
    img = None
    schedule = []

    for service in services:
        divs = service.findChildren("div" , recursive=False)
        arrival_time, _ = get_time_and_delay(divs[0])
        departure_time, _  = get_time_and_delay(divs[1])
        station = station_map.get(divs[2].text.strip())
        print(divs[2].text.strip())
        print(station)
        platform =  divs[3].text.strip()
        if img is None:
            img = divs[4].find("img")
            if img is not None:
                img = img["src"]

        if len(divs) > 5:
            data = divs[5].text.split()
            on_time = data[0]
            cancelled = data[3]
        else:
            on_time = None
            cancelled = None

        schedule.append({
                "arrival_time": arrival_time,
                "departure_time":departure_time,
                "station":station,
                "platform": platform,
                "on_time": on_time,
                "cancelled": cancelled
                })
    for d,a in zip(schedule, schedule[1:]):
        if d["departure_time"] is not None and a["arrival_time"] is not None and \
        d["station"] is not None and a["station"] is not None:
            yield {
                "departure_time": d["departure_time"],
                "arrival_time": a["arrival_time"],
                "departure_station": d["station"],
                "arrival_station": a["station"],
                "line" : ride_number,
                "img" : img
            }

if __name__ == "__main__":
    week   = [
        'Saturday',
        'Sunday',
        'Monday',
        'Tuesday',
        'Wednesday',
        'Thursday',
        'Friday',
        ]
    with open(os.path.join(home_dir, 'static/data/stations.json'),"r") as stations:
        stations = json.loads(stations.read())
        station_to_code = {station["namen"]["lang"]:station["code"]  for station in stations["payload"]}

    with open(os.path.join(home_dir, 'static/data/rides.json'),"r") as rides:
        rides = json.loads(rides.read())

    #rides = {"Monday": rides["Monday"][:10]}

    data = list()
    for day,rides_in_the_day in rides.items():
        new_data = list()
        for ride in rides_in_the_day:
            for e in get_timetable_data(ride,date(day=14,month=12,year=2019) + timedelta(days=week.index(day)),station_to_code):
                new_data.append(e)
        data = data + new_data
    #data = sorted(data, x: lambda:x[''])
    with open(os.path.join(home_dir, 'static/data/timetable.json'),'w') as outfile:
        json.dump(data, outfile)

for e in get_timetable_data(1132,date(day=16,month=12,year=2019),station_to_code):
    print(e)

print([e for e,_ in rides.items()])
