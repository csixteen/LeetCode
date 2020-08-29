// https://leetcode.com/problems/odd-even-linked-list/

#include "lists.h"


struct ListNode *oddEvenList(struct ListNode *head) {
    if (!head || !head->next) return head;

    struct ListNode *odd = head;
    struct ListNode *even = head->next;
    struct ListNode *even_head = head->next;

    while (even && even->next) {
        odd->next = odd->next->next;
        odd = odd->next;
        even->next = even->next->next;
        even = even->next;
    }

    odd->next = even_head;

    return head;
}
