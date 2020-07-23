<!-- markdownlint-disable no-inline-html no-bare-urls line-length header-increment -->
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
