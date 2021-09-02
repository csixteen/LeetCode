// https://leetcode.com/problems/remove-linked-list-elements/

#include "lists.h"


struct ListNode *removeElements(struct ListNode *head, int val) {
    while (head && head->val == val) head = head->next;

    if (!head) return head;

    struct ListNode *curr = head;
    while (curr->next) {
        if (curr->next->val == val) {
            curr->next = curr->next->next;
        } else {
            curr = curr->next;
        }
    }

    return head;
}
