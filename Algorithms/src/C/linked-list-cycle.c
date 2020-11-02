// https://leetcode.com/problems/linked-list-cycle/

#include "lists.h"


int has_cycle(struct ListNode *head) {
    if (!head) return 0;

    struct ListNode *slow = head;
    struct ListNode *fast = head;
    
    while (fast && fast->next) {
        fast = fast->next->next;
        slow = slow->next;

        if (slow == fast) return 1;
    }

    return 0;
}
