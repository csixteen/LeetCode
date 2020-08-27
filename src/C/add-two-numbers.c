// https://leetcode.com/problems/add-two-numbers/

struct ListNode {
    int val;
    struct ListNode *next;
};


struct ListNode* addTwoNumbers(struct ListNode* l1, struct ListNode* l2) {
    struct ListNode *dummy = (struct ListNode*)malloc(sizeof(struct ListNode));
    dummy->val = -1;
    dummy->next = NULL;
    struct ListNode *curr = dummy;
    int carry = 0;

    while (l1 || l2 || carry > 0) {
        int res = carry;

        if (l1) { res += l1->val; l1 = l1->next; }
        if (l2) { res += l2->val; l2 = l2->next; }

        carry = res / 10;
        res = res % 10;

        curr->next = malloc(sizeof(struct ListNode));
        curr->next->val = res;
        curr->next->next = NULL;
        curr = curr->next;
    }

    return dummy->next;
}

int main(int argc, char **argv) {
    printf("Hello, world!\n");

    return 0;
}
