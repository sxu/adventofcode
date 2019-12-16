module Day13 where

import Control.Monad.State
import qualified  Data.HashMap.Strict as H
import Data.List.Split (splitOn)
import qualified Data.Vector.Unboxed as V
import System.IO

import Intcode

data Tile = Empty | Wall | Block | Paddle | Ball deriving (Enum, Eq, Show)

data GameState = GameState
               { gameTiles :: H.HashMap (Int, Int) Tile
               , gameScore :: Int
               , paddlePosition :: (Int, Int)
               , ballPosition :: (Int, Int)
               }

update :: [Int] -> GameState -> GameState
update ((-1):0:s:is) (GameState ts _ p b) = update is (GameState ts s p b)
update (x:y:tile:is) (GameState ts s p b) =
  update is (GameState (H.insert (x, y) (toEnum tile) ts) s p' b')
  where p' = if tile == 3 then (x, y) else p
        b' = if tile == 4 then (x, y) else b
update [] s = s
update _ _ = error "unreachable"

paint :: GameState -> IO ()
paint (GameState tiles score _ _) = do
  putStrLn ""
  let screen = map (\(pos, tile) -> (pos, toChar tile)) $ H.toList tiles
  let (maxX, maxY) = foldr (\((x, y), _) (x', y') -> (max x x', max y y'))
                           (0, 0)
                           screen
  let blank = V.replicate ((maxX + 1) * (maxY + 1)) ' '
  let painted = blank V.// map (\((x, y), c) -> (x + y * (maxX + 1), c)) screen
  forM_ [maxY, (maxY - 1) .. 0]
        (\y -> putStrLn $ V.toList
                        $ V.slice (y * (maxX + 1)) (maxX + 1) painted)
  putStrLn $ "Score: " ++ show score
  where
    toChar Empty = ' '
    toChar Wall = '#'
    toChar Block = 'O'
    toChar Paddle = '-'
    toChar Ball = '*'

play :: V.Vector Int -> ProgramState -> State GameState ()
play _ Halted = return ()
play ram (Running pc rb) = do
  let (ram', outputs, prgState) = runProgramUnsafe ram pc rb []
  modify $ update outputs
  play ram' prgState
play ram (WaitingForInput pc rb) = do
  GameState _ _ paddle ball <- get
  let input = case compare (fst paddle) (fst ball) of EQ -> 0
                                                      LT -> 1
                                                      GT -> -1
  let (ram', outputs, prgState) = runProgramUnsafe ram pc rb [input]
  modify $ update outputs
  play ram' prgState

day13 :: String -> IO ()
day13 input = do
  hSetBuffering stdout NoBuffering
  let code = (map read $ splitOn "," input) :: [Int]
  let (_, outputs, _) = runProgram (V.fromList code) 0 0 []
  let numBlocks = countBlocks outputs :: Int
  guard (numBlocks == 247)
  print numBlocks
  let end = execState (play (V.fromList (2 : tail code)) $ Running 0 0) 
                      (GameState H.empty 0 (0, 0) (0, 0))
  print $ gameScore end
  where
    countBlocks = go 0
      where go count (_:_:2:os) = go (count + 1) os
            go count (_:_:_:os) = go count os
            go count [] = count
            go _ _ = error "unreachable"
