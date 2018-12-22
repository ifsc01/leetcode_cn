// https://leetcode-cn.com/problems/add-two-numbers/



/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */


func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
    dummyHead := &ListNode{Val: 0}
    curr := dummyHead
    p := l1
    q := l2
    carry := 0 
    
    for(p != nil || q != nil ) {
        x := 0
        if p != nil {
            x = p.Val
        }
        
        y := 0
        if q != nil {
            y = q.Val
        }
        
        sum := x + y + carry
        carry = sum / 10
        curr.Next = &ListNode{Val: sum % 10}
        curr = curr.Next
        if p != nil {
            p = p.Next
        }
        
        if q != nil {
            q = q.Next
        }
    }
    
    if carry > 0 {
        curr.Next = &ListNode{Val: carry}
    }
    
    return dummyHead.Next
}