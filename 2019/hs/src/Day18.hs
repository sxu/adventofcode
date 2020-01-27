{-# LANGUAGE FlexibleContexts #-}
{-# LANGUAGE TupleSections #-}

module Day18 where

import Control.Monad.Cont
import Control.Monad.State.Strict
import Control.Monad.Writer.Strict
import Data.Char (isLetter, isLower, isUpper, toLower, toUpper)
import Data.Maybe (fromMaybe)
import Data.Hashable (Hashable(..))
import qualified Data.HashMap.Strict as M
import qualified Data.HashPSQ as PSQ
import qualified Data.HashSet as S
import Data.Sequence (Seq(..), (><))
import qualified Data.Sequence as Seq
import qualified Data.Vector as V
import qualified Data.Vector.Unboxed as UV

data Node = Start
          | Key Char
          | Door Char
          deriving (Eq, Show, Ord)

instance Hashable Node where
  hashWithSalt salt Start = hashWithSalt salt '@'
  hashWithSalt salt (Key c) = hashWithSalt salt $ toLower c
  hashWithSalt salt (Door c) = hashWithSalt salt $ toUpper c

type Vault = V.Vector (UV.Vector Char)
type Graph = M.HashMap Node (M.HashMap Node Int)

index :: Vault -> (Int, Int) -> Char
index vault (x, y) = fromMaybe '#' $ (vault V.!? y) >>= (UV.!? x)
                      
nodeFromChar :: Char -> Node
nodeFromChar c | isLower c = Key c
               | isUpper c = Door c
               | c == '@' = Start
               | otherwise = error "unreachable"

findNodes :: Vault -> [(Node, (Int, Int))]
findNodes vault = foldMap doRow [0 .. V.length vault - 1]
  where
    doRow y = foldMap (doCol y) [0 .. UV.length (vault V.! y) - 1]
    doCol y x
      | isLetter c || c == '@' = [(nodeFromChar c, (x, y))]
      | otherwise = []
      where c = index vault (x, y)

findNeighbors :: Vault -> (Int, Int) -> M.HashMap Node Int
findNeighbors vault nodePos = 
  execWriter $ execStateT (bfs $ Seq.singleton (nodePos, 0))
                          (S.singleton nodePos)
  where
    bfs Empty = return ()
    bfs ((pos, dis) :<| qs)
      | cur == '.' || dis == 0 = continue
      | otherwise = case nodeFromChar cur of
                      Start -> continue
                      node -> tell (M.singleton node dis) >> stop
      where cur = index vault pos
            continue = explore pos dis >>= bfs . (qs ><)
            stop = bfs qs
    explore pos dis = do
      discovered <- get
      let new = filter (\p -> (&&) (index vault p /= '#') 
                                   (not $ S.member p discovered))
                       (adjs pos)
      put $ discovered <> S.fromList new
      return $ Seq.fromList $ map (, dis + 1) new
    adjs (x, y) = [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]

createGraph :: Vault -> [(Node, (Int, Int))] -> Graph
createGraph vault nodeLocations =
  foldMap (\(n, pos) -> M.singleton n (findNeighbors vault pos))
          nodeLocations

findDistanceToKeys :: Graph -> Node -> M.HashMap Node Int
findDistanceToKeys graph source =
  execWriter $ execStateT (dijkstra $ PSQ.singleton source 0 ())
                          (S.singleton source)
  where
    dijkstra queue = do
      finished <- get
      case PSQ.findMin queue of
        Nothing -> return ()
        Just (node, dis, ()) ->
          if S.member node finished
          then dijkstra $ PSQ.deleteMin queue
          else do
            when (isKey node) $ tell $ M.singleton node dis
            let newQueue = explore node dis finished queue
            dijkstra newQueue
    explore node dis finished queue = foldl updateQueue queue neighbors
      where
        neighbors = M.toList $ fromMaybe M.empty $ M.lookup node graph
        updateQueue q (n, d) =
          if S.member n finished then q
          else let (_, q') = PSQ.alter (update (d + dis)) n q in q'
        update d Nothing = ((), Just (d, ()))
        update d (Just (d', ())) = ((), Just (min d d', ()))
    isKey (Key _) = True
    isKey _ = False

day18 :: String -> IO ()
day18 input = do
  let vault = V.fromList $ map UV.fromList $ lines input
  let graph = createGraph vault $ findNodes vault
  print $ M.filterWithKey
          (\k _ -> case k of Door _ -> False
                             _ -> True)
          graph
  print $ M.elems $ M.map M.size graph
