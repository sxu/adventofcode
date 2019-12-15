module Day9 (day9) where

import Data.List.Split (splitOn)
import qualified Data.Vector.Unboxed as V

import Intcode

day9 :: String -> IO ()
day9 input = do
  let program = (V.fromList $ map read $ splitOn "," input) :: V.Vector Int
  let (_, outputs, prgState) = runProgram program 0 0 [1]
  print (prgState, reverse outputs)
  let (_, outputs2, prgState2) = runProgram program 0 0 [2]
  print (prgState2, reverse outputs2)