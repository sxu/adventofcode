module Day1 (day1) where

import Control.Monad (guard)
import qualified Data.HashSet as S

findProduct2 :: Int -> [Int] -> S.HashSet Int -> Maybe Int
findProduct2 target (x:xs) set
  | S.member (target - x) set = Just $ x * (target - x)
  | otherwise = findProduct2 target xs set
findProduct2 _ [] _ = Nothing

findProduct3 :: Int -> [Int] -> S.HashSet Int -> Maybe Int
findProduct3 target (x:xs) set = 
   case findProduct2 (target - x) xs set of
     Nothing -> findProduct3 target xs set
     Just y -> Just $ x * y
findProduct3 _ [] _ = Nothing

day1 :: String -> IO ()
day1 input = do
  let xs = (map read $ lines input) :: [Int]
  let set = S.fromList xs
  guard (findProduct2 2020 xs set == Just 437931)
  guard (findProduct3 2020 xs set == Just 157667328)
