# resource_packer
A simple resource packer that recursively traverses the current directory, inserts every file into a hashmap, and writes it to a `resources.pck` file.
## Behavior
It skips packing itself by comparing its file name (Usually `resource_packer.exe`). It also skips packing any file with the `.pck` extension.
## Todo
- Implement file splitting functionality, `resource0.pck`, `resource1.pck`, etc.
