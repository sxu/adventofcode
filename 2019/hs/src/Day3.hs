{-# LANGUAGE BlockArguments #-}

module Day3 (day3) where

import Control.Exception (assert)
import Control.Monad.State
import Data.Functor
import qualified Data.Text as T
import qualified Text.Parsec as P
import qualified Text.Parsec.Text as P
import Text.Read (readMaybe)

data Point = Point { xCoord :: Int, yCoord :: Int } deriving (Show, Eq)

data Segment = Horizontal Point Point | Vertical Point Point deriving (Show)

data Direction = Up Int | Dn Int | Lf Int | Rt Int deriving (Show)

segLength :: Segment -> Int
segLength (Horizontal s e) = l1Distance s e
segLength (Vertical s e) = l1Distance s e

l1Distance :: Point -> Point -> Int
l1Distance (Point x1 y1) (Point x2 y2) = abs (x1 - x2) + abs (y1 - y2)

parseDirection :: P.Parser Direction
parseDirection = do
  cons <- parseCons
  cons <$> parseDistance
  where
    parseCons = msum [ P.string "U" $> Up
                     , P.string "D" $> Dn
                     , P.string "L" $> Lf
                     , P.string "R" $> Rt
                     ] P.<?> "U/D/L/R"
    parseDistance = do
      s <- P.many1 P.digit 
      case readMaybe s of
        Nothing -> fail $ show s 
        Just v -> return v

followDirections :: [Direction] -> [(Segment, Int)]
followDirections dirs = evalState (go dirs) (Point 0 0, 0) []
  where
    go :: [Direction] -> State (Point, Int) ([(Segment, Int)] -> [(Segment, Int)])
    go [] = return id
    go (d:ds) = do
      (cur, dis) <- get
      let (new, seg) = follow d cur
      put (new, dis + segLength seg)
      rest <- go ds
      return $ ((seg, dis):) . rest
    follow (Up dis) p@(Point x y) = (new, Vertical p new)
      where new = Point x (y + dis)
    follow (Dn dis) p@(Point x y) = (new, Vertical p new)
      where new = Point x (y - dis)
    follow (Lf dis) p@(Point x y) = (new, Horizontal p new)
      where new = Point (x - dis) y
    follow (Rt dis) p@(Point x y) = (new, Horizontal p new)
      where new = Point (x + dis) y

intersections :: Segment -> Segment -> [Point]
intersections seg1 seg2 = filter (/= Point 0 0) $ go seg1 seg2
  where
    go (Horizontal s1 e1) (Horizontal s2 e2) =
      map (\x -> Point x $ yCoord s1) xs
      where xs = goParallel s1 e1 s2 e2 xCoord yCoord
    go (Vertical s1 e1) (Vertical s2 e2) =
      map (Point (xCoord s1)) ys
      where ys = goParallel s1 e1 s2 e2 yCoord xCoord
    go seg1' seg2' =
      case seg1' of Horizontal _ _ -> goPerpendicular seg1' seg2'
                    _ -> goPerpendicular seg2' seg1'
    goParallel pStart pEnd qStart qEnd getEnd getLevel
      | getLevel pStart /= getLevel qStart = []
      | otherwise =
          if start1 <= start2
          then if start2 <= end1 then [start2, min end1 end2] else []
          else if start1 <= end2 then [start1, min end1 end2] else []
      where start1 = min (getEnd pStart) (getEnd pEnd)
            end1 = max (getEnd pStart) (getEnd pEnd)
            start2 = min (getEnd qStart) (getEnd qEnd)
            end2 = max (getEnd qStart) (getEnd qEnd)
    goPerpendicular (Horizontal h1 h2) (Vertical v1 v2) =
      [Point vX hY | not (vX < hStart || vX > hEnd || hY < vStart || hY > vEnd)]
      where vX = xCoord v1
            hY = yCoord h1
            hStart = min (xCoord h1) (xCoord h2)
            hEnd = max (xCoord h1) (xCoord h2)
            vStart = min (yCoord v1) (yCoord v2)
            vEnd = max (yCoord v1) (yCoord v2)
    goPerpendicular _ _ = fail "unreachable"

intersectionCost :: (Segment, Int) -> (Segment, Int) -> Point -> (Int, Int)
intersectionCost (seg1, step1) (seg2, step2) intersection@(Point x y) =
  (abs x + abs y, step1 + extra1 + step2 + extra2)
  where
    extra1 = l1Distance (start seg1) intersection 
    extra2 = l1Distance (start seg2) intersection
    start (Horizontal p _) = p
    start (Vertical p _) = p
    
day3 :: String -> IO ()
day3 input = do
  let (input1:input2:_) = lines input
  let parser = (parseDirection `P.sepBy` P.string ",") <* P.eof
  let costs = do
        wire1 <- followDirections <$> P.parse parser "" (T.pack input1)
        wire2 <- followDirections <$> P.parse parser "" (T.pack input2)
        return $ mconcat [ map (intersectionCost p1 p2) $ intersections seg1 seg2
                         | p1@(seg1, _) <- wire1, p2@(seg2, _) <- wire2
                         ]
  case costs of
    Left err -> print err
    Right cs -> do
      let part1 = minimum $ map fst cs
      let part2 = minimum $ map snd cs
      assert (part1 == 870 && part2 == 13698) $ return ()
      print part1
      print part2
