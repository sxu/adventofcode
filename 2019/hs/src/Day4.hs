module Day4 (day4) where 

import Data.List.Split (splitOn)
import Data.Monoid (All(..))

hasRepeats :: String -> Bool
hasRepeats (a:b:xs) = a == b || hasRepeats (b:xs)
hasRepeats _ = False

hasExactRepeats :: String -> Bool
hasExactRepeats (a:b:c:xs)
 | a == b && b == c = hasExactRepeats $ dropWhile (== a) xs
 | otherwise = a == b || hasExactRepeats (b:c:xs)
hasExactRepeats [a, b] = a == b
hasExactRepeats _ = False

increasingDigits :: String -> Bool
increasingDigits (a:b:xs) = a <= b && increasingDigits (b:xs)
increasingDigits _ = True

isValid :: [String -> Bool] -> String -> Bool
isValid checks pwd = getAll $ foldMap (\f -> All $ f pwd) checks

day4 :: String -> IO ()
day4 input = do
  let (line:_) = lines input
  (lo, hi) <- case splitOn "-" line of
                [x, y] -> return ((read x, read y) :: (Int, Int))
                _ -> fail $ "Bad input: " ++ line
  print $ sum
        $ map (fromEnum . isValid [hasRepeats, increasingDigits] . show)
              [lo..hi]
  print $ sum
        $ map (fromEnum . isValid [hasExactRepeats, increasingDigits] . show)
              [lo..hi]
