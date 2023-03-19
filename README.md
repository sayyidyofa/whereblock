# whereblock
CLI Tool to find coordinates of blocks that match a string from minecraft region files

## How to Use (may change):

`whereblock "BLOCK_STRING" "PATH_TO_REGION_FILES"`

where BLOCK_STRING is a string/substring of the block'stag being searched, and PATH_TO_REGION_FILES is absolute path to regions file directory

### Example:

`whereblock diamond C:\\Users\\admin\\.minecraft\\saves\\world\\region`

### Output (may change)

Sample streaming output until it finishes:

`Block at [realpos -171, 1, -176] mca r.-1.-1.mca [chunk x: 21, y: 21] [chunkpos x: 5, y: 4, z: 0] has name: minecraft:air AND description: minecraft:air|`

## TODOs
 - Help command
 - Option to scan full chunks/partially rendered chunks
 - Skip non-mca files
 - Refactor unwraps
 - Validate user inputs
 - Option to export result as JSON
 - Option to export result as Xaero's Waypoints
 - Option to scan one region file/region files that match string
