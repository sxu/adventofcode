module Day16 (day16) where

import Control.Monad (guard)
import qualified Data.Vector as V
import qualified Data.Vector.Unboxed as UV
import Data.Char (ord)

-- The pattern consists of repeating subsequences of 1, 0, and -1. Each
-- subsequence is represented by (start index, end index, value), indices are
-- inclusive and start at 1. Zeros are ommitted.
getPattern :: Int -> Int -> Int -> UV.Vector (Int, Int, Int)
getPattern len offset pos = UV.fromList $ applyOffset $ go pos 1
  where
    go cur sign
      | cur > len = []
      | otherwise = (cur, min (cur + pos - 1) len, sign) : go (cur + pos * 2)
                                                              (negate sign)
    applyOffset [] = []
    applyOffset ((start, end, val):xs)
      | end <= offset = applyOffset xs
      | start <= offset = (1, end - offset, val) : applyOffset xs
      | otherwise = (start - offset, end - offset, val) : applyOffset xs

fft :: UV.Vector Int -> Int -> Int -> UV.Vector Int
fft signal nIters offset = go sigTail nIters
  where
    sigTail = UV.drop offset signal
    positions = UV.fromList [1..(UV.length sigTail)]
    go sig 0 = sig
    go sig n
      | n > 0 = go (UV.map ((`mod` 10) . abs . compute) positions) (n - 1)
      | otherwise = error "unreachable"
      where
        prefixSums = UV.scanl' (+) 0 sig
        subSum (start, end, sign) =
          sign * ((prefixSums UV.! end) - (prefixSums UV.! (start - 1)))
        compute pos = UV.foldl' (\acc range -> acc + subSum range)
                                0
                                (patterns V.! (pos - 1))
    signalLength = UV.length signal
    patterns = V.fromList $ map (getPattern signalLength offset)
                                [(offset + 1) .. signalLength]


day16 :: String -> IO ()
day16 input = do
  let (input':_) = lines input
  let signal = map (\c -> ord c - ord '0') input'
  let part1 = UV.toList $ UV.take 8 $ fft (UV.fromList signal) 100 0
  guard (part1 == [5, 9, 5, 2, 2, 4, 2, 2])
  let offset = read (take 7 input') :: Int
  let realSignal = take (length signal * 10000) $ cycle signal
  let part2 = UV.toList $ UV.take 8
                        -- $ UV.drop offset
                        $ fft (UV.fromList realSignal) 100 offset
  guard (part2 == [1, 8, 6, 5, 0, 8, 3, 4])
