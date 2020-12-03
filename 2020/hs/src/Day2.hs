module Day2 (day2) where

import Control.Monad (guard)
import Data.List.Split (splitOn)

data Policy = Policy Int Int Char

parseInput :: String -> (Policy, String)
parseInput input = (Policy lower upper char, password)
  where splitInput = splitOn ": " input
        policy = head splitInput
        password = splitInput !! 1
        splitPolicy = splitOn " " policy
        range = head splitPolicy
        char = head (splitPolicy !! 1)
        splitRange = splitOn "-" range
        lower = read $ head splitRange :: Int
        upper = read (splitRange !! 1) :: Int

isValid1 :: Policy -> String -> Bool
isValid1 (Policy lower upper char) password = lower <= count && count <= upper
  where count = length $ filter (== char) password

isValid2 :: Policy -> String -> Bool
isValid2 (Policy lower upper char) password = checkPos lower /= checkPos upper
  where checkPos pos = password !! (pos - 1) == char

day2 :: String -> IO ()
day2 input = do
  let inputs = map parseInput $ lines input
  let numValid1 = length $ filter (uncurry isValid1) inputs
  let numValid2 = length $ filter (uncurry isValid2) inputs
  guard (numValid1 == 500)
  guard (numValid2 == 313)
