rollingPairs :: [a] -> [(a, a)]
rollingPairs [] = []
rollingPairs (x:xs)
  | length xs > 0 = (x, head xs) : rollingPairs xs
  | otherwise = []

-- rollingPairs xs = zip xs (tail xs)

main :: IO ()
main = do
  contents <- readFile "input.txt"
  let xs = (map read $ lines contents) :: [Int]
  print . length . filter (uncurry (<)) . rollingPairs $ xs
