# https://leetcode-cn.com/problems/maximum-product-of-three-numbers

def maximum_product(nums)
  sort_nums = nums.sort {|a, b| b <=> a }
  a = sort_nums.take(3).reduce(:*)
  b = sort_nums.last(3).reduce(:*)
  [a,b, sort_nums.first * sort_nums.last(2).reduce(:*) ].max
end

