sealed trait Vector[+T]:
    import Vector.VectorConvert
    def ||>[S](f: T => S): VectorConvert[this.type, S]
    def combine[S, U](f: (T, S) => U, x: VectorConvert[this.type, S]): VectorConvert[this.type, U] /* FIX THIS.TYPE */

final case class Cons[+T, A <: Vector[T]](t: T, v: A) extends Vector[T]:
    import Vector.VectorConvert
    def ||>[S](f: T => S): Cons[S, VectorConvert[A, S]] = Cons(f(t), v.asInstanceOf[A] ||> f)
    def combine[S, U](f: (T, S) => U, x: Cons[S, VectorConvert[A, S]]): Cons[U, VectorConvert[A, U]] =
        Cons(f(t, x.t), v.combine(f, x.v.asInstanceOf[VectorConvert[v.type, S]]).asInstanceOf[VectorConvert[A, U]])

sealed trait VNil extends Vector[Nothing]

case object VNil extends VNil:
    import Vector.VectorConvert
    def ||>[T, S](f: T => S): VNil = VNil
    def combine[S, U](f: (_, S) => U, x: VNil): VNil = VNil

object Vector:
    type VectorConvert[V <: Vector[_], +S] <: Vector[S] = V match {
        case Cons[_, v] => Cons[S, VectorConvert[v, S]]
        case VNil => VNil
    }
    def [T, S <: Vector[T]](x: T) ## (y: S): Cons[T, S] = Cons(x, y)
    def [T](x: T) ## (y: T): Cons[T, Cons[T, VNil]] = Cons(x, Cons(y, VNil))
    def [T, S, U](x: Vector[T]) << (f: (T, S) => U): (x.type, (T, S) => U) = (x, f)
    def [T, S, U, A <: Vector[T]](x: (A, (T, S) => U)) >> (y: VectorConvert[A, S]): VectorConvert[A, U] =
        val a = x._1
        a.combine(x._2, y.asInstanceOf[VectorConvert[a.type, S]]).asInstanceOf[VectorConvert[A, U]]

/*
Foo < Bar
Vector[Foo] < Vector[Bar]


*/