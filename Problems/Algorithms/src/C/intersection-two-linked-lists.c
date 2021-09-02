// https://leetcode.com/problems/intersection-of-two-linked-lists/

#include <stdlib.h>

#include "lists.h"

#define MAX(A,B) ((A) > (B)) ? (A) : (B)


struct ListNode *getIntersectionNode(struct ListNode *headA, struct ListNode *headB) {
    int len_a = 0, len_b = 0;

    struct ListNode *a_ptr = headA;
    struct ListNode *b_ptr = headB;

    while (a_ptr) { len_a++; a_ptr = a_ptr->next; }
    while (b_ptr) { len_b++; b_ptr = b_ptr->next; }

    diff_a = MAX(0, len_a - len_b);
    diff_b = MAX(0, len_b - len_a);

    a_ptr = headA;
    b_ptr = headB;

    for(int i = 0; i < diff_a; i++, a_ptr = a_ptr->next);
    for(int i = 0; i < diff_b; i++, b_ptr = b_ptr->next);

    while (a_ptr && b_ptr) {
        if (a_ptr->val == b_ptr->val)
            return a_ptr;

        a_ptr = a_ptr->next;
        b_ptr = b_ptr->next;
    }

    return NULL;
}
