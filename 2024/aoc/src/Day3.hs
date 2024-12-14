module Day3 (part1, part2) where

import Control.Monad (void)
import Data.Void (Void)
import Text.Megaparsec
import Text.Megaparsec.Char (char, digitChar, latin1Char, string)

type Parser = Parsec Void String

part1 :: FilePath -> IO (Either (ParseErrorBundle String Void) Int)
part1 path = part1' <$> readFile path

part1' :: String -> Either (ParseErrorBundle String Void) Int
part1' s = computeSumOfMul <$> parse' parserPart1 s

part2 :: FilePath -> IO (Either (ParseErrorBundle String Void) Int)
part2 path = part2' <$> readFile path

part2' :: String -> Either (ParseErrorBundle String Void) Int
part2' s = computeSumOfMul <$> parse' parserPart2 s

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
integer = read <$> some digitChar

parserPart1 :: Parser [(Int, Int)]
parserPart1 =
  let p = try mul <|> try (latin1Char >> p)
   in many p

computeSumOfMul :: [(Int, Int)] -> Int
computeSumOfMul = sum . map (uncurry (*))

parse' :: Parser a -> String -> Either (ParseErrorBundle String Void) a
parse' p = parse p ""

do_ :: Parser ()
do_ = void $ string "do()"

dont :: Parser ()
dont = void $ string "don't()"

comment :: Parser ()
comment = dont >> void (manyTill latin1Char (do_ <|> eof))

parserPart2 :: Parser [(Int, Int)]
parserPart2 = many go
  where
    go = do
      void $ optional $ try comment
      try mul <|> try ((comment <|> void latin1Char) >> go)
