module Day3 where

import Control.Applicative ((<|>))
import Control.Monad (void)
import Text.Parsec (ParseError, anyChar, char, digit, many, many1, parse, string, try)
import Text.Parsec.String (Parser)

part1 :: FilePath -> IO (Either ParseError Int)
part1 path = part1' <$> readFile path

part1' :: String -> Either ParseError Int
part1' s = computeSumOfMul <$> parse' inputParser s

mul :: Parser (Int, Int)
mul = string "mul" >> factors

factors :: Parser (Int, Int)
factors = do
  void $ char '('
  x <- integer
  void $ char ','
  y <- integer
  void $ char ')'
  return (x, y)

integer :: Parser Int
integer = read <$> many1 digit

inputParser :: Parser [(Int, Int)]
inputParser =
  let p = try mul <|> try (anyChar >> p)
   in many p

computeSumOfMul :: [(Int, Int)] -> Int
computeSumOfMul = sum . map (uncurry (*))

parse' :: Parser a -> String -> Either ParseError a
parse' p = parse p ""
