module Main where
import Data.List (findIndex)

-- solve :: String -> String
solve s = do 
  st <- s `findIndex` "Game "
  return ""

main :: IO ()
main = undefined
-- main = interact $ unlines . map solve . lines
