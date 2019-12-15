module Day5 (day5) where

import Control.Exception (assert)
import Data.List.Split
import qualified Data.Vector.Unboxed as V

import Intcode

day5 :: String -> IO ()
day5 input = do
  let program = (V.fromList $ map read $ splitOn "," input) :: V.Vector Int
  let (_, outputs1, Halted) = runProgram program 0 0 [1]
  let (_, outputs2, Halted) = runProgram program 0 0 [5]
  assert (head outputs1 == 13285749 && head outputs2 == 5000972) $ return ()
  print $ head outputs1
  print $ head outputs2
