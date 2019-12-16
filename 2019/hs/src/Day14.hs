{-# LANGUAGE OverloadedStrings #-}
module Day14 (day14) where

import Control.Monad (guard)
import Data.List (foldl')
import Data.Maybe (fromMaybe)
import qualified Data.Dequeue as Q
import qualified Data.HashMap.Strict as H
import qualified Data.Text as T
import qualified Text.Parsec as P
import qualified Text.Parsec.Text as P

data Reaction = Reaction
              { reactionInputs :: [(T.Text, Int)]
              , reactionOutputQuantity :: Int
              } deriving (Show)

parseReaction :: P.Parser (H.HashMap T.Text Reaction)
parseReaction = do
  inputs <- parseChemical `P.sepBy` (P.string "," >> P.spaces)
  P.spaces >> P.string "=>" >> P.spaces
  (output, quantity) <- parseChemical
  return $ H.singleton output $ Reaction inputs quantity
  where
    parseChemical = do
      ds <- P.many1 P.digit
      ls <- P.spaces >> P.many1 P.letter
      return (T.pack ls, read ds)

produce :: (Q.Dequeue q, Show (q T.Text))
        => H.HashMap T.Text Reaction
        -> q T.Text
        -> H.HashMap T.Text Int
        -> H.HashMap T.Text Int
produce recipe queue balance
  | Q.null queue = balance
  | otherwise = if target == "ORE"
                then produce recipe queue' balance
                else if numReactions > 0
                then produce recipe newQueue newBalance
                else produce recipe queue' balance
  where
    (target, queue') = fromMaybe (error "empty queue") $ Q.popFront queue
    needed = -(H.lookupDefault 0 target balance)
    Reaction inputs outputQuantity = fromMaybe (error "reaction not found")
                                               (H.lookup target recipe)
    numReactions = (max 0 needed + outputQuantity - 1) `div` outputQuantity
    inputsNeeded = map (\(c, q) -> (c, q * numReactions)) inputs
    newBalance = H.insertWith (+)
                              target
                              (numReactions * outputQuantity)
                              (foldr (\(c, q) b -> H.insertWith (+) c (-q) b)
                                       balance
                                       inputsNeeded)
    newQueue = foldl' (\q (c, _) -> Q.pushBack q c) queue' inputs


maximizeFuel :: H.HashMap T.Text Reaction -> Int -> Int -> Int -> Int
maximizeFuel recipe ore = go
  where
    go lo hi
      | lo == hi = lo
      | otherwise = let target = (lo + hi) `div` 2 + 1
                    in if canProduce target then go target hi else go lo (target - 1)
    canProduce target =
      fromMaybe (error "") (H.lookup "ORE" balance) >= 0
      where
        balance = produce recipe
                          (Q.fromList ["FUEL"] :: Q.BankersDequeue T.Text)
                          (H.fromList [("FUEL", -target), ("ORE", ore)])

day14 :: String -> IO  ()
day14 input = do
  let parsed = P.parse (parseReaction `P.sepEndBy` P.endOfLine) 
                       ""
                       (T.pack input)
  case parsed of
    Left err -> print err
    Right rs -> do
      let recipe = mconcat rs
      let balance = produce recipe
                            (Q.fromList ["FUEL"] :: Q.BankersDequeue T.Text)
                            (H.singleton "FUEL" (-1))
      let ore = fromMaybe (error "") $ H.lookup "ORE" balance
      guard (-ore == 857266)
      print (-ore)
      let minFuel = 1000000000000 `div` (-ore)
      let fuel = maximizeFuel recipe 1000000000000 minFuel (minFuel * 2)
      guard (fuel == 2144702)
      print fuel
