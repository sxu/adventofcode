module Day15 (day15) where

import Control.Monad
import qualified Data.HashMap.Strict as H
import qualified Data.HashSet as S
import Data.List.Split (splitOn)
import qualified Data.Vector.Unboxed as V

import Intcode

data Direction = North | South | West | East deriving (Bounded, Enum, Eq)

followDir :: Direction -> (Int, Int) -> (Int, Int)
followDir North (x, y) = (x, y + 1)
followDir South (x, y) = (x, y - 1)
followDir West (x, y) = (x - 1, y)
followDir East (x, y) = (x + 1, y)

data LocationType = Empty | Oxygen deriving (Eq, Show)

drawMap :: V.Vector Int
        -> ProgramState
        -> (Int, Int)
        -> H.HashMap (Int, Int) LocationType
        -> H.HashMap (Int, Int) LocationType
drawMap _ Halted _ _ = error "unreachable"
drawMap ram (Running pc rb) pos roomMap =
  let (ram', outputs, prgState) = runProgram ram pc rb []
  in case outputs of
       [] -> drawMap ram' prgState pos roomMap
       _ -> error "unreachable"
drawMap ram (WaitingForInput pc rb) pos roomMap =
  let roomMap' = go North roomMap
      roomMap'' = roomMap' `seq` go South roomMap'
      roomMap''' = roomMap'' `seq` go West roomMap''
      roomMap'''' = roomMap''' `seq` go East roomMap'''
  in roomMap''''
  where
    go dir roomMap' =
      if H.member newPos roomMap'
      then roomMap'
      else 
        case outputs of
          [0] -> roomMap'
          [1] -> drawMap ram' prgState newPos $ H.insert newPos Empty roomMap'
          [2] -> drawMap ram' prgState newPos $ H.insert newPos Oxygen roomMap'
          _ -> error "unreachable"
      where 
        (ram', outputs, prgState) = runProgram ram pc rb [1 + fromEnum dir]
        newPos = followDir dir pos

findOxygen :: H.HashMap (Int, Int) LocationType
           -> S.HashSet (Int, Int)
           -> [((Int, Int), Int, Direction)]
           -> Maybe Int
findOxygen _ _ [] = Nothing
findOxygen roomMap visited ((pos, dis, dir):queue) =
  case H.lookup newPos roomMap of
    Just Oxygen -> Just $ dis + 1
    Just Empty -> findOxygen roomMap
                             (S.insert newPos visited)
                             (queue' ++ newQueue)
    Nothing -> findOxygen roomMap visited queue'
  where
    newPos = followDir dir pos
    queue' = if dir == maxBound then queue else (pos, dis, succ dir):queue
    newQueue = [(newPos, dis + 1, minBound) | not (S.member newPos visited)]

fillOxygen :: H.HashMap (Int, Int) LocationType
           -> S.HashSet (Int, Int)
           -> Int
           -> [((Int, Int), Int, Direction)]
           -> Int
fillOxygen _ _ maxDis [] = maxDis
fillOxygen roomMap visited maxDis ((pos, dis, dir):queue) =
  case H.lookup newPos roomMap of
    Just _ -> fillOxygen roomMap
                         (S.insert newPos visited)
                         newMax
                         (queue' ++ newQueue)
    Nothing -> fillOxygen roomMap visited newMax queue'
  where
    newMax = max maxDis dis
    newPos = followDir dir pos
    queue' = if dir == maxBound then queue else (pos, dis, succ dir):queue
    newQueue = [(newPos, dis + 1, minBound) | not (S.member newPos visited)]

day15 :: String -> IO ()
day15 input = do
  let program = (V.fromList $ map read $ splitOn "," input) :: V.Vector Int
  let roomMap = drawMap program (Running 0 0) (0, 0) $ H.singleton (0, 0) Empty
  let disToO2 = findOxygen roomMap (S.singleton (0, 0)) [((0, 0), 0, minBound)]
  guard (disToO2 == Just 252)
  let (o2Pos, _) = head $ H.toList $ H.filter (== Oxygen) roomMap
  let timeToFill = fillOxygen roomMap 
                              (S.singleton o2Pos)
                              0
                              [(o2Pos, 0, minBound)]
  guard (timeToFill == 350)
  print disToO2
  print timeToFill
