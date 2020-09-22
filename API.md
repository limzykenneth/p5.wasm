Currently the following functions are implemented:

| p5.wasm                               | p5                             |
| ------------------------------------- |--------------------------------|
| **Calculation**                       |                                |
| `wasm.abs(n)`                         | `abs(n)`                       |
| `wasm.ceil(n)`                        | `ceil(n)`                      |
| `wasm.constrain(n, low, high)`        | `constrain(n, low, high)`      |
| `wasm.dist(x1, y1, x2, y2)`           | `dist(x1, y1, x2, y2)`         |
| `wasm.dist3d(x1, y1, z1, x2, y2, z2)` | `dist(x1, y1, z1, x2, y2, z2)` |
| `wasm.exp(n)`                         | `exp(n)`                       |
| `wasm.floor(n)`                       | `floor(n)`                     |
| `wasm.lerp(start, stop, amt)`         | `lerp(start, stop, amt)`       |
| `wasm.log(n)`                         | `log(n)`                       |
| `wasm.mag(x, y)`                      | `mag(x, y)`                    |
| `wasm.map(n, start1, stop1, start2, stop2, [withinBounds])` | `map(n, start1, stop1, start2, stop2, [withinBounds])` |
| `wasm.norm(n, start, stop)`           | `norm(n, start, stop)`         |
| `wasm.round(n)`                       | `round(n)`                     |
| `wasm.round_decimal(n, decimal)`      | `round(n, decimal)`            |
| `wasm.sq(n)`                          | `sq(n)`                        |
| `wasm.sqrt(n)`                        | `sqrt(n)`                      |
| `wasm.fract(n)`                       | `fract(n)`                     |
| **Noise**                             |                                |
| `wasm.noise(x)`                       | `noise(x)`                     |
| `wasm.noise2d(x, y)`                  | `noise(x, y)`                  |
| `wasm.noise3d(x, y, z)`               | `noise(x, y, z)`               |
| `wasm.noise_seed(seed)`               | `noiseSeed(seed)`              |
| `wasm.noise_detail(lod, falloff)`     | `noiseDetail(lod, falloff)`    |
| **Vectors**                           |                                |
| `wasm.create_vector()`                | `createVector()`               |
| `wasm.create_vector_1d(x)`            | `createVector(x)`              |
| `wasm.create_vector_2d(x, y)`         | `createVector(x, y)`           |
| `wasm.create_vector_3d(x, y, z)`      | `createVector(x, y, z)`        |
|                                       |                                |
| `Vector.to_string()`                  | `Vector.toString()`            |
| `Vector.set_vector(vector)`           | `Vector.set(vector)`           |
| `Vector.set_1d(x)`                    | `Vector.set(x)`                |
| `Vector.set_2d(x, y)`                 | `Vector.set(x, y)`             |
| `Vector.set_3d(x, y, z)`              | `Vector.set(x, y, z)`          |
| `Vector.copy()`                       | `Vector.copy()`                |
| `Vector.add_vector(vector)`           | `Vector.add(vector)`           |
| `Vector.add_1d(x)`                    | `Vector.add(x)`                |
| `Vector.add_2d(x, y)`                 | `Vector.add(x, y)`             |
| `Vector.add_3d(x, y, z)`              | `Vector.add(x, y, z)`          |
| `Vector.sub_vector(vector)`           | `Vector.sub(vector)`           |
| `Vector.sub_1d(x)`                    | `Vector.sub(x)`                |
| `Vector.sub_2d(x, y)`                 | `Vector.sub(x, y)`             |
| `Vector.sub_3d(x, y, z)`              | `Vector.sub(x, y, z)`          |
| `Vector.mult(n)`                      | `Vector.mult(n)`               |
| `Vector.div(n)`                       | `Vector.div(n)`                |
| `Vector.mag()`                        | `Vector.mag()`                 |
| `Vector.mag_sq()`                     | `Vector.magSq()`               |
| `Vector.dot_vector(vector)`           | `Vector.dot(vector)`           |
| `Vector.dot_1d(x)`                    | `Vector.dot(x)`                |
| `Vector.dot_2d(x, y)`                 | `Vector.dot(x, y)`             |
| `Vector.dot_3d(x, y, z)`              | `Vector.dot(x, y, z)`          |
| `Vector.cross(vector)`                | `Vector.cross(vector)`         |
| `Vector.dist(vector)`                 | `Vector.dist(vector)`          |
| `Vector.normalize()`                  | `Vector.normalize()`           |
| `Vector.limit(max)`                   | `Vector.limit(max)`            |
| `Vector.set_mag(n)`                   | `Vector.setMag(n)`             |
| `Vector.heading()`                    | `Vector.heading()`             |
| `Vector.rotate(a)`                    | `Vector.rotate(a)`             |
| `Vector.angle_between(vector)`        | `Vector.angleBetween(vector)`  |
| `Vector.lerp_vector(vector)`          | `Vector.lerp(vector)`          |
| `Vector.lerp_1d(x)`                   | `Vector.lerp(x)`               |
| `Vector.lerp_2d(x, y)`                | `Vector.lerp(x, y)`            |
| `Vector.lerp_3d(x, y, z)`             | `Vector.lerp(x, y, z)`         |
| `Vector.array()`                      | `Vector.array()`               |
| `Vector.equals_vector(vector)`        | `Vector.equals(vector)`        |
| `Vector.equals_1d(x)`                 | `Vector.equals(x)`             |
| `Vector.equals_2d(x, y)`              | `Vector.equals(x, y)`          |
| `Vector.equals_3d(x, y, z)`           | `Vector.equals(x, y, z)`       |
| **Trigonometry**                                                       |
| `wasm.acos(value)`                    | `acos(value)`                  |
| `wasm.asin(value)`                    | `asin(value)`                  |
| `wasm.atan(value)`                    | `atan(value)`                  |
| `wasm.atan2(y, x)`                    | `atan2(y, x)`                  |
| `wasm.cos(angle)`                     | `cos(angle)`                   |
| `wasm.sin(angle)`                     | `sin(angle)`                   |
| `wasm.tan(angle)`                     | `tan(angle)`                   |
| `wasm.degrees(radians)`               | `degrees(radians)`             |
| `wasm.radians(degrees)`               | `radians(degrees)`             |


## Note
* If you create a vector using `wasm.create_vector[..]`, you have to use the corresponding p5.wasm functions with it and not the original p5.js ones. You should be able to pass these vectors directly to functions that accept vectors as an input, as long as they only try to access the `x`, `y`, `z` properties only.
* All trigonometry functions use radians as units, to pass in degrees as a unit, use `wasm.radians()` to convert the angle first.