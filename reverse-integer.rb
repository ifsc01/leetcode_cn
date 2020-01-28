# @param {Integer} x
# @return {Integer}
# https://leetcode-cn.com/problems/reverse-integer/
def reverse(x)
    r = if x < 0
        -x.to_s.reverse.to_i
    else
        x.to_s.reverse.to_i
    end
    if r < -2**31 || r >2**31-1
        0
    else 
        r
    end
end
