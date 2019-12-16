{-# LANGUAGE OverloadedStrings #-}

module Day2 (day2) where

import Control.Monad (guard)
import Control.Monad.ST
import Data.List
import Data.List.Split
import qualified Data.Vector.Unboxed as V
import qualified Data.Vector.Unboxed.Mutable as MV

import Intcode

runProgramWithNV ::Int -> Int -> V.Vector Int -> V.Vector Int
runProgramWithNV noun verb program = runST $ do
  thawed <- V.thaw program
  MV.write thawed 1 noun
  MV.write thawed 2 verb
  modified <- V.unsafeFreeze thawed
  let (ram, _, Halted) = runProgram modified 0 0 []
  return ram

desiredOutput :: Int
desiredOutput = 19690720

day2 :: String -> IO ()
day2 input = do
  let program = (V.fromList $ map read $ splitOn "," input) :: V.Vector Int
  let part1 = runProgramWithNV 12 2 program V.! 0
  let part2 =
        find (\(n, v) -> runProgramWithNV n v program V.! 0 == desiredOutput)
             [(n, v) | n <- [0..99], v <- [0..99]]
  guard (part1 == 5290681 && part2 == Just (57, 41))
  print part1
  print part2
