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
[x] fix some Z fighting
[ ] build some sort of render queue, that distributes rendering assignments to correct renderer
[ ] remove mod.rs files

## Dust
[ ] when reading dust through corner rules, need to make sure there is an air block between

## Redstone Torches
[ ] when removing dirt block that holds the torch, torch needs to be destroyed
[ ] not be able to place redstone torches unless there is a block next to it
[ ] not be able to place redstone toch on redstone torches etc.
[ ] not place redstone torches on under sides

## Redstone lamps
[ ] overhaul logic to use the new type, asking for weak power etc
[ ] ensure buttons next to lamps work
[ ] ensure buttons on lamps work

## Ticks
[x] implement tick resource
[x] update ticks by fixed update time (10 per sec)
[x] visually display ticks on screen
[x] bool to pause/run ticks
[ ] step ticks n stops

## Stone Button
[x] run for 15 ticks, then kill power when placing button
[x] implement click on Button
[x] animate click on button
[ ] fix render of button placed on side of blocks
[ ] change visal direction of button depending on where on the block it is placed???

## Sound
[ ] Block placement Sound
[ ] Block destruction Sound
[ ] button click sound

## Lever
[ ] Build lever meshes
[ ] implement flip
[ ] hook up to redstone

## Repeater
[ ] build repeter meshes
[ ] implement repetition of redstone signal
[ ] click animation 
[ ] implement delay
[ ] power blocks?
