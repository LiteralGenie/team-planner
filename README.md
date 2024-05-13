### Building

Make sure [Rust](https://www.rust-lang.org/tools/install), [Node.js](https://github.com/nvm-sh/nvm), and [Python 3.10+](https://launchpad.net/~deadsnakes/+archive/ubuntu/ppa) are installed.

Then run 
```
bash launch.sh
```

After downloading any neccessary dependencies / assets (which may a take a few minutes), the app will be running at http://localhost:4000.

### Development

Make sure [Rust](https://www.rust-lang.org/tools/install), [Node.js](https://github.com/nvm-sh/nvm), and [Python 3.10+](https://launchpad.net/~deadsnakes/+archive/ubuntu/ppa) are installed.

Then download the TFT metadata / assets from [CDragon](https://www.communitydragon.org/).
```bash
cd scripts
./venv/bin/python -m pip install -r requirements.txt
./venv/bin/python download_misc_icons.py
./venv/bin/python download_traits.py
./venv/bin/python download_champions.py
cd ..
```

Then compile the Rust code to a WASM module.
```bash
cd core
bash build.sh
cd ..
```

And finally, run the web gui.
```bash
cd gui
npm install
npm run dev
```
