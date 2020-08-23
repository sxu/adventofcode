{-# LANGUAGE TemplateHaskell #-}

module Day11 (day11) where

import Control.Lens
import Control.Monad.State
import qualified Data.HashMap.Strict as H
import Data.List.Split (splitOn)
import qualified Data.Vector.Unboxed as V

import Intcode

data Direction = Up | Dn | Lf | Rt deriving (Show)

data RobotState = RobotState
                { _robotPosition :: (Int, Int)
                , _robotDirection :: Direction
                , _shipPanels :: H.HashMap (Int, Int) Int
                } deriving (Show)

makeLenses ''RobotState

robot :: V.Vector Int -> ProgramState -> State RobotState ()
robot _ Halted = return ()
robot ram (Running pc rb) = do
  let (ram', outputs, prgState) = runProgram ram pc rb []
  performActions outputs
  robot ram' prgState
robot ram (WaitingForInput pc rb) = do
  pos <- gets $ view robotPosition
  panels <- gets $ view shipPanels
  let color = H.lookupDefault 0 pos panels
  let (ram', outputs, prgState) = runProgram ram pc rb [color]
  performActions outputs
  robot ram' prgState

performActions :: [Int] -> State RobotState ()
performActions [] = return ()
performActions (color:turnDir:outputs) = do
  pos <- gets $ view robotPosition
  dir <- gets $ view robotDirection
  modify $ over shipPanels (H.insert pos color)
  let newDir = turn turnDir dir
  let newPos = advance pos newDir
  modify $ set robotDirection newDir
  modify $ set robotPosition newPos
  performActions outputs
  where
    turn x = if x == 0 then turnLeft else turnRight
    turnLeft Up = Lf
    turnLeft Dn = Rt
    turnLeft Lf = Dn
    turnLeft Rt = Up
    turnRight Up = Rt
    turnRight Dn = Lf
    turnRight Lf = Up
    turnRight Rt = Dn
    advance (x, y) Up = (x, y + 1)
    advance (x, y) Dn = (x, y - 1)
    advance (x, y) Lf = (x - 1, y)
    advance (x, y) Rt = (x + 1, y)
performActions _ = error "unreachable"

paintPanels :: H.HashMap (Int, Int) Int -> IO ()
paintPanels panels = do
  let whitePanels = H.keys $ H.filter (== 1) panels
  let (minX, minY) = foldr (\(x, y) (x', y') -> (min x x', min y y'))
                           (0, 0)
                           whitePanels
  let translated = map (\(x, y) -> (x - minX, y - minY)) whitePanels
  let (maxX, maxY) = foldr (\(x, y) (x', y') -> (max x x', max y y'))
                           (0, 0)
                           translated
  let blank = V.replicate ((maxX + 1) * (maxY + 1)) ' '
  let painted = blank V.// map (\(x, y) -> (x + y * (maxX + 1), '#')) translated
  forM_ [maxY, (maxY - 1) .. 0]
        (\y -> putStrLn $ V.toList
                        $ V.slice (y * (maxX + 1)) (maxX + 1) painted)

day11 :: String -> IO ()
day11 input = do
  let program = (V.fromList $ map read $ splitOn "," input) :: V.Vector Int
  let RobotState _ _ panels1 = execState (robot program (Running 0 0)) 
                                         (RobotState (0, 0) Up H.empty)
  guard (H.size panels1 == 1967)
  print $ H.size panels1
  let RobotState _ _ panels2 = execState (robot program (Running 0 0)) 
                                         (RobotState (0, 0)
                                                     Up 
                                                     (H.singleton (0, 0) 1))
  paintPanels panels2
