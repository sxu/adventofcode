module Day9 (day9) where

import Control.Monad (guard)
import Data.List.Split (splitOn)
import qualified Data.Vector.Unboxed as V

import Intcode

day9 :: String -> IO ()
day9 input = do
  let program = (V.fromList $ map read $ splitOn "," input) :: V.Vector Int
  let (_, outputs, prgState) = runProgram program 0 0 [1]
  let (_, outputs2, prgState2) = runProgram program 0 0 [2]
  guard (outputs == [3429606717] && outputs2 == [33679])
  print (prgState, outputs)
  print (prgState2, outputs2)
