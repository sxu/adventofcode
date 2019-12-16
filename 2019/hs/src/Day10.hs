module Day10 (day10) where

import Control.Monad (guard)
import Data.List (groupBy, maximumBy, sortBy)
import qualified Data.HashSet as S

asteroidsCoordinates :: [String] -> S.HashSet (Int, Int)
asteroidsCoordinates asteroidsMap =
  foldMap processRow $ zip [0..] asteroidsMap
  where
    processRow :: (Int, String) -> S.HashSet (Int, Int)
    processRow (y, row) = foldMap (look y) $ zip [0..] row
    look :: Int -> (Int, Char) -> S.HashSet (Int, Int)
    look y (x, '#') = S.singleton (x, y)
    look _ (_, '.') = S.empty
    look _ (_, v) = error $ "invalid map value: " ++ [v]

getDirection :: (Int, Int) -> (Int, Int) -> (Int, Int)
getDirection (x, y) (x', y') = (dx `div` step, dy `div` step)
  where dx = x' - x
        dy = y' - y
        step = gcd dx dy

data Station = Station { stationLocation :: (Int, Int)
                       , numAsteroidsInSight :: Int
                       } deriving (Show)

evaluate :: S.HashSet (Int, Int) -> (Int, Int) -> Station
evaluate coords (x, y) =
  Station { stationLocation = (x, y)
          , numAsteroidsInSight = S.size $ S.map (getDirection (x, y))
                                $ S.filter (/= (x, y)) coords
          }

data Asteroid = Asteroid { asteroidLocation :: (Int, Int)
                         , directionToStation :: (Int, Int)
                         , distanceToStation :: Double
                         } deriving (Show)

vaporizations :: S.HashSet (Int, Int) -> (Int, Int) -> [Asteroid]
vaporizations coords (x, y) = rotateLaser asteroidsSortedByDir
  where
    rotateLaser :: [[Asteroid]] -> [Asteroid]
    rotateLaser [] = []
    rotateLaser asByDir =
      map head asByDir ++ rotateLaser (filter (not . null) $ map tail asByDir)
    asteroidsSortedByDir = map (sortBy cmpDist) $ groupBy sameDir
                                                $ sortBy cmpDir asteroids
    asteroids = map (\c -> Asteroid c (getDirection (x, y) c) (getDistance2 c))
                    (S.toList $ S.delete (x, y) coords)
    sameDir (Asteroid _ d1 _) (Asteroid _ d2 _) = d1 == d2
    cmpDir (Asteroid _ d1@(dx1, dy1) _) (Asteroid _ d2@(dx2, dy2) _)
      | d1 == d2 = EQ
      | dx1 == 0 && dx2 == 0 = compare dy1 dy2
      | dx1 == 0 = if dy1 < 0 then LT else if dx2 > 0 then GT else LT
      | dx2 == 0 = if dy2 < 0 then GT else if dx1 > 0 then LT else GT
      | (dx1 > 0) && (dx2 > 0) = compare a2 a1
      | (dx1 < 0) && (dx2 < 0) = compare a1 a2
      | otherwise = compare dx2 dx1
      where a1 = angle dx1 dy1 -- always <= pi
            a2 = angle dx2 dy2
    cmpDist (Asteroid _ _ d1) (Asteroid _ _ d2) = compare d1 d2
    angle x' y' = acos $ fy / sqrt (fx * fx + fy * fy) :: Double
      where fx = fromIntegral x' 
            fy = fromIntegral y'
    getDistance2 (x', y') = dx * dx + dy * dy :: Double
      where dx = fromIntegral (x' - x)
            dy = fromIntegral (y' - y)

day10 :: String -> IO ()
day10 input = do
  let coords = asteroidsCoordinates $ lines input
  let best = maximumBy cmpStation $ map (evaluate coords)
                                  $ S.toList coords
  let vaporized = map asteroidLocation $ vaporizations coords
                                                       (stationLocation best)
  guard (numAsteroidsInSight best == 230 && vaporized !! 199 == (12, 5))
  print best
  print $ vaporized !! 199
  where
    cmpStation s1 s2 = compare (numAsteroidsInSight s1) (numAsteroidsInSight s2)
