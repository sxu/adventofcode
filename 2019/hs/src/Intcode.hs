{-# LANGUAGE FlexibleContexts #-}
module Intcode (runProgram, ProgramState(..)) where

import Control.Monad.ST
import Control.Monad.State.Strict
import Data.Maybe (listToMaybe)
import qualified Data.Vector.Unboxed as V
import qualified Data.Vector.Unboxed.Mutable as MV

data Instruction = Instruction { opcode :: Int
                               , mode1 :: Int
                               , mode2 :: Int
                               , mode3 :: Int
                               } deriving (Show)

data InputOutput = InputOutput
                 { programInputs :: [Int]
                 , programOutputs :: [Int]
                 }

data ProgramState = Running Int | WaitingForInput Int | Halted deriving (Show)

runProgram :: V.Vector Int -> Int -> [Int] -> (V.Vector Int, [Int], ProgramState)
runProgram program pc' inputs = runST $ do
  thawed <- V.thaw program
  ((ram, state), io) <- runStateT (run thawed pc') $ InputOutput inputs []
  frozen <- V.unsafeFreeze ram
  return (frozen, programOutputs io, state)
  where
    run ram pc = do
      value <- MV.read ram pc
      let instruction = decode value
      let opc = opcode instruction
      state <- case opc of
        1 -> runBinaryOp instruction (+)
        2 -> runBinaryOp instruction (*)
        3 -> do
          mbInput <- gets $ listToMaybe . programInputs
          case mbInput of
            Just input -> do
              dst <- MV.read ram (pc + 1)
              MV.write ram dst input
              modify $ \io -> io { programInputs = tail $ programInputs io }
              return $ Running $ pc + 2
            Nothing -> return $ WaitingForInput pc
        4 -> do
          output <- getOperand (pc + 1) (mode1 instruction)
          modify $ \io -> io { programOutputs = output : programOutputs io }
          return $ Running $ pc + 2
        5 -> runJump instruction (/= 0)
        6 -> runJump instruction (== 0)
        7 -> runBinaryOp instruction (\x y -> if x < y then 1 else 0)
        8 -> runBinaryOp instruction (\x y -> if x == y then 1 else 0)
        99 -> return Halted
        x -> fail $ "unknown opcode " ++ show x
      case state of
        Running newPc -> run ram newPc
        Halted ->  return (ram, Halted)
        WaitingForInput _ -> return (ram, state)
      where
        runBinaryOp inst op = do
          x <- getOperand (pc + 1) (mode1 inst)
          y <- getOperand (pc + 2) (mode2 inst)
          dst <- MV.read ram (pc + 3)
          MV.write ram dst $ op x y
          return $ Running $ pc + 4
        runJump inst pred' = do
          condition <- getOperand (pc + 1) (mode1 inst)
          newPc <- if pred' condition
                   then getOperand (pc + 2) (mode2 inst)
                   else return $ pc + 3
          return $ Running newPc
        getOperand addr 0 = do
          pointer <- MV.read ram addr
          MV.read ram pointer
        getOperand addr 1 = MV.read ram addr
        getOperand _ mode = fail $ "bad mode " ++ show mode
    decode value = Instruction opcode' m1 m2 m3
      where opcode' = value `mod` 100
            m1 = value `mod` 1000 `div` 100
            m2 = value `mod` 10000 `div` 1000
            m3 = value `div` 10000
