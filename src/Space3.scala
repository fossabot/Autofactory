import math.floor

case class Vec3(x: Double, y: Double, z: Double):
    def +(other: Vec3): Vec3 =
        Vec3(x + other.x, y + other.y, z + other.z)
    def -(other: Vec3): Vec3 =
        Vec3(x - other.x, y - other.y, z - other.z)
    def *(other: Vec3): Vec3 =
        ??? /* TODO: FIX */
    def *(other: Double): Vec3 =
        Vec3(x * other, y * other, z * other)
    def /(other: Double): Vec3 =
        Vec3(x / other, y / other, z / other)
    def unary_- : Vec3 = Vec3(-x, -y, -z)
    def bounding: (Vec3, Vec3, Vec3, Vec3, Vec3, Vec3, Vec3, Vec3) =
        val fx = floor(x)
        val fy = floor(y)
        val fz = floor(z)
        val cx = fx + 1
        val cy = fy + 1
        val cz = fz + 1
        (Vec3(fx, fy, fz), Vec3(cx, fy, fz),
            Vec3(fx, cy, fz), Vec3(cx, cy, fz),
            Vec3(fx, fy, cz), Vec3(cx, fy, cz),
            Vec3(fx, cy, cz), Vec3(cx, cy, cz))
    def map(f: Double => Double): Vec3 = Vec3(f(x), f(y), f(z))
    def |>(f: Double => Double): Vec3 = map(f)
    def combine[T](f: (Double, Double, Double) => T): T = f(x, y, z)
    def <>[T](f: (Double, Double, Double) => T): T = combine(f)
    def merge(other: Vec3, f: (Double, Double) => Double): Vec3 =
        Vec3(f(x, other.x), f(y, other.y), f(z, other.z))
    def elementProduct(other: Vec3): Vec3 = merge(other, _ * _)
    def len: Double = x * x + y * y
    def normalize: Vec3 = this / len

object Vec3:
    val i = Vec3(1, 0, 0)
    val j = Vec3(0, 1, 0)
    val k = Vec3(0, 0, 1)
    val origin = Vec3(0, 0, 0)
    def (x: Double) * (y: Vec3) = y * x;

final class Space3 private (
        val width: Int,
        val height: Int,
        val length: Int,
        val grid: Array[Array[Array[Space3.Axes]]]
    ):
    import Vec3.{*}
    import Space3.{Absolute, Relative, Axes}
    def (x: Axes) + (y: Axes) = (x._1 + y._1, x._2 + y._2, x._3 + y._3)

    def set(x: Int, y: Int, z: Int, axes: Axes): Unit =
        grid(x)(y)(z) = axes
    def set(a: Vec3, axes: Axes): Unit =
        grid(a.x.toInt)(a.y.toInt)(a.z.toInt) = axes
    def at(x: Int, y: Int, z: Int): Axes = grid(x)(y)(z)
    def at(a: Vec3): Axes = grid(a.x.toInt)(a.y.toInt)(a.z.toInt)
    def computeAxes(pos: Absolute): Axes =
        val bounds = pos.bounding
        def distance(other: Absolute): Absolute =
            (pos - other) |> math.abs
        def axes(other: Absolute, opposite: Absolute): Axes =
            val axes = at(other)
            val dist = (distance(opposite) <> (_ * _ * _))
            (axes._1 * dist, axes._2 * dist, axes._3 * dist)
        axes(bounds._1, bounds._5)
            + axes(bounds._2, bounds._6)
            + axes(bounds._3, bounds._7)
            + axes(bounds._4, bounds._8)
            + axes(bounds._5, bounds._1)
            + axes(bounds._6, bounds._2)
            + axes(bounds._7, bounds._3)
            + axes(bounds._8, bounds._4)
    def step(pos: Absolute, vel: Relative, dt: Double): Absolute =
        val axes = computeAxes(pos)
        pos + dt * (axes._1 * vel._1 + axes._2 * vel._2 + axes._3 * vel._3)

object Space3:
    type Absolute = Vec3
    type Relative = Vec3
    type Axes = (Absolute, Absolute, Absolute)
    object Axes:
        val default = (Vec3.i, Vec3.j, Vec3.k)

    def apply(width: Int, height: Int, length: Int): Space3 = new Space3(width, height, length, Array.fill(width, height, length)(Axes.default))
    def from(x: Seq[Seq[Seq[Axes]]]): Space3 =
        val width = x.size
        val height = x(0).size
        val length = x(0)(0).size
        new Space3(width, height, length, Array(x.map(a => Array(a.map(b => Array(b: _*)): _*)): _*))
