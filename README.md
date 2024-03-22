# resource_packer
A simple resource packer that recursively traverses the current working directory, inserts every file into a hashmap, and writes it to a `resources` folder or `resources.pck` file.

## Usage
```
Usage: resource_packer.exe [OPTIONS]

Options:
      --pack-mode <PACK_MODE>  The mode/format the packer will use [default: multi] [possible values: multi, single]
  -h, --help                   Print help (see more with '--help')
```

## Behavior
It skips packing itself by comparing its file name (Usually `resource_packer.exe`). It also skips packing any file with the `.pck` extension.
