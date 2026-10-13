# Wasm rust Chinese Writer
This a downsized version of my wasm rust chinese writer Wasmzidian.
It focusses only on writing short text, and is meant to be used on phones or equipment with small screens.
Just as in Wasmzidian, the 'dictionary' is no longer a sqlite file but a rust Vec (derived from the sqlite file by a small ad hoc rust utility and stored in data.rs).

With some 3500 characters, the dictionary contains all characters in the HSK levels 1 to 6, plus many more. It keeps being enriched.

Watch live at [https://www.eludev.fr/wazi](https://www.eludev.fr/wazi)
