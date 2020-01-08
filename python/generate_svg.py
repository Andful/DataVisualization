from bs4 import BeautifulSoup
import json
import os
import json

home_dir = os.popen('git rev-parse --show-toplevel').read().strip()

with open(os.path.join(home_dir, "static/data/stations.json"),"r") as file:
    json_data = json.loads(file.read())
    json_codes = set(e["code"] for e in json_data["payload"])


with open(os.path.join(home_dir, "static/svg/index3.html"),"r") as file:
    soup = BeautifulSoup(file.read(), 'html.parser')
    connection = soup.find("pre", {"id": "connection_data"})
    for i, e in enumerate(soup.findAll("circle", {"class": "curve"})):
        new_id = "joint%d" % (i,)
        old_id = e["id"]
        e["id"] = new_id
        e["class"] = "joint"
        connection.string = connection.text.replace("\""+old_id+"\"","\""+new_id+"\"")

data = json.loads(connection.text)

with open(os.path.join(home_dir, "static/svg/index.html"), "w") as file:
    file.write(str(soup))

pairs = []

for l in data:
    for (a,b) in zip(l,l[1:]):
        pairs.append([a,b])

pairs

adjacency_list = {}
nodes = set()

for e1,e2 in pairs:
    nodes.add(e1)
    nodes.add(e2)
    newAdj = adjacency_list.get(e2,[])
    newAdj.append(e1)
    adjacency_list[e2] = newAdj

    newAdj = adjacency_list.get(e1,[])
    newAdj.append(e2)
    adjacency_list[e1] = newAdj

adjacency_list

for node in nodes:
    if node.startswith("joint") and len(adjacency_list[node]) == 2:
        print(node,":",adjacency_list[node])
        l1,l2 = (e for e in pairs if node in e)
        print(l1,l2)
        pairs.remove(l1)
        pairs.remove(l2)
        if l1[0] != node:
            l1.reverse()
        if l2[-1] != node:
            l2.reverse()
        l1.remove(node)
        pairs.append(l2 + l1)

pairs

m = set()
for p in pairs:
    m.add(p[0])
    m.add(p[-1])

soupSvg = BeautifulSoup('<svg version="1.1" id="second" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px" viewBox="0 0 612 792"></svg>', 'xml')
svg = soupSvg.svg
g = soupSvg.new_tag("g", attrs={'id':'main','transform':"scale(0.5) translate(200,300)"})
svg.append(g)
joint_id = 0
joint_map = {}
for e in m:
    clone_element = soup.find("circle", {"id": e})
    if e.startswith("joint"):
        clazz = "joint"
        id = 'joint%d' % (joint_id,)
        g.append(soupSvg.new_tag("circle",attrs={'id':id, 'class':"joint", 'cx':clone_element['cx'], 'cy':clone_element['cy'], 'r':3}))
        joint_map[e] = id
        joint_id += 1
    else:
        g.append(soupSvg.new_tag("circle",attrs={'id':e, 'class':'station', 'cx':clone_element['cx'], 'cy':clone_element['cy'], 'r':3}))

for p in pairs:
    d = ""
    for i,(e1, e2) in enumerate(zip(p, p[1:])):
        clone_element1 = soup.find("circle", {"id": e1})
        clone_element2 = soup.find("circle", {"id": e2})
        if i==0:
            d += "M%s,%s " % (clone_element1["cx"], clone_element1["cy"])

        if clone_element1["cx"] == clone_element2["cx"]:
            d += "V%s " % (clone_element2["cy"],)
        elif clone_element1["cy"] == clone_element2["cy"]:
            d += "H%s " % (clone_element2["cx"],)
        else:
            d += "L%s,%s " % (clone_element2["cx"],clone_element2["cy"])
    d = d[:-1]
    start = p[0]
    end = p[-1]
    if start.startswith("joint"):
        start = joint_map[start]
    if end.startswith("joint"):
        end = joint_map[end]
    g.append(soupSvg.new_tag("path", attrs={'id':'%s-%s' % tuple(sorted([start,end])), 'class':'rails', 'd':d, 'style':"stroke-width:3;stroke:#660000; fill:none;"}))
print(svg)
