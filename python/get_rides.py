import requests
import os
import json
import urllib.request
from datetime import date
from datetime import timedelta
from datetime import time
from bs4 import BeautifulSoup

home_dir = os.popen('git rev-parse --show-toplevel').read().strip()
url = "https://www.rijdendetreinen.nl/en/train-archive/%s/%s"

def station_to_url(station):
    return station \
        .lower() \
        .replace("ë","e") \
        .replace("â","a") \
        .replace(" ","-") \
        .replace("/","-") \
        .replace(".","") \
        .replace("'","")

def get_ride_data(station_name : str, d : date):
    print("loading date %s station %s" % (d.isoformat(),station_name))
    station = station_to_url(station_name)
    d = d.isoformat()

    response = requests.get(url % (d,station))
    soup = BeautifulSoup(response.text, "html.parser")
    services = soup.find_all(class_='service')

    for service in services:
        divs = service.findChildren("div" , recursive=False)
        yield int(divs[4].text.strip())

def get_rides(day):
    with open(os.path.join(home_dir, 'static/data/stations.json'),'r') as infile:
        data = json.loads(infile.read())

    trips = set(e for station in data["payload"] for e in get_ride_data(station['namen']['lang'], day))
    return sorted(list(trips))

def get_rides_in_week():
    week   = [
        'Monday',
        'Tuesday',
        'Wednesday',
        'Thursday',
        'Friday',
        'Saturday',
        'Sunday',
        ]

    weekdays = [date(day=14,month=12,year=2019) + timedelta(days=i) for i in range(7)]
    return {week[day.weekday()]: get_rides(day) for day in weekdays}

with open(os.path.join(home_dir, 'static/data/rides.json'),'w') as outfile:
    json.dump(get_rides_in_week(), outfile)

with open(os.path.join(home_dir, 'static/data/rides.json')) as file:
    a = json.loads(file.read())
    for key, item in a.items():
        for i in range(len(item)):
            try:
                item[i] = int(item[i])
            except:
                print(key[i])

        item.sort()
    with open(os.path.join(home_dir, 'static/data/rides2.json'),'w') as out:
        json.dump(a, out)
