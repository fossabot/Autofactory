object Main:
    def main(args: Array[String]): Unit =
        val dt = 1.0/60
        val o = Space2.Axes.default
        val x = (Vec2(1, 1).normalize, Vec2(1, -1).normalize)
        val space = Space2.from(Seq(
            Seq(o, o, o, o, o),
            Seq(o, o, x, x, o),
            Seq(o, x, x, x, o),
            Seq(o, x, x, o, o),
            Seq(o, o, o, o, o)
        ))
        def iterate(pos: Vec2, vel: Vec2, t: Double): Vec2 =
            if t <= 0 then
                pos
            else
                iterate(space.step(pos, vel, dt), vel, t - dt)
        val forward = iterate(Vec2(1, 1), Vec2(1, 1), 2)
        println(forward)
        println(iterate(forward, Vec2(-1, -1), 2))