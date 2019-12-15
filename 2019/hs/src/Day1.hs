module Day1 (day1) where

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
  print $ sum $ map (fuel . read) ls
  print $ sum $ map (fuelRec . read) ls
