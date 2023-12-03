module Main where

import Data.ByteString.Char8 (readInt)
import Data.Char (isDigit)
import Data.List (findIndex, isPrefixOf)

-- get the firsst integer value
getCalibrationOnlyDigits :: String -> Int
getCalibrationOnlyDigits line =
  let digits = filter isDigit line
   in read [head digits, last digits]

first :: String -> (String -> String) -> String
first s f =
  if isDigit (head s)
    then take 1 s
    else
      maybe
        (first (tail s) f)
        (show . (+ 1))
        ( findIndex
            (`isPrefixOf` s)
            ( map
                f
                ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
            )
        )

-- When numbers are spelled out like one, two, three, four, five, six, seven, eight, nine
getCalibration :: String -> Int
getCalibration s = read $ first s id ++ first (reverse s) reverse

partA :: String -> IO ()
partA content = print $ sum $ map getCalibrationOnlyDigits $ lines content

partB :: String -> IO ()
partB content = print $ sum $ map getCalibration $ lines content

-- Read till no input, combine first and last character together and sum the up
main :: IO ()
main = do
  content <- getContents
  partA content
  partB content
  return ()

{-
 - Problem: Trebuchet?!
 - Start: 2023-12-03
-}
