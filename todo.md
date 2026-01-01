## TODOs

[ ] load all meshes during startup preferrably app.add_systems(OnEnter(GameState::Loading), load_assets); 
[ ] split textures into, debug, blocks, redstone
[ ] split dirty_blocks into, dirty_blocks and dirty_redstone
[ ] write seperate renderer for the redstone
[ ] rename files, and structure project into directories
[ ] refactor to use SpawnCtx instead (SystemParam)
[ ] add redstone turns
[ ] ensure that blocks count as redstone adjecent (so line into blocks, and not dot)
[ ] implement so that redstone go up ontop of blocks (side redstone)
[ ] fix some Z fighting
