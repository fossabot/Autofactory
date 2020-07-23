import math.floor

case class Vec2(x: Double, y: Double):
    def +(other: Vec2): Vec2 =
        Vec2(x + other.x, y + other.y)
    def *(other: Vec2): Vec2 =
        Vec2(x * other.x + y * other.y, x * other.y + y * other.x)
    def *(other: Double): Vec2 =
        Vec2(x * other, y * other)
    def /(other: Double): Vec2 =
        Vec2(x / other, y / other)
    def -(other: Vec2): Vec2 =
        Vec2(x - other.x, y - other.y)
    def unary_- : Vec2 = Vec2(-x, -y)
    def bounding: (Vec2, Vec2, Vec2, Vec2) =
        (Vec2(floor(x), floor(y)), Vec2(floor(x) + 1, floor(y)), Vec2(floor(x), floor(y) + 1), Vec2(floor(x) + 1, floor(y) + 1))
    def map(f: Double => Double): Vec2 = Vec2(f(x), f(y))
    def |>(f: Double => Double): Vec2 = map(f)
    def combine[T](f: (Double, Double) => T): T = f(x, y)
    def <>[T](f: (Double, Double) => T): T = combine(f)
    def merge(other: Vec2, f: (Double, Double) => Double): Vec2 =
        Vec2(f(x, other.x), f(y, other.y))
    def elementProduct(other: Vec2): Vec2 = merge(other, _ * _)
    def len: Double = x * x + y * y
    def normalize: Vec2 = this / len

object Vec2:
    val i = Vec2(1, 0)
    val j = Vec2(0, 1)
    val origin = Vec2(0, 0)
    def (x: Double) * (y: Vec2) = y * x;

final class Space2 private (
        val width: Int,
        val height: Int,
        val grid: Array[Array[Space2.Axes]]
    ):
    import Vec2.{*}
    import Space2.{Absolute, Relative, Axes}
    def (x: Axes) + (y: Axes) = (x._1 + y._1, x._2 + y._2)

    def set(x: Int, y: Int, axes: Axes): Unit =
        grid(x)(y) = axes
    def set(a: Vec2, axes: Axes): Unit =
        grid(a.x.toInt)(a.y.toInt) = axes
    def at(x: Int, y: Int): Axes = grid(x)(y)
    def at(a: Vec2): Axes = grid(a.x.toInt)(a.y.toInt)
    def computeAxes(pos: Absolute): Axes =
        val bounds = pos.bounding
        def distance(other: Absolute): Absolute =
            (pos - other) |> math.abs
        def axes(other: Absolute, opposite: Absolute): Axes =
            val axes = at(other)
            val dist = (distance(opposite) <> (_ * _))
            (axes._1 * dist, axes._2 * dist)
        axes(bounds._1, bounds._3)
            + axes(bounds._2, bounds._4)
            + axes(bounds._3, bounds._1)
            + axes(bounds._4, bounds._2)

    def step(pos: Absolute, vel: Relative, dt: Double): Absolute =
        val axes = computeAxes(pos)
        pos + dt * (axes._1 * vel._1 + axes._2 * vel._2)

object Space2:
    type Absolute = Vec2
    type Relative = Vec2
    type Axes = (Absolute, Absolute)
    object Axes:
        val default = (Vec2.i, Vec2.j)

    def apply(width: Int, height: Int): Space2 = new Space2(width, height, Array.fill(width, height)(Axes.default))
    def from(x: Seq[Seq[Axes]]): Space2 =
        val width = x.size
        val height = x(0).size
        new Space2(width, height, Array(x.map(a => Array(a: _*)): _*))
