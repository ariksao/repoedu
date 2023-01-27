import bpy
import math
from random import randint

# Hello!

a = 2
b = 3
r = 30

stepSize = 0.5
rounds = 6
deviation = 5

t = 0
while t < 2 * math.pi * rounds:
    x = r * math.cos(t) + a
    y = r * math.sin(t) + a
    z = 30
    d = deviation
    bpy.ops.mesh.primitive_torus_add(location=(x + randint(-1 * d,1 * d),y + randint(-1 * d,1 * d),z + randint(-1 * d,1 * d)))
    t += stepSize