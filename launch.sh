send() {
    echo "$2: $1"
    screen -S "$2" -X stuff "$1^M"
}

SNAME=tft
PORT=4000

# Compile rust to wasm
cd core
bash build.sh
cd ..

# Download assets / data from CDragon
cd scripts
if [ ! -d ./venv ]; then
  python3 -m venv venv
fi
./venv/bin/python -m pip install -r requirements.txt
./venv/bin/python download_misc_icons.py
./venv/bin/python download_traits.py
./venv/bin/python download_champions.py
cd ..

# Build web gui
cd gui
npm install
npm run build

# Run web gui
screen -dmS $SNAME
send "cd build" $SNAME
send "PORT=$PORT node index.js" $SNAME
