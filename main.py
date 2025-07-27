import ctypes
import math
import time

mydll = ctypes.cdll.LoadLibrary(
    "./tiny-rpa-engine/target/release/my_tiny_rpa_engine.dll"
)

px, py = 100 * math.sin(0), 100 * math.cos(0)

for t in range(100000):
    time.sleep(0.005)
    dx = 100 * math.sin(t * math.pi / 50) - px
    dy = 100 * math.cos(t * math.pi / 50) - py
    mydll.mmv(int(dx), int(dy))
    px, py = px + dx, py + dy
    print(px, py, dx, dy)
