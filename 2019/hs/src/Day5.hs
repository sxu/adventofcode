module Day5 (day5) where

import Data.List.Split
import qualified Data.Vector.Unboxed as V

import Intcode

day5 :: String -> IO ()
day5 input = do
  let program = (V.fromList $ map read $ splitOn "," input) :: V.Vector Int
  let (_, outputs1, Halted) = runProgram program 0 [1]
  print $ head outputs1
  let (_, outputs2, Halted) = runProgram program 0 [5]
  print $ head outputs2
