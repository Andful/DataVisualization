import os
import json

import http.client, urllib.request, urllib.parse, urllib.error, base64

key = '874ef5c14e9c4d809dde57b1d0e3e1b9'

home_dir = os.popen('git rev-parse --show-toplevel').read().strip()

headers = {
    # Request headers
    'Ocp-Apim-Subscription-Key': '%s' % (key,),
}

try:
    conn = http.client.HTTPSConnection('gateway.apiportal.ns.nl')
    conn.request("GET", "/reisinformatie-api/api/v2/stations", "", headers)
    response = conn.getresponse()
    data = response.read().decode("utf-8")
    data = json.loads(data)
    conn.close()
    data["payload"] = [station for station in data["payload"]
        if station['land'] == 'NL']

    for station in data["payload"][:]:
        station["code"] = station["code"].lower()
        '''if station['namen']['lang'] == "Delft Zuid (t/m 14 dec)":
            print("changed Delft Zuid (t/m 14 dec) to Delft Zuid")
            station['namen']['lang'] = "Delft Zuid"
        if station['namen']['lang'] == "Delft Campus (vanaf 15 dec)":
            print("removed Delft Campus (vanaf 15 dec)")
            data["payload"].remove(station)
        if station['namen']['lang'] == "Zwolle Stadshagen na 14 dec":
            print("removed Zwolle Stadshagen na 14 dec")
            data["payload"].remove(station)
        if station['namen']['lang'] == "Haren OV Transferium":
            data["payload"].remove(station)'''

    with open(os.path.join(home_dir, 'static/data/stations.json'), 'w') as outfile:
        json.dump(data, outfile)
except Exception as e:
    print(e)

print(home_dir)
with open(os.path.join(home_dir, 'static/data/stations.json'), 'r') as infile:
    data = infile.read()
    data = json.loads(data)
    station_codes = set(e["code"].lower() for e in data["payload"])

try:
    conn = http.client.HTTPSConnection('gateway.apiportal.ns.nl')
    conn.request("GET", "/Spoorkaart-API/api/v1/spoorkaart", "", headers)
    response = conn.getresponse()
    data = response.read().decode("utf-8")
    data = json.loads(data)
    conn.close()
    data["payload"]["features"] = [
        e for e in data["payload"]["features"]
            if e['properties']['to'] in station_codes and
            e['properties']['from'] in station_codes
    ]
    with open(os.path.join(home_dir, 'static/data/rails.json'), 'w') as outfile:
        json.dump(data, outfile)
except Exception as e:
    print(e)
