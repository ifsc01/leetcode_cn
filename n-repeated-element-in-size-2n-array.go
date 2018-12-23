// https://leetcode-cn.com/problems/n-repeated-element-in-size-2n-array

func repeatedNTimes(A []int) int {
    n := len(A)/2
    
    result := make(map[int]int)
    for _, v := range A {
        val, ok := result[v]
        if ok {
            result[v] = val + 1
        } else {
            result[v] = 1
        }
    }
    
    for k,v := range result {
        if v == n {
            return k
        }
    }
    panic("error")
}