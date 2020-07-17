// https://leetcode.com/problems/linked-list-cycle/

#include <stdio.h>
#include <stdlib.h>

typedef enum { FALSE, TRUE } bool;

typedef struct _ListNode {
    int val;
    struct _ListNode *next;
} ListNode;

bool has_cycle(ListNode *head) {
    ListNode *slow = head;
    ListNode *fast = NULL;
    
    if (head && head->next) {
        fast = head->next;
    }

    while (slow && fast) {
        if (slow->val == fast->val) {
            return TRUE;
        }

        slow = slow->next;
        fast = fast->next;
        if (fast) { fast = fast->next; }
    }

    return FALSE;
}

int main(int argc, char **argv) {
    printf("Hello, world\n");

    return 0;
}
