from bs4 import BeautifulSoup
import json
import os

home_dir = os.popen('git rev-parse --show-toplevel').read().strip()

with open(os.path.join(home_dir, "static/data/stations.json"),"r") as file:
    json_data = json.loads(file.read())
    json_codes = set(e["code"] for e in json_data["payload"])

with open(os.path.join(home_dir, "static/svg/index3.html"),"r") as file:
    soup = BeautifulSoup(file.read(), 'html.parser')

    svg_codes = [e.get("id") for e in soup.find_all('circle')]
    assert(len(svg_codes) == len(set(svg_codes)))
    svg_codes = set(svg_codes)

json_data.keys()
json_codes - svg_codes
svg_codes - json_codes
