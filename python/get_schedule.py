import requests
import os
import json
import urllib.request
from datetime import date
from datetime import timedelta
from bs4 import BeautifulSoup

example_url = "https://www.rijdendetreinen.nl/en/train-archive/2019-11-30/maastricht"

url = "https://www.rijdendetreinen.nl/en/train-archive/%s/%s"

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


def get_data(station_name : str, d : date):
    print("loading date %s station %s" % (d.isoformat(),station_name))
    station = station_name \
        .lower() \
        .replace("ë","e") \
        .replace("â","a") \
        .replace(".","") \
        .replace(" ","-") \
        .replace("/","-") \
        .replace("'","")
    d = d.isoformat()

    response = requests.get(url % (d,station))
    soup = BeautifulSoup(response.text, "html.parser")
    services = soup.find_all(class_='service')

    for service in services:
        divs = service.findChildren("div" , recursive=False)
        arrival_time = divs[0].text.replace(" ","").replace("\n","")
        arrival_time, arrival_delay = get_delay(arrival_time)
        departure_time = divs[1].text.replace(" ","").replace("\n","")
        departure_time, departure_delay  = get_delay(departure_time)
        arrival_station = divs[2].text.strip()
        train_type =  divs[3].text.strip()
        train_code =  divs[4].text.strip()
        platform = divs[5].text.strip()
        platform = None if platform == "—" else platform
        cancelled = True if platform is None else False
        #print("%s-%s-%s-%s-%s-%s-%s-%s-%s" % (arrival_time,arrival_delay,departure_time,departure_delay,arrival_station,train_type,train_code,platform,cancelled))
        yield {"arrival_time": arrival_time,
                "arrival_delay": arrival_delay,
                "departure_time":departure_time,
                "departure_delay":departure_delay,
                "arrival_station":arrival_station,
                "train_type": train_type,
                "train_code": train_code,
                "platform": platform,
                "cancelled": cancelled}

if __name__ == "__main__":
    d = date.today() - timedelta(days=1)
    with open(os.path.join(home_dir, 'static/data/stations.json'),'r') as infile:
        data = json.loads(infile.read())

    out = [{"station":station['namen']['lang'], "timetable":[e for e in get_data(station['namen']['lang'], d)]} for station in data["payload"]]

    with open(os.path.join(home_dir, 'static/data/timetable.json'),'w') as outfile:
        json.dump(out, outfile)
