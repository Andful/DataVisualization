from bs4 import BeautifulSoup
import json
import os
import json

home_dir = os.popen('git rev-parse --show-toplevel').read().strip()

with open(os.path.join(home_dir, "static/svg/railway_map.svg"),"r") as file:
    soup = BeautifulSoup(file.read(), 'lxml')
    network = soup.find("g", {"id": "network"})

pairs = [path["id"].split("-") for path in network.findAll("path")]
s = json.dumps(pairs)
with open(os.path.join(home_dir, "static/data/pairs.json"),"w") as file:
    file.write(s)
