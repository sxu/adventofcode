module Day8 (day8) where

import Control.Applicative
import Control.Monad
import Data.List

day8 :: String -> IO ()
day8 input = do
  let (line:_) = lines input
  let img = map (\c -> fromEnum c - fromEnum '0') line :: [Int]
  let layers = everyN (height * width) img
  let (layer, _) = minimumBy (\x y -> compare (snd x) (snd y))
                 $ map (\l -> (l, numOf 0 l)) layers
  print $ numOf 1 layer * numOf 2 layer
  let rendered = getZipList $ foldl1' (liftA2 overlay) $ map ZipList layers
  let rows = everyN width rendered
  forM_ rows $ \row -> forM_ row (\p -> putStr $ if p == 1 then "X" else " ")
                    >> putStr "\n"
  where
    height = 6
    width = 25
    everyN _ [] = []
    everyN n img = 
      let (layer, rest) = splitAt n img
      in layer : everyN n rest
    numOf x = length . filter (== x)
    overlay 0 _ = 0
    overlay 1 _ = 1
    overlay 2 p = p
    overlay _ _ = error "invalid pixel value"
