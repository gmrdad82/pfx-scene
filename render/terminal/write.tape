Sleep 700ms
Type "scene-write scenes/room.scene.toml move 'right crate' 1.5 0.5 0.3"
Sleep 300ms
Enter
Sleep 1s
Type "scene-write scenes/room.scene.toml material lamp gold"
Sleep 300ms
Enter
Sleep 1.4s
Type "scene-write scenes/room.scene.toml rename lamp 'ceiling lamp'"
Sleep 300ms
Enter
Sleep 1s
Type "git diff -U1 --no-index tmp/room.toml scenes/room.scene.toml"
Sleep 300ms
Enter
Sleep 4.5s
