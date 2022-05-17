from ReveeEngine import *

tree = Tree([])
engine = Engine(tree)
sprite1 = engine.create_node(Nodes.Texture, Vec2(100, 200), {"path": "C:\\Users\\jakisuzytkownikniepodamnazwy\\Documents\\ReveeEngine\\assets\\icon.png"})
engine.add_node(sprite1)
engine.start()
