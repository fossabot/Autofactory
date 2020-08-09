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
