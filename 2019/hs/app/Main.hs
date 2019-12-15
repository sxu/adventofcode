module Main where

import System.Environment   

import Day1
import Day2
import Day3
import Day4
import Day5
import Day6
import Day7
import Day8
import Day9

days :: [String -> IO ()]
days = [day1, day2, day3, day4, day5, day6, day7, day8, day9]

main :: IO ()
main = do
  argv <- getArgs
  case argv of
    [] -> print "USAGE: aoc19 [DAY]"
    argv' -> do let day = (read $ head argv') :: Int
                input <- readFile $ "../input" ++ show day
                (days !! (day - 1)) input
