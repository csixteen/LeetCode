#include <stdlib.h>


struct ListNode {
    int val;
    struct ListNode *next;
};


int lists_equal(struct ListNode *, struct ListNode *);
struct ListNode *atolist(int a[], size_t len);
