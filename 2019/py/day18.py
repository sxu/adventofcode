#!/usr/bin/env python3

import heapq
import numpy as np
import functools
import itertools
from collections import deque
from dataclasses import dataclass
from typing import List


reachable_keys_cache = {}


def day18(input_path):
    lines = []
    with open(input_path) as f:
        for line in f:
            lines.append(line.strip())
    n = len(lines)
    m = len(lines[0])
    vault = np.array([list(line) for line in lines])
    num_keys = np.count_nonzero(np.vectorize(lambda val: val.islower())(vault))

    (x,), (y,) = np.where(vault == "@")

    global reachable_keys_cache
    reachable_keys_cache = {}
    start_state = State([(x, y)], 0)
    assert dijkstra(vault, start_state, num_keys) == 3512

    vault[x][y] = "#"
    vault[x - 1][y] = "#"
    vault[x + 1][y] = "#"
    vault[x][y - 1] = "#"
    vault[x][y + 1] = "#"
    vault[x - 1][y - 1] = "@"
    vault[x - 1][y + 1] = "@"
    vault[x + 1][y - 1] = "@"
    vault[x + 1][y + 1] = "@"

    xs, ys = np.where(vault == "@")
    start_state = State(list(zip(xs, ys)), 0)
    num_keys = np.count_nonzero(np.vectorize(lambda val: val.islower())(vault))
    reachable_keys_cache = {}
    assert dijkstra(vault, start_state, num_keys) == 1514


@dataclass
class State(object):
    positions: List
    keys: int

    def neighbor_states(self, vault):
        for i, pos in enumerate(self.positions):
            neighbors = find_reachable_keys(vault, pos, self.keys)
            for ngbr_val, ngbr_pos, ngbr_dist in neighbors:
                new_positions = [x for x in self.positions]
                new_positions[i] = ngbr_pos
                keys = self.keys | (1 << (ord(ngbr_val) - ord("a")))
                yield (State(new_positions, keys), ngbr_dist)

    def __hash__(self):
        return hash((tuple(self.positions), self.keys))


def dijkstra(vault, start_state, num_keys):
    def set_lowest_bits(n):
        mask = 0
        for i in range(n):
            mask |= 1 << i
        return mask

    target = set_lowest_bits(num_keys)
    max_ = 0
    tie_breaker = itertools.count()
    queue = [[0, next(tie_breaker), start_state]]
    state_to_entry = {}
    visited = set()
    while queue:
        dist, _, state = heapq.heappop(queue)
        if state is None:
            continue
        visited.add(state)

        if state.keys == target:
            return dist

        for ngbr_state, ngbr_dist in state.neighbor_states(vault):
            if ngbr_state in visited:
                continue

            new_dist = dist + ngbr_dist
            new_entry = [new_dist, next(tie_breaker), ngbr_state]
            if ngbr_state in state_to_entry:
                existing_entry = state_to_entry[ngbr_state]
                if new_dist < existing_entry[0]:
                    existing_entry[-1] = None
                    heapq.heappush(queue, new_entry)
                    state_to_entry[ngbr_state] = new_entry
            else:
                state_to_entry[ngbr_state] = new_entry
                heapq.heappush(queue, new_entry)

    raise RuntimeError(f"Didn't find all {num_keys} keys")


def find_reachable_keys(vault, pos, keys):
    def up(pos):
        return (pos[0] - 1, pos[1])

    def down(pos):
        return (pos[0] + 1, pos[1])

    def left(pos):
        return (pos[0], pos[1] - 1)

    def right(pos):
        return (pos[0], pos[1] + 1)

    def out_of_bounds(pos):
        return pos[0] < 0 or pos[1] < 0

    def is_wall(val):
        return val == "#"

    def is_key(val):
        return val.islower()

    def is_door(val):
        return val.isupper()

    global reachable_keys_cache
    cache_key = (pos, keys)
    if cache_key in reachable_keys_cache:
        return reachable_keys_cache[cache_key]

    result = []
    visited = set()
    queue = deque([(pos, 0)])
    while queue:
        pos, dist = queue.popleft()
        for adj in (up(pos), down(pos), left(pos), right(pos)):
            if out_of_bounds(pos):
                continue
            val = vault[adj]
            if is_wall(val) or adj in visited:
                continue
            visited.add(adj)
            if is_key(val):
                result.append((val, adj, dist + 1))
                continue
            if is_door(val):
                if not keys & (1 << (ord(val) - ord("A"))):
                    continue
            queue.append((adj, dist + 1))

    reachable_keys_cache[cache_key] = result
    return result
