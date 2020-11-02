#include <stdlib.h>

#include "lists.h"


int lists_equal(struct ListNode *a, struct ListNode *b) {
    struct ListNode *a_ptr = a;
    struct ListNode *b_ptr = b;

    while (a_ptr && b_ptr) {
        if (a_ptr->val != b_ptr->val) return 0;

        a_ptr = a_ptr->next;
        b_ptr = b_ptr->next;
    }

    if (a_ptr || b_ptr) return 0;

    return 1;
}


struct ListNode *atolist(int a[], size_t len) {
    if (len < 1) return NULL;

    struct ListNode *curr = (struct ListNode*)malloc(sizeof(struct ListNode));
    curr->val = a[0];
    curr->next = NULL;
    struct ListNode *head = curr;

    for (int i = 1; i < len; ++i) {
        curr->next = (struct ListNode*)malloc(sizeof(struct ListNode));
        curr->next->val = a[i];
        curr->next->next = NULL;
        curr = curr->next;
    }

    return head;
}
