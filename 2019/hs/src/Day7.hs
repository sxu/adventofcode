module Day7 (day7) where

import Control.Monad.State
import Data.Maybe (listToMaybe)
import qualified Data.List as L
import Data.List.Split (splitOn)
import qualified Data.Vector as V
import qualified Data.Vector.Mutable as MV
import qualified Data.Vector.Unboxed as UV

import Intcode

runAmplifiers :: UV.Vector Int -> Int -> [Int] -> Int
runAmplifiers program input (phase:ps) = 
  case prgState of
    Halted -> runAmplifiers program output ps
    x -> error $ show x
  where (_, [output], prgState) = runProgram program 0 0 [phase, input]
runAmplifiers _ input [] = input


data FeedbackLoopState = FeedbackLoopState
  { lastAmp :: Int
  , nextAmp :: Int
  , nextInputs :: [Int]
  , amplifierStates :: V.Vector (UV.Vector Int, [Int], ProgramState)
  }

runFeedbackLoop :: UV.Vector Int -> Int -> [Int] -> Maybe Int
runFeedbackLoop program input [p0, p1, p2, p3, p4] =
  listToMaybe $ evalState feedbackLoop loopState
  where
    loopState = FeedbackLoopState { lastAmp = 4
                                  , nextAmp = 1
                                  , nextInputs = []
                                  , amplifierStates = V.fromList 
                                      [ (program, [p0, input], Running 0 0)
                                      , (program, [p1], Running 0 0)
                                      , (program, [p2], Running 0 0)
                                      , (program, [p3], Running 0 0)
                                      , (program, [p4], Running 0 0)
                                      ]
                                  }
    feedbackLoop = do
      FeedbackLoopState lst nxt nxtInputs ampStates <- get
      (ram, inputs, prgState) <- gets $ (V.! nxt) . amplifierStates
      let (pc, rb) = case prgState of
                 Running pc' rb' -> (pc', rb')
                 WaitingForInput pc' rb' -> (pc', rb')
                 Halted -> error "unreachable"
      let (ram', outputs, prgState') = runProgram ram pc rb (inputs ++ nxtInputs)
      let newAmpStates =
            V.modify (\v -> MV.write v nxt (ram', [], prgState')) ampStates
      let update s = s { nextAmp = (nxt + 1) `mod` (lst + 1)
                       , nextInputs = reverse outputs
                       , amplifierStates = newAmpStates
                       }
      case prgState' of
        WaitingForInput _ _ -> modify update >> feedbackLoop
        Halted -> if nxt == lst
                  then return $ reverse outputs
                  else modify update >> feedbackLoop
        Running _ _ -> error "unreachable"
runFeedbackLoop _ _ _ = error "need exactly 5 phases"

day7 :: String -> IO ()
day7 input = do
  let program = (UV.fromList $ map read $ splitOn "," input) :: UV.Vector Int
  let allPhases = L.permutations [0..4]
  print $ L.maximumBy (\a b -> compare (snd a) (snd b))
        $ map (\ps -> (ps, runAmplifiers program 0 ps)) allPhases 
  let allPhases2 = L.permutations [5..9]
  print $ L.maximumBy (\a b -> compare (snd a) (snd b))
        $ map (\ps -> (ps, runFeedbackLoop program 0 ps)) allPhases2 
