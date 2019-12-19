module Day16 (day16) where

import Control.DeepSeq (($!!))
import Control.Monad (guard)
import qualified Data.Vector as V
import qualified Data.Vector.Unboxed as UV
import Data.Char (ord)

getPattern :: Int -> Int -> UV.Vector (Int, Int, Int)
getPattern len pos = UV.fromList $ go pos 1
  where
    go cur sign
      | cur > len = []
      | otherwise = (cur, min (cur + pos - 1) len, sign) : go (cur + pos * 2)
                                                              (negate sign)

fft :: UV.Vector Int -> Int -> UV.Vector Int
fft signal nIters = go signal nIters
  where
    go sig 0 = sig
    go sig n
      | n > 0 = (go $!! UV.map ((`mod` 10) . abs . compute) positions) (n - 1)
      | otherwise = error "unreachable"
      where
        prefixSums = UV.scanl' (+) 0 sig
        subSum (start, end, sign) =
          sign * ((prefixSums UV.! end) - (prefixSums UV.! (start - 1)))
        compute pos = UV.foldl' (\acc range -> acc + subSum range)
                                0
                                (patterns V.! (pos - 1))
    signalLength = UV.length signal
    positions = UV.fromList [1 .. signalLength]
    patterns = V.fromList $ map (getPattern signalLength) [1 .. signalLength]


day16 :: String -> IO ()
day16 input = do
  let (input':_) = lines input
  let signal = map (\c -> ord c - ord '0') input'
  print $ length signal
  let part1 = UV.toList $ UV.take 8 $ fft (UV.fromList signal) 100
  guard (part1 == [5, 9, 5, 2, 2, 4, 2, 2])
  print part1
  let offset = read (take 7 input') :: Int
  let realSignal = take (length signal * 10000) $ cycle signal
  let part2 = UV.toList $ UV.take 8
                        $ UV.drop offset
                        $ fft (UV.fromList realSignal) 100
  guard (part2 == [1, 8, 6, 5, 0, 8, 3, 4])
  print part2
