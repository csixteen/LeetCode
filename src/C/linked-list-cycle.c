// https://leetcode.com/problems/linked-list-cycle/

#include <stdio.h>
#include <stdlib.h>

struct ListNode {
    int val;
    struct ListNode *next;
};

int has_cycle(struct ListNode *head) {
    if (!head) { return 0; }

    struct ListNode *slow = head;
    struct ListNode *fast = head;
    
    while (fast && fast->next) {
        fast = fast->next->next;
        slow = slow->next;

        if (slow == fast) {
            return 1;
        }
    }

    return 0;
}

int main(int argc, char **argv) {
    printf("Hello, world\n");

    return 0;
}
