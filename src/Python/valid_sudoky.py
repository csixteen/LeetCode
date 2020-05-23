#!/usr/bin/env python3
# coding: utf-8
import unittest


class Solution:
    def valid_sudoku(self, board):
        verticals = [set() for _ in range(9)]
        horizontals = [set() for _ in range(9)]
        squares = [set() for _ in range(9)]

        # Check horizontals
        for row in range(9):
            for col in range(9):
                x = board[row][col]
                if x != '.':
                    if any([
                        x in horizontals[row],
                        x in verticals[col],
                        x in squares[(row // 3) * 3 + (col // 3)]
                    ]):
                        return False
                    else:
                        horizontals[row].add(x)
                        verticals[col].add(x)
                        squares[(row // 3) * 3 + (col // 3)].add(x)

        return True
        

class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test_cases = [
            (
                [
                    ["5","3",".",".","7",".",".",".","."],
                    ["6",".",".","1","9","5",".",".","."],
                    [".","9","8",".",".",".",".","6","."],
                    ["8",".",".",".","6",".",".",".","3"],
                    ["4",".",".","8",".","3",".",".","1"],
                    ["7",".",".",".","2",".",".",".","6"],
                    [".","6",".",".",".",".","2","8","."],
                    [".",".",".","4","1","9",".",".","5"],
                    [".",".",".",".","8",".",".","7","9"]
                ], 
                True
            ),
            (
                [
                    ["8","3",".",".","7",".",".",".","."],
                    ["6",".",".","1","9","5",".",".","."],
                    [".","9","8",".",".",".",".","6","."],
                    ["8",".",".",".","6",".",".",".","3"],
                    ["4",".",".","8",".","3",".",".","1"],
                    ["7",".",".",".","2",".",".",".","6"],
                    [".","6",".",".",".",".","2","8","."],
                    [".",".",".","4","1","9",".",".","5"],
                    [".",".",".",".","8",".",".","7","9"]
                ],
                False
            )
        ]

    def test_valid_sudoku(self):
        s = Solution()
        for i, e in self.test_cases:
            self.assertEqual(s.valid_sudoku(i), e)


if __name__ == '__main__':
    unittest.main(verbosity=2)

