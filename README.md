<!-- markdownlint-disable no-inline-html no-bare-urls line-length header-increment no-duplicate-header -->
# Spacebending Game

## Goals for the Project

The goals for this game are as follows:

The game should use a block-like system to allow people to build constructions. These blocks will not be made out of points with springs because (a) it would be way too processor-consuming, (b) it's very difficult to get it to work correct without collapsing, and (c) these are blocks not softbodies (it wouldn't be good for gameplay).

The game should provide people with some prebuilt weapons, but building your own weapons out of blocks should be incentivized.

I also want the game to support spacebending. Spacebending should not be the entire game, but should be an important way of solving problems. The only problem with this is that spacebending requires rewriting practically all of the physics engine and will cause it to be much slower.

The uses of spacebending in the game are to camoflauge a base, deliver items faster (although using a mass driver is probably best), fit more stuff into a space, redirect gunfire away from the base, etc.

This is probably not enough uses for spacebending for it to be worth implementing, although it still would be very cool.

The game should be a decently open-ended world, but just open enough for people to have free space, we don't want people to be lost.

It would be nice to have devices ran by AI. Being able to make a turret and setting what to do to rotate sideways, and what to do to rotate up/down, and the max angle on both, and what to do to fire, would be nice. Only issue is that the firing system may be rather complex, with a bullet having to be loaded (5 steps) etc. Perhaps if the game uses some sort of circuit blocks like minecraft could make it simpler. The gun would then rotate, then activate the firing circuit, which would presumably fire the gun and then reload.

The game should allow full mechanization of all parts of life, including building stuff, mining, etc. This can simply be done by having infinite resources in places, and allowing template builders. Then, the template builders can build other template builders...

Perhaps something where (a) you don't know where people are unless you have scanners or something (the player is a scanner), and (b) you can command units that have engines to go places.

I want there to be light pvp, but not an extreme amount of competition to tech faster than the opponent. If a person looses in a fight, he should not be completely ruined.

The game should probably be set in a giant clump of astroids. The space between two astroids should be approximately twice of the size of one, and there should be ship-sized to giant astroids. The astroids should go to zero gravity after a certain period, and perhaps repulse other astroids of the same size (if giant). This is to allow people to have space-bases, and then put a space-elevator to it.

This allows for people not to have to worry about gravity too much when building spaceships. Their spaceships can be built and should easily be able to escape the gravity of an astroid.

I'm not quite sure how to make the game slightly competitive but not totally. Adding in a neutral enemy would make there still be a reason to have defenses even if there are no other players to attack.

## Implementation

Blocks should be treated as one large polygon. When a block is hit with another block, they should both gain some stress equal to the force exerted. The stress slowly decays. When a block has high enough stress, it gets detached from other blocks. Of course, people can reconnect two things toghether, but that is not very important for now.

If a block reaches a critical amount of stress, it will be destroyed. This will cause an explosion which will exert a force on nearby things. The stress also will disperse to nearby blocks.

Armour will have a higher strength of both stress than other items. Reactive armour will just be destroyed and will never be detached.

Energy beams will just add stress to the objects it touches.

I also want force-fields. These fields will (a) be used to prevent space-dragging, and (b) are completely invulernable but cost power based on stress (and of course have a max limit).

I think the best way to make force-fields is to have a bunch of force-nodes, and you can connect each node to another, to make polygonal surfaces. But then, people can target the nodes. That may be a good idea, but it would be nice to make virtual projectors that treat it as if a force-node existed a bit away from that place. If force-nodes are small, there should be a seperate device which actually powers the force-fields.

---

# Non-Spacebending-Game

## Project Goals

The goals for the non-spacebending-game are as follows: The game should be a game with strategy about making and planning attacks. It should allow people to have freedom to build things in not the most optimal way, but yet have space for optimized designs. I want the game to support PvP well as well as customization.

## Design

The game should consist of multiple blocks. These blocks will fit within a 1x1x1 bounding box, but they may not be completely a cube. Some of them may be triangles, but they should all be simple geometric shapes (apart from turrets). The blocks will be fixed into a grid, which can move freely. This grid will be called an *entity*. Each *entity* will have physics collisions with other entities. Some entities which meet certain requirements (has thrusters) may be driven by an AI. There will also be devices which can manifacture entities.

## Implementation

### Entities

An entity needs to support three features:

* Quick insertion into an place adjacent to another place that is filled.
* Quick deletion
* Seperation into two entities
* Stress calculation quickly

#### Arrays

The problem with an array is that it can not be resized or split. If things have a max size and can not be split then arrays are good, but as this is not the case arrays are not useful.

#### Octree

An octree seems like a much better strategy. I'm not quite sure how an octree works, but it seems to have a thing where space is split into a bunch of small buckets. If each bucket has a max size of like 2 blocks on each side if it contains anything, then that can be represented as an array. If the entities split in one place, chances are that most buckets will not have to be redone, only a couple on the boundry.

### Vector / Matrix Math

Vectors should be an opaque type of an array, specialized. They should also have a seperate type which describes the length. I would like it if the constructor would be forced to fit the size, although that is difficult.

There are two ways to make Matrixes. The first way is where matrixes will be an array of array of vectors, with a type alias so that you don't have to write Vector twice. The second way is for them to be their own type that is a `T[][]`. The first way would make it a little more inconveinent to write vector/matrix multiplication. If we make vectors matrixes with a type alias that requires it to be single length, that would make it less efficient due to wrapping.

One issue is that the Java Multidimentional arrays are not truly Multidimentional, and as such it is probably more efficient to use the modulo mechanism instead. In that case, vectors would simply be matrixes with a single length on one direction.

### Interfacing

I plan to use GraalVM's Native Image feature to do interfacing with the physics engine (PhysX) which is written in C++. I probably will also use LWJGL as it provides a simple way of writing OpenGL code.

One difficulty is that GraalVM does not function well with C++ code it seems; it can only handle structs, not classes. PhysX seems to use C++ primarly and has a lot of classes.

We can just use the rust bindings (https://github.com/EmbarkStudios/physx-rs/tree/main/physx-sys) as these have a good C api instead. It may also be nice if I could auto-generate the Scala bindings as well. This link (https://github.com/EmbarkStudios/physx-rs/blob/main/physx-sys/src/generated/x86_64-unknown-linux/structgen_out.hpp) has a giant list of structs which can be parsed easily to generate Scala types.

The program could just be written in Rust instead, that would simplify quite a lot of these difficulties, although Rust is very annoying as it's type system can't be exploited in the ways I want.
