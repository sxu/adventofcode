module Main where

import System.Environment   

import Day1
import Day2

days :: [String -> IO ()]
days = [day1, day2]

main :: IO ()
main = do
  argv <- getArgs
  case argv of
    [] -> sequence_ $ map runDay [1]
    argv' -> do let day = (read $ head argv') :: Int
                runDay day

runDay :: Int -> IO ()
runDay day = do
  putStr $ "Day " ++ (show day) ++ "..."
  input <- readFile $ "../input" ++ show day
  (days !! (day - 1)) input
  putStrLn $ " OK"
