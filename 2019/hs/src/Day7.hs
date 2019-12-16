module Day7 (day7) where

import Control.Monad.ST
import Control.Monad.State
import qualified Data.List as L
import Data.List.Split (splitOn)
import qualified Data.Vector as V
import qualified Data.Vector.Mutable as MV
import qualified Data.Vector.Unboxed as UV
import qualified Data.Vector.Unboxed.Mutable as UMV

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

runFeedbackLoop :: UV.Vector Int -> Int -> [Int] -> Int
runFeedbackLoop program input [p0, p1, p2, p3, p4] =
  head $ evalState feedbackLoop initState
  where
    (prg0, prg1, prg2, prg3, prg4) =
      runST $ (,,,,) <$> copy <*> copy <*> copy <*> copy <*> copy
      where copy :: ST s (UV.Vector Int)
            copy = do new <- UMV.new $ UV.length program
                      UV.copy new program
                      UV.freeze new
    initState = FeedbackLoopState { lastAmp = 4
                                  , nextAmp = 1
                                  , nextInputs = []
                                  , amplifierStates = V.fromList 
                                      [ (prg0, [p0, input], Running 0 0)
                                      , (prg1, [p1], Running 0 0)
                                      , (prg2, [p2], Running 0 0)
                                      , (prg3, [p3], Running 0 0)
                                      , (prg4, [p4], Running 0 0)
                                      ]
                                  }
    feedbackLoop = do
      FeedbackLoopState lst nxt nxtInputs ampStates <- get
      (ram, inputs, prgState) <- gets $ (V.! nxt) . amplifierStates
      let (pc, rb) = case prgState of
                 Running pc' rb' -> (pc', rb')
                 WaitingForInput pc' rb' -> (pc', rb')
                 Halted -> error "unreachable"
      let (ram', outputs, prgState') = 
            runProgramUnsafe ram pc rb (inputs ++ nxtInputs)
      let newAmpStates =
            V.modify (\v -> MV.write v nxt (ram', [], prgState')) ampStates
      let update s = s { nextAmp = (nxt + 1) `mod` (lst + 1)
                       , nextInputs = outputs
                       , amplifierStates = newAmpStates
                       }
      case prgState' of
        WaitingForInput _ _ -> modify update >> feedbackLoop
        Halted -> if nxt == lst
                  then return outputs
                  else modify update >> feedbackLoop
        Running _ _ -> error "unreachable"
runFeedbackLoop _ _ _ = error "need exactly 5 phases"

day7 :: String -> IO ()
day7 input = do
  let program = (UV.fromList $ map read $ splitOn "," input) :: UV.Vector Int
  let allPhases = L.permutations [0..4]
  let allRuns = map (\ps -> (ps, runAmplifiers program 0 ps)) allPhases
  let part1@(_, s1) = L.maximumBy (\a b -> compare (snd a) (snd b)) allRuns
  let allPhases2 = L.permutations [5..9]
  let allRuns2 = map (\ps -> (ps, runFeedbackLoop program 0 ps)) allPhases2 
  let part2@(_, s2) = L.maximumBy (\a b -> compare (snd a) (snd b)) allRuns2
  guard (s1 == 206580 && s2 == 2299406)
  print part1
  print part2
