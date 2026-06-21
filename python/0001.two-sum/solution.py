# Created by zhang-mo-peng-mo-peng-mo-mo-peng at 2026/06/21 00:18
# leetgo: 1.4.17
# https://leetcode.cn/problems/two-sum/

from typing import *
from leetgo_py import *

# @lc code=begin


class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        mp = {}
        for i,num in enumerate(nums):
            other = target - num
            if other in mp:
                return [mp[other],i]
            mp[num] = i


# @lc code=end

if __name__ == "__main__":
    nums: List[int] = deserialize("List[int]", read_line())
    target: int = deserialize("int", read_line())
    ans = Solution().twoSum(nums, target)
    print("\noutput:", serialize(ans, "integer[]"))
