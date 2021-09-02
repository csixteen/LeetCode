#!/usr/bin/env python3
# coding: utf-8

from typing import List


class Solution:
    def maximumUnits(self, boxTypes: List[List[int]], truckSize: int) -> int:
        boxes = sorted(boxTypes, key=lambda x: x[1], reverse=True)
        total = 0

        for num_boxes, units in boxes:
            d = min(num_boxes, truckSize)
            total += units * d
            truckSize -= d
            if truckSize == 0:
                break

        return total
