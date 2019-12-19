module Day17 (day17) where

import Control.Monad (guard)
import Data.Char (chr, ord)
import qualified Data.HashSet as S
import Data.List.Split (splitOn)
import qualified Data.Vector.Unboxed as V

import Intcode

data Instruction = TurnLeft
                 | TurnRight
                 | MoveForward Int

instance Show Instruction where
  show TurnLeft = "L"
  show TurnRight = "R"
  show (MoveForward steps) = show steps

data Direction = Up | Down | West | East

parseView :: String -> (S.HashSet (Int, Int), [((Int, Int), Direction)])
parseView = go 0 0 S.empty []
  where
    go _ _ scaffolds mbRobot [] = (scaffolds, mbRobot)
    go x y scaffolds mbRobot (c:cs)
      | c == '^' || c == 'v' || c == '<' || c == '>' =
        go (x + 1) y
           (S.insert (x, y) scaffolds)
           (((x, y), toDirection c) : mbRobot)
           cs
      | c == '#' = go (x + 1) y (S.insert (x, y) scaffolds) mbRobot cs
      | c == '.' || c == 'X' = go (x + 1) y scaffolds mbRobot cs
      | c == '\n' = go 0 (y + 1) scaffolds mbRobot cs
      | otherwise = error $ "invalid character: " ++ [c]
    toDirection '^' = Up
    toDirection 'v' = Down
    toDirection '>' = East
    toDirection '<' = West
    toDirection _ = error "unreachable"

findIntersections :: S.HashSet (Int, Int) -> S.HashSet (Int, Int)
findIntersections scaffolds = S.filter isIntersection scaffolds
  where isIntersection (x, y) = all ($ scaffolds) [ S.member (x + 1, y)
                                                  , S.member (x - 1, y)
                                                  , S.member (x, y + 1)
                                                  , S.member (x, y - 1)
                                                  ]

walkScaffolds :: S.HashSet (Int, Int)
              -> (Int, Int)
              -> Direction
              -> [Instruction]
walkScaffolds scaffolds = walk
  where
    walk :: (Int, Int) -> Direction -> [Instruction]
    walk pos dir
      | isScaffold (forward dir pos 1) =
        MoveForward stepsForward : walk newPos dir
      | otherwise = case mbTurn of
                      Just (turn, newDir) -> turn : walk pos newDir
                      Nothing -> []
      where
        (stepsForward, newPos) =
          let steps= length $ takeWhile (isScaffold . forward dir pos) [1 ..]
          in (steps, forward dir pos steps)
        mbTurn
          | isScaffold left = Just (TurnLeft, turnLeft dir)
          | isScaffold right = Just (TurnRight, turnRight dir)
          | otherwise = Nothing
        left = forward (turnLeft dir) pos 1
        right = forward (turnRight dir) pos 1
        isScaffold pos' = S.member pos' scaffolds
        forward Up (x, y) steps = (x, y - steps)
        forward Down (x, y) steps = (x, y + steps)
        forward West (x, y) steps = (x - steps, y)
        forward East (x, y) steps = (x + steps, y)
        turnLeft Up = West
        turnLeft Down = East
        turnLeft West = Down
        turnLeft East = Up
        turnRight Up = East
        turnRight Down = West
        turnRight West = Up
        turnRight East = Down

day17 :: String -> IO ()
day17 input = do
  let program = map read $ splitOn "," input :: [Int]
  let (_, outputs, _) = runProgram (V.fromList program) 0 0 []
  let view = map chr outputs
  let (scaffolds, rs) = parseView view
  let (pos, dir) = head rs
  let part1 = sum $ map (uncurry (*)) $ S.toList $ findIntersections scaffolds
  guard (part1 == 4044)
  let mainRoutine = "A,B,A,C,A,B,C,B,C,B"
  let funcA = "R,8,L,10,L,12,R,4"
  let funcB = "R,8,L,12,R,4,R,4"
  let funcC = "R,8,L,10,R,8"
  let inputs = unlines [mainRoutine, funcA, funcB, funcC, "n"]
  let (_, outputs2, _) =
        runProgram (V.fromList $ 2 : tail program) 0 0 $ map ord inputs
  let part2 = last outputs2
  guard (part2 == 893283)
  print part1
  print $ walkScaffolds scaffolds pos dir
  print part2
