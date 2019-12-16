{-# LANGUAGE FlexibleContexts #-}
module Intcode (runProgram, runProgramUnsafe, ProgramState(..)) where

import Control.Monad.Except
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
                 , programOutputs :: [Int] -> [Int]
                 }

data ProgramState = Running Int Int
                  | WaitingForInput Int Int
                  | Halted deriving (Show)

newtype PageFault = PageFault Int deriving (Show)

decodeInstruction :: Int -> Instruction
decodeInstruction value = Instruction opcode' m1 m2 m3
  where opcode' = value `mod` 100
        m1 = value `mod` 1000 `div` 100
        m2 = value `mod` 10000 `div` 1000
        m3 = value `div` 10000

runProgram :: V.Vector Int
           -> Int
           -> Int
           -> [Int]
           -> (V.Vector Int, [Int], ProgramState)
runProgram = runProgram' True

runProgramUnsafe :: V.Vector Int
           -> Int
           -> Int
           -> [Int]
           -> (V.Vector Int, [Int], ProgramState)
runProgramUnsafe = runProgram' False

runProgram' :: Bool
            -> V.Vector Int
            -> Int
            -> Int
            -> [Int]
            -> (V.Vector Int, [Int], ProgramState)
runProgram' safe program pc' rb' inputs = runST $ do
  thawed <- if safe then V.thaw program else V.unsafeThaw program
  ((ram, prgState), io) <- runStateT (run thawed pc' rb')
                                     (InputOutput inputs id)
  frozen <- V.unsafeFreeze ram
  return (frozen, programOutputs io [], prgState)

run :: MV.MVector s Int
    -> Int
    -> Int
    -> StateT InputOutput (ST s) (MV.MVector s Int, ProgramState)
run ram pc rb = do
  value <- MV.read ram pc
  let instruction = decodeInstruction value
  result <- runExceptT $ runInstruction instruction
  case result of
    Left (PageFault addr) -> do
      let n = MV.length ram
      ram' <- MV.grow ram (addr + 1 - n) 
      forM_ [n .. (MV.length ram' - 1)] (\idx -> MV.write ram' idx 0)
      run ram' pc rb
    Right prgState -> case prgState of
      Running newPc newRb -> run ram newPc newRb
      Halted ->  return (ram, Halted)
      WaitingForInput _ _ -> return (ram, prgState)
  where
    runInstruction instruction = do
      let opc = opcode instruction
      case opc of
        1 -> runBinaryOp instruction (+)
        2 -> runBinaryOp instruction (*)
        3 -> do
          mbInput <- gets $ listToMaybe . programInputs
          case mbInput of
            Just input -> do
              dst <- getDst (pc + 1) (mode1 instruction)
              writeRam dst input
              modify $ \io -> io { programInputs = tail $ programInputs io }
              return $ Running (pc + 2) rb
            Nothing -> return $ WaitingForInput pc rb
        4 -> do
          output <- getOperand (pc + 1) (mode1 instruction)
          modify $ \io -> io { programOutputs = programOutputs io . (output:) }
          return $ Running (pc + 2) rb
        5 -> runJump instruction (/= 0)
        6 -> runJump instruction (== 0)
        7 -> runBinaryOp instruction (\x y -> if x < y then 1 else 0)
        8 -> runBinaryOp instruction (\x y -> if x == y then 1 else 0)
        9 -> do
          adjustment <- getOperand (pc + 1) (mode1 instruction)
          return $ Running (pc + 2) (rb + adjustment)
        99 -> return Halted
        x -> fail $ "unknown opcode " ++ show x
    runBinaryOp inst op = do
      x <- getOperand (pc + 1) (mode1 inst)
      y <- getOperand (pc + 2) (mode2 inst)
      dst <- getDst (pc + 3) (mode3 inst)
      writeRam dst $ op x y
      return $ Running (pc + 4) rb
    runJump inst pred' = do
      condition <- getOperand (pc + 1) (mode1 inst)
      newPc <- if pred' condition
               then getOperand (pc + 2) (mode2 inst)
               else return $ pc + 3
      return $ Running newPc rb
    getOperand addr 0 = do
      pointer <- readRam addr
      readRam pointer
    getOperand addr 1 = readRam addr
    getOperand addr 2 = do
      diff <- readRam addr
      readRam (diff + rb)
    getOperand _ mode = fail $ "bad mode " ++ show mode
    getDst addr 0 = readRam addr
    getDst addr 2 = do
      diff <- readRam addr
      return (diff + rb)
    getDst _ mode = fail $ "bad mode " ++ show mode
    readRam addr = do
      let n = MV.length ram
      if addr < n
      then MV.read ram addr
      else return 0
    writeRam addr val = do
      let n = MV.length ram
      if addr < n
      then MV.write ram addr val
      else throwError $ PageFault addr
