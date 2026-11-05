NO AI HELP!!!!!!!!!!!!!!!!

DAY 1
Set up Rust compilation to WASM and a TypeScript frontend with a button to add objects.
https://rustwasm.github.io/docs/book/game-of-life/setup.html

DAY 2
Set up a graphics engine using a canvas object.
And have actively drawn objects.

DAY 3
Create a Vec2 class with methods like .magnitude, .normalized, and math operations between vectors.

DAY 4
Add Object Settings (adjust mass, size, and elasticity)

DAY 5-6
Make a function to get the normal direction from a specific point (not sure how to do this) :( WAIT I FIGURED IT OUT,
each object should have its own area that it owns. Before an object moves, it should look to see what object it is. Then, if the object is a circle, it can get the direction away from the center (which is always the normal point)
and calculate it by the position it wants to move to. For a square, it checks the edge and, since the square is a square, gets the corner points and uses the perpendicular angle for the normal. (if it hits a corner not sure what to do)

DAY 7
Add movement details (momentum, travel direction).

Continue working on this
