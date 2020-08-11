<!-- markdownlint-disable no-inline-html no-bare-urls line-length header-increment -->

# Space

## Basic Space Structure

At each point in space, there are three vectors. These vectors are the $x$, $y$, and $z$ vectors, which are represented in terms of the global $i$, $j$, and $k$ vectors. The $x$, $y$, and $z$ vectors are the basis the **Relative** coordinate system, and the $i$, $j$, and $k$ vectors are the basis of the **Absolute** coordinate system.

## Path of a Moving Particle with no External Forces Acting on it (Light)

Suppose that $\vec{s}$ is the *absolute* position of the particle, and $f(\vec{s})$ describes the basis of the *relative* coordinate system at that point. Also assume that $\vec{v}$ is the *relative* velocity vector. Then, the particle follows the *absolute* path:

$$\frac{d\vec{s}}{dt} = f(\vec{s}) \cdot \vec{v}$$

## Interpolation

Of course, it is inefficient to store the *relative* basis for each point. As such, the *relative* basis is only stored for points in a fixed grid with large spacing, and interpolation is used to calculate the basis at an arbitrary place. For simplicity, the size of the grid will be $1$.

### Trilinear Interpolation

The simplest form of interpolation is trilinear interpolation. In this case, the basis at a point is equal to the basis at each of the corners multiplied by the area of the cube on the opposite side. More details can be seen at https://en.wikipedia.org/wiki/Trilinear_interpolation.

#### Euler's Method

As the differential equation describing the path of light using trilinear interpolation is not solvable, it is necessary to use numerical methods to do it. Euler's method is very simple. Just find the derivative, add, and repeat.

#### Using the First Derivative

As the Gradient of $\frac{d\vec{s}}{dt}$ is defined, we can use that to get a more accurate interpolation. Unfortunately, this requires solving a linear first-order differential equation, which is solvable but requires the cubic formula.

As such, barycentric interpolation is preferred.

### Barycentric Interpolation

In Barycentric Interpolation, the cube is separated into a series of tetrahedrons. This method works by using barycentric coordinates, where a point is specified as a weighted average of the corners.

Suppose that the four locations are $a$, $b$, $c$, and $d$, and the weights are $x$, $y$, $z$, and $w$, and the value function is $f$. Then, the value at an arbitrary location (described by the weights) is:

$$x f(a) + y f(b) + z f(c) + w f(d)$$

To use Barycentric Interpolation, the cube is split into 5 tetrahedrons as shown in this picture: ![Tetrahedron Split](cube.jpeg)

There are four outside tetrahedrons and one interior regular tetrahedron.

Barycentric coordinates for the outside tetrahedrons are simple. For brevity, I will include only the $(1, 0, 0), (0, 1, 0), (0, 0, 1), (0, 0, 0)$ tetrahedron.

The weights for this tetrahedron (assuming a position $s = (a, b, c)$), are:

$$x = a$$
$$y = b$$
$$z = c$$
$$w = 1 - x - y - z$$

The middle tetrahedron is a bit more complicated. This one has vertices $(1, 0, 0), (0, 1, 0), (0, 0, 1), (1, 1, 1)$.

As such, we get the system of equations:

$$x + w = a$$
$$y + w = b$$
$$z + w = c$$
$$x + y + z + w = 1$$

Which can be solved to get:

$$x = \frac{a - b - c + 1}{2}$$
$$y = \frac{b - a - c + 1}{2}$$
$$z = \frac{c - a - b + 1}{2}$$
$$w = \frac{a + b + c - 1}{2}$$

## Optimization

If the cube's corners are all the same, no interpolation is necessary, and the path of the particle is simply a straight line.
