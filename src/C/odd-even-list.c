// https://leetcode.com/problems/odd-even-linked-list/

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>


typedef struct ListNode {
    int val;
    struct ListNode *next;
} LNode;


LNode *oddEvenList(LNode *head) {
    if (!head || !head->next)
        return head;

    LNode *odd = head;
    LNode *even = head->next;
    LNode *even_head = head->next;

    while (even && even->next) {
        odd->next = odd->next->next;
        odd = odd->next;
        even->next = even->next->next;
        even = even->next;
    }

    odd->next = even_head;

    return head;
}


int lists_equal(LNode *l1, LNode *l2) {
    LNode *l1_ptr = l1;
    LNode *l2_ptr = l2;

    while (l1_ptr && l2_ptr) {
        if (l1_ptr->val != l2_ptr->val)
            return 0;

        l1_ptr = l1_ptr->next;
        l2_ptr = l2_ptr->next;
    }

    if (l1_ptr || l2_ptr)
        return 0;

    return 1;
}


int main(void) {
    printf("Write some tests!\n");

    return 0;
}
