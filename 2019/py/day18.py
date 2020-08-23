#!/usr/bin/env python3

import heapq
import numpy as np
import functools
import itertools
from collections import deque
from dataclasses import dataclass
from typing import FrozenSet


PART1 = 0
PART2 = 1


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
    start_state = State(frozenset([(x, y)]), frozenset(), PART1)
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
    start_state = State(frozenset(zip(xs, ys)), frozenset(), PART2)
    num_keys = np.count_nonzero(np.vectorize(lambda val: val.islower())(vault))
    assert dijkstra(vault, start_state, num_keys) == 1514


@dataclass
class State(object):
    positions: FrozenSet
    keys: FrozenSet
    tag: int

    def neighbor_states(self, vault):
        positions = list(self.positions)
        for i, pos in enumerate(positions):
            neighbors = self.find_neighbors(vault, pos)
            for ngbr_val, ngbr_pos, ngbr_dist in neighbors:
                new_positions = [x for x in positions]
                new_positions[i] = ngbr_pos
                new_positions = frozenset(new_positions)
                if is_key(ngbr_val):
                    keys = set(self.keys)
                    keys.add(ngbr_val)
                    yield (State(new_positions, frozenset(keys), self.tag), ngbr_dist)
                elif is_door(ngbr_val):
                    if ngbr_val.lower() in self.keys:
                        yield (State(new_positions, self.keys, self.tag), ngbr_dist)

    def find_neighbors(self, vault, pos):
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

        @functools.lru_cache(maxsize=100)
        def cached(pos, tag):
            result = set()
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
                    if is_key(val) or is_door(val):
                        result.add((val, adj, dist + 1))
                        continue
                    queue.append((adj, dist + 1))

            return result

        return cached(pos, self.tag)

    def __hash__(self):
        return hash((self.positions, self.keys))


def dijkstra(vault, start_state, num_keys):
    max_ = 0
<<<<<<< HEAD
    tie_breaker = itertools.count()
=======
    tie_breaker = itertools.count() 
>>>>>>> 1c222b6f7a72b69af8115d7c54c295da55c97f1d
    queue = [[0, next(tie_breaker), start_state]]
    state_to_entry = {}
    visited = set()
    while queue:
        dist, _, state = heapq.heappop(queue)
        if state is None:
            continue
        visited.add(state)
        if max_ < len(state.keys):
            max_ = len(state.keys)
            print(max_, dist)

        if len(state.keys) == num_keys:
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


def is_key(val):
    return val.islower()


def is_door(val):
    return val.isupper()
