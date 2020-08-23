{-# LANGUAGE BlockArguments #-}

module Day3 (day3) where

import Control.Monad.State
import Data.Functor
import Data.Maybe (catMaybes)
import qualified Data.Text as T
import qualified Text.Parsec as P
import qualified Text.Parsec.Text as P
import Text.Read (readMaybe)

data Point = Point { xCoord :: Int, yCoord :: Int } deriving (Show, Eq)

data Segment = Horizontal Point Point | Vertical Point Point deriving (Show)

data Piece = Up Int | Dn Int | Lf Int | Rt Int deriving (Show)

segLength :: Segment -> Int
segLength (Horizontal s e) = l1Distance s e
segLength (Vertical s e) = l1Distance s e

l1Distance :: Point -> Point -> Int
l1Distance (Point x1 y1) (Point x2 y2) = abs (x1 - x2) + abs (y1 - y2)

parsePiece :: P.Parser Piece
parsePiece = do
  cons <- parseDirection
  cons <$> parseDistance
  where
    parseDirection = msum [ P.string "U" $> Up
                          , P.string "D" $> Dn
                          , P.string "L" $> Lf
                          , P.string "R" $> Rt
                          ] P.<?> "U/D/L/R"
    parseDistance = do
      s <- P.many1 P.digit 
      case readMaybe s of
        Nothing -> fail $ show s 
        Just v -> return v

followDirections :: [Piece] -> [(Segment, Int)]
followDirections dirs = evalState (go dirs) (Point 0 0, 0) []
  where
    go :: [Piece] -> State (Point, Int) ([(Segment, Int)] -> [(Segment, Int)])
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

intersectionOf :: Segment -> Segment -> Maybe Point
intersectionOf (Horizontal _ _) (Horizontal _ _) = Nothing
intersectionOf (Vertical _ _) (Vertical _ _) = Nothing
intersectionOf seg1 seg2 =
  case seg1 of Horizontal _ _ -> go seg1 seg2
               _ -> go seg2 seg1
  where
    go (Horizontal h1 h2) (Vertical v1 v2) =
      if (vX == 0 && hY == 0)
         || vX < hStart
         || vX > hEnd
         || hY < vStart
         || hY > vEnd
      then Nothing
      else Just $ Point vX hY
      where vX = xCoord v1
            hY = yCoord h1
            hStart = min (xCoord h1) (xCoord h2)
            hEnd = max (xCoord h1) (xCoord h2)
            vStart = min (yCoord v1) (yCoord v2)
            vEnd = max (yCoord v1) (yCoord v2)
    go _ _ = fail "Unreachable"

intersectionCost :: (Segment, Int) -> (Segment, Int) -> Maybe (Int, Int)
intersectionCost (seg1, step1) (seg2, step2) = do
  intersection@(Point x y) <- intersectionOf seg1 seg2
  let extra1 = l1Distance (start seg1) intersection 
  let extra2 = l1Distance (start seg2) intersection
  return (abs x + abs y, step1 + extra1 + step2 + extra2)
  where start (Horizontal p _) = p
        start (Vertical p _) = p
    
day3 :: String -> IO ()
day3 input = do
  let (input1:input2:_) = lines input
  let parser = (parsePiece `P.sepBy` P.string ",") <* P.eof
  let costs = do
        wire1 <- followDirections <$> P.parse parser "" (T.pack input1)
        wire2 <- followDirections <$> P.parse parser "" (T.pack input2)
        return $ catMaybes [intersectionCost p1 p2 | p1 <- wire1, p2 <- wire2]
  case costs of
    Left err -> print err
    Right cs -> do
      let part1 = minimum $ map fst cs
      let part2 = minimum $ map snd cs
      guard (part1 == 870 && part2 == 13698)
