# ExpressionParser

## elixir

```sh
cd elixir

mix test
```

## web assembly

```sh
cd www/

wasm-pack build

npm install 

npm run start
```

## python

```sh

cd python

python3 -m venv env

# activate virtualenv
# windows: .\env\Scripts\activate.bat
# linux/osx source env/bin/activate

python -m pip install maturin

maturin develop

python example.py

# for tests
python -m pip install -r test_requirements.txt

pytest

```