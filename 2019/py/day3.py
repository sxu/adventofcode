#!/usr/bin/env python3

import itertools
import operator
from dataclasses import dataclass


def day3(input_path):
    with open(input_path) as f:
        lines = f.read()
    lines = lines.strip().split("\n")
    wire1, wire2 = [follow(l.split(",")) for l in lines]
    assert (
        min(map(lambda p: sum(map(abs, p[0])), wire_intersections(wire1, wire2))) == 870
    )
    assert min(map(lambda x: x[1], wire_intersections(wire1, wire2))) == 13698


@dataclass
class Segment(object):
    start: (int, int)
    end: (int, int)
    orientation: int

    HORIZONTAL = 0
    VERTICAL = 1


def follow(directions):
    result = []
    start = (0, 0)
    dist = 0
    for d in directions:
        length = int(d[1:])
        if d[0] == "U":
            end = (start[0], start[1] + length)
            orientation = Segment.VERTICAL
        elif d[0] == "D":
            end = (start[0], start[1] - length)
            orientation = Segment.VERTICAL
        elif d[0] == "L":
            end = (start[0] - length, start[1])
            orientation = Segment.HORIZONTAL
        elif d[0] == "R":
            end = (start[0] + length, start[1])
            orientation = Segment.HORIZONTAL
        else:
            raise RuntimeError(f"Malformed direction {d}")
        result.append((Segment(start, end, orientation), dist))
        dist += length
        start = end
    return result


def segment_intersection(seg1, seg2):
    if seg1.orientation == seg2.orientation:
        return None

    if seg1.orientation == Segment.HORIZONTAL:
        h_seg, v_seg = seg1, seg2
    else:
        h_seg, v_seg = seg2, seg1

    x = v_seg.start[0]
    h_left = min(h_seg.start[0], h_seg.end[0])
    h_right = max(h_seg.start[0], h_seg.end[0])
    if x < h_left or x > h_right:
        return None

    y = h_seg.start[1]
    v_top = max(v_seg.start[1], v_seg.end[1])
    v_bottom = min(v_seg.start[1], v_seg.end[1])
    if y < v_bottom or y > v_top:
        return None

    return (x, y)


def wire_intersections(wire1, wire2):
    def l1(p1, p2):
        return abs(p1[0] - p2[0]) + abs(p1[1] - p2[1])

    result = []
    for (seg1, dist1), (seg2, dist2) in itertools.product(wire1, wire2):
        inter = segment_intersection(seg1, seg2)
        if inter is not None and inter != (0, 0):
            wire_steps = dist1 + dist2 + l1(inter, seg1.start) + l1(inter, seg2.start)
            result.append((inter, wire_steps))
    return result
