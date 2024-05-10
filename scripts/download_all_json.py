import json
import time

import requests
from lib.utils import CDRAGON_URL, DATA_DIR
from tqdm import tqdm

CDRAGON_DIR = DATA_DIR / "cdragon"

LINKS_URL = CDRAGON_URL / "cdragon/files.exported.txt"
LINKS_FILE = CDRAGON_DIR / LINKS_URL.name
LINKS_FILE.parent.mkdir(parents=True, exist_ok=True)

if LINKS_FILE.exists():
    data = LINKS_FILE.read_text()
else:
    print("Fetching links")

    data = requests.get(str(LINKS_URL)).text
    with open(LINKS_FILE, "w+") as file:
        file.write(data)

lines = [l.strip() for l in data.split("\n") if len(l) > 0]
paths = [l for l in lines if l.endswith(".json")]

pbar = tqdm(paths)
for p in pbar:
    pbar.set_description(p)

    # if "tft" not in p or "skin" in p:
    #     continue

    url = CDRAGON_URL / p

    fp_out = CDRAGON_DIR / p
    fp_out.parent.mkdir(parents=True, exist_ok=True)
    if fp_out.exists():
        continue

    # print("Downloading", url)
    resp = requests.get(str(url))
    if resp.status_code != 200:
        print("Download failed", resp.status_code)
        continue

    data = resp.json()
    with open(fp_out, "w+") as file:
        json.dump(data, file, indent=4)

    time.sleep(0.25)
