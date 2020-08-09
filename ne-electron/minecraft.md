<!-- markdownlint-disable no-inline-html no-bare-urls line-length header-increment no-duplicate-header -->

# Minecraft's Rendering

From what I've seen, I believe Minecraft's rendering works as follows:

Each block has a boolean for each face that tells whether that face is rendered or not. When a block is placed, if it isn't transparent (stair-like object), it hides the face behind it. The block also hides it's own faces that are adjacent to other blocks.

This computation may be able to be done on the GPU; if one chunk is stored as a giant buffer, if it is on the GPU the calculation of which faces are seen can be quite quickly done via a bulk method call (that will cull all the faces in the middle of the buffer). One example of chunk culling is this link: https://www.voxelquest.com/videos.html . You can see that within each chunk the outside is still rendered.

One way of making things less dependent on the things around them is to draw all the chunk boundry edges anyway, even if they wouldn't be seen. With decently large chunks (32x32x32), only a little stuff would actually be drawn unnecessarly.

The way Minecraft's shadows work is that each place has a light level, and things around it have a light level of the maximum light around it - 1. The sun makes all the most positive blocks have a light level of 16. There is also something to do with partial shadows, although I think that is just due to the light distribution.

# Game Rendering

I think that due to transparent objects, it is best to render the game in two steps. The first step uses the G-buffer and deferred fragment shaders, and after all the solid blocks have been rendered, then transparent objects are rendered on top. As the transparent blocks do not adjust the Z-buffer, it is better if they are rendered as late as possible. This could be done by some config, but I don't know how to do so. (Have transparent blocks be last on the buffer? I could use some sort of two-way stack.)

Additionally, if I use chunks seperated by octrees, I could do the chunk interior culling. I do not expect all the blocks to be transparent, so I believe it is better to specify transparent faces instead (like stairs, the side faces would be transparent but not the back).

Unfortunately, it seems that threejs doesn't allow for deferred rendering, and chunk culling will also be difficult with it (as you can't really specify individual faces).

If I use a series of chunks with a large octree for each entity, that may work decently well. I also need one giant octree for the whole world or something.

One thing someone suggested is to use floating points relative to the current sector (big area that includes many chunks) to avoid things like the minecraft far lands.

# Reasons why not to use points and springs

* Physics engines can not handle giant amounts of points and springs.
* I don't like how points and springs look (also I don't want my objects to feel like jello)
* If objects have input / output directions, having objects being spheres doesn't work very well.

If a physics engine could handle it, I think it would be fine to have blocks with springs between them.

# WebGL

I think that I can use three.js, although I will have to have a seperate vertex generating in C++ code instead of using three's giant tree. This allows for increased efficiency due to (a) only having all the sides I need, and (b) I can have improved loops. Another issue is that three.js does not support G-buffering. Ideally, I would do deferred rendering, and then after that finishes, compute all the transparent objects (where it doesn't write to the depth buffer, and as such is best left to the end). Unless I have a very intensive fragment shader, I may just be able to ignore the deferred rendering as I only draw textures with a little computation.

# Basic Test Case

I would like to have an astroid get split in two due to having a lot of stress. This could be done by configuring it so that wherever the person clicks on the screen the object at that location gets a bunch of stress (and some force). This would have both the rendering and physics.

# Issues

There are a couple of difficulties I find. The primary difficulty is that due to blocks not having a fixed size and data, there will need to be seperate storages. For example, an armour block only has the stress value, while another block like a thruster also consumes energy (and can take energy, and does damage to stuff behind it, etc). A weapon would take in some items as well. The way that I think Minecraft does it is that the chunks have a bunch of pointers and stuff gets garbage collected to prevent memory fragmentation. Similarly for Mindustry.

I don't really see how Minecraft could possibly store all of the blocks. I computed the amount of pointers in each chunk to be 500 kb, so by default it will be around 50 mb of data in the array. This is ignoring all of the objects (which won't actually be very much more, by default).

My game can work by allocating 32 bytes or so of memory for each block, and then having the remaining data like textures be stored per block type instead. The use of these 32 bytes will be for any sort of data needing to be stored. If a block needs to store more than this amount, then it can simply have a pointer to the actual data. Each chunk (16x16x16 area) will take around 131 kb of data. This will prevent memory fragmentation usually.

It seems that Minecraft actually has something better than the pointers to everything tactic. I'm not quite sure how Mindustry does their block management though.

Additionally, it seems that 32 bytes may be a bit too much. The stress is going to be a 32 bit int (4 bytes), direction is a byte, and a bunch of other booleans.

Due to the Node.js Buffer object being (a) able to be set with C++ code and (b) being a UInt8Array, I can pass it directly to the WebGL code. (This buffer will probably just store the verticies).

I don't think I will need to have the mipmap generator be done before-hand, as ~~it's only a constant time increase!!~~ it should not take long due to only having to generate the mipmaps once.

Each block will also need to have a boolean for each side which describes whether that side is visible. (It also needs a description of which sides are transparent, but that is not necessary to be stored in each block, it can be stored in the block type) Blocks will also need a 2 byte id (1 byte is too small, so I choose 2 bytes).

Most of the "methods" will be in a seperate array that is mapped by block id. I think it will be optiomal to set the block id at runtime, so therefore you don't have conflicting mods with two same IDs.

# Multiplayer

## Determinstic Lockstep

Determinstic lockstep is where a couple of the old physics steps are stored, and when people receive an old packet they rewind and apply it retroactively. This seems like a very good idea, but it has issues due to the game having many objects instead of just a couple. Because the game has many objects, it is necessary to store and copy these giant arrays to do deterministic lockstep. If we restrict the players influence to the area around the player, then it may make sense due to (a) each entity not having too much information with it, and (b) there being not too many entities. The only issue is when entities break up. Additionally, if the server is slow in sending new info, then the physics will suddenly jerk back and speed up for a bit before going back to normal time.

## Normal Multiplayer

Normal multiplayer is where the server does the physics simulation and tells the clients what is currently going on. One way this could work is to continously update entities. One issue is that the server will continously be having to send a lot of information to the client, and it may not be able to do so. (Low bandwidth = fail)

One way to fix this is to simulate each of the client's physics seperately, and have the server diff the virtual client and tell the actual client what happened. Needless to say, this requires way too much computation on the server to be feasible. If the client only asks for updates when it gets a packet late, that may work, but then all packets will be late. Obviously, the best solution is to just allow desync. One thing could work if chunks are updated instead of the whole map, and the updates are biased towards which places players are.

One thing that may work is that if (a) players have zero mass, and (b) all player effects take 1 second to happen (like building), then the game would barely have any desync. Building would wait for 1 second before actually placing the object (at half built already). If the packet gets sent to the server later than 1 second, the sender needs to resync. If it gets sent to the clients late (and gets acknowledged by the server) then those clients have to resync. The server will have a lesser time requirement so that a hacked client doesn't cause many refreshes by the other clients. Additionally, this will make building small objects really inconveinent as it will always be slow.

---

One issue is that PhysX doesn't sleep objects correctly; entities will probably always be moving (albet slowly). This is still decently good as objects far away from the players are unloaded and are effectively slept.
