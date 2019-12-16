module Day1 (day1) where

import Control.Monad (guard)

fuel :: Int -> Int
fuel mass = mass `div` 3 - 2

fuelRec :: Int -> Int
fuelRec = go 0
  where
    go acc 0 = acc
    go acc new =
      let f = max 0 $ fuel new
      in go (acc + f) f

day1 :: String -> IO ()
day1 input = do
  let ls = lines input
  let part1 = sum $ map (fuel . read) ls
  let part2 = sum $ map (fuelRec . read) ls
  guard (part1 == 3455717 && part2 == 5180690)
  print part1
  print part2
