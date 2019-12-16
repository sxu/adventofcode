module Day12 (day12) where

import Control.Applicative (liftA2, ZipList(..))
import Control.Monad (guard)
import Data.Hashable (Hashable(..))
import Data.List (transpose)
import qualified Data.HashMap.Strict as H
import qualified Data.Text as T
import qualified Text.Parsec as P
import qualified Text.Parsec.Text as P

data Moon1D = Moon1D { moonPosition :: Int
                     , moonVelocity :: Int
                     } deriving (Show, Eq)

instance Hashable Moon1D where
  hashWithSalt salt (Moon1D p v) = hashWithSalt salt (p, v)

parseMoon :: P.Parser (ZipList Moon1D)
parseMoon = do
  (x, y, z) <- P.between (P.string "<") (P.string ">") parsePosition
  return $ ZipList [Moon1D x 0, Moon1D y 0, Moon1D z 0]
  where parsePosition :: P.Parser (Int, Int, Int)
        parsePosition = do
          x1 <- P.string "x=" *> P.choice [P.char '-', P.digit]
          xs <- P.many P.digit <* P.string "," <* P.spaces
          y1 <- P.string "y=" *> P.choice [P.char '-', P.digit]
          ys <- P.many P.digit <* P.string "," <* P.spaces
          z1 <- P.string "z=" *> P.choice [P.char '-', P.digit]
          zs <- P.many P.digit
          return (read (x1:xs), read (y1:ys), read (z1:zs))

applyGravity :: Moon1D -> Moon1D -> Moon1D
applyGravity (Moon1D x' _) (Moon1D x vx) = Moon1D x (vx + pull x' x)
  where pull a' a | a' == a = 0
                  | a' > a = 1
                  | a' < a = -1
                  | otherwise = error "unreachable"

applyVelocity :: Moon1D -> Moon1D
applyVelocity (Moon1D x vx) = Moon1D (x + vx) vx

simulateOneStep :: [ZipList Moon1D] -> [ZipList Moon1D]
simulateOneStep moons = map (fmap applyVelocity . updateVelocity) moons
  where
    updateVelocity moon = foldr (liftA2 applyGravity) moon moons

totalEnergy :: [ZipList Moon1D] -> Int
totalEnergy moons = sum $ map energy moons
  where
    energy moon = potential * kinetic
      where 
        dims = getZipList moon 
        potential = sum (map (abs . moonPosition) dims)
        kinetic = sum (map (abs . moonVelocity) dims)

simulateOneStep1D :: [Moon1D] -> [Moon1D]
simulateOneStep1D moons = map (applyVelocity . updateVelocity) moons
  where
    updateVelocity moon = foldr applyGravity moon moons

findCycle :: [[Moon1D]] -> H.HashMap [Moon1D] Int -> Int -> (Int, Int)
findCycle (state:states) history current =
  case H.lookup state history of
    Nothing -> findCycle states (H.insert state current history) (current + 1)
    Just time -> (time, current)
findCycle [] _ _ = error "unreachable"

day12 :: String -> IO ()
day12 input =
  case P.parse (parseMoon `P.sepEndBy` P.endOfLine) "" $ T.pack input of
    Left err -> print err
    Right moons -> do
      let energy = totalEnergy $ iterate simulateOneStep moons !! 1000
      guard (energy == 6423)
      print energy
      case transpose $ map getZipList moons of
        [xs, ys, zs] -> do
          let (x1, x2) = findCycle (iterate simulateOneStep1D xs) H.empty 0
          let (y1, y2) = findCycle (iterate simulateOneStep1D ys) H.empty 0
          let (z1, z2) = findCycle (iterate simulateOneStep1D zs) H.empty 0
          let period = lcm (x2 - x1) $ lcm (y2 - y1) (z2 - z1)
          guard (period == 327636285682704 && x1 == 0 && y1 == 0 && z1 == 0)
          print period
        _ -> error "unreachable"
