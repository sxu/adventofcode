{-# LANGUAGE OverloadedStrings #-}

module Day6 (day6) where

import Control.Monad.State
import qualified Data.HashMap.Strict as H
import qualified Data.Text as T
import qualified Text.Parsec as P hiding (State)

data Mass = Mass T.Text[Mass] deriving (Show)

buildSystem :: [(T.Text, T.Text)] -> Mass
buildSystem orbits = evalState (parse orbits >> build "COM") H.empty
  where
    build :: T.Text -> State (H.HashMap T.Text [T.Text]) Mass
    build id' = do
      mbSats <- gets $ H.lookup id'
      ms <- case mbSats of
              Just satellites -> mapM build satellites
              Nothing -> return []
      return $ Mass id' ms
    parse :: [(T.Text, T.Text)] -> State (H.HashMap T.Text [T.Text]) ()
    parse ((p, s) : os) = do
      satellites <- lookup' p
      modify $ H.insert p (s:satellites)
      parse os
      where
        lookup' :: T.Text -> State (H.HashMap T.Text [T.Text]) [T.Text]
        lookup' k = do
          mb <- gets $ H.lookup k
          case mb of
            Just v -> return v
            Nothing -> modify (H.insert k []) >> lookup' k
    parse [] = return ()

numOrbits :: Mass -> Int
numOrbits mass = fst $ go mass
  where
    go (Mass _ []) = ( 0 -- number of orbits round mass
                     , 1 -- number of masses
                     )
    go (Mass _ satellites) = (recOrbits + numMasses, numMasses + 1)
      where (os, ms) = unzip $ map go satellites
            recOrbits = sum os
            numMasses = sum ms

data Transfer = Partial Int | Complete Int | NA deriving (Show, Eq)

numTransfers :: T.Text -> T.Text -> Mass -> Transfer
numTransfers x y (Mass name satellites)
  | name == x || name == y = Partial 0
  | otherwise = case filter found $ map (numTransfers x y) satellites of
                  [Partial v1, Partial v2] -> Complete $ v1 + v2
                  [Partial v] -> Partial $ v + 1
                  [Complete v] -> Complete v
                  [] -> NA
                  _ -> error "unreachable"
  where found NA = False
        found _ = True
           
day6 :: String -> IO ()
day6 input =
  case P.parse (parseOrbit `P.sepEndBy` P.endOfLine) "" (T.pack input) of
    Left err -> fail $ show err
    Right orbits -> do
      let system = buildSystem orbits
      let part1 = numOrbits system
      let part2 = numTransfers "YOU" "SAN" system
      guard (part1 == 171213 && part2 == Complete 292)
  where
    parseOrbit = do
      parent <- parseMass
      satellite <- P.string ")" >> parseMass
      return (T.pack parent, T.pack satellite)
    parseMass = P.many1 P.alphaNum
