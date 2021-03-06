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
import Day10
import Day11
import Day12
import Day13
import Day14
import Day15
import Day16
import Day17
import Day18

days :: [String -> IO ()]
days = [ day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11
       , day12, day13, day14, day15, day16, day17, day18
       ]

main :: IO ()
main = do
  argv <- getArgs
  case argv of
    [] -> sequence_ $ map runDay [1..18]
    argv' -> do let day = (read $ head argv') :: Int
                runDay day

runDay :: Int -> IO ()
runDay day = do
  putStr $ "Day " ++ (show day) ++ "..."
  input <- readFile $ "../input" ++ show day
  (days !! (day - 1)) input
  putStrLn $ " OK"
