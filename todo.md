## TODOs

[ ] load all meshes during startup preferrably app.add_systems(OnEnter(GameState::Loading), load_assets); 
[ ] split textures into, debug, blocks, redstone
[x] split dirty_blocks into, dirty_blocks and dirty_redstone
[x] write seperate renderer for the redstone
[ ] rename files, and structure project into directories
[ ] refactor to use SpawnCtx instead (SystemParam)
[x] add redstone turns
[ ] ensure that blocks count as redstone adjecent (so line into blocks, and not dot)
[ ] implement so that redstone go up ontop of blocks (side redstone)
[ ] fix some Z fighting
[ ] build some sort of render queue, that distributes rendering assignments to correct renderer
