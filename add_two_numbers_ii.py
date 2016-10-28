def sll_to_list(l):
    n, padding = [], 0
    while l:
        n.append(l.val)
        l = l.next

    # Padding only matters if the number is not 0
    if len(n) > 1:
        j = 0
        while j < len(n) and n[j] == 0:
            padding += 1
            j += 1

    return n, padding

def list_to_int(l):
    return int("".join(map(str, l)))

def list_to_sll(l, padding=0):
    lis = ListNode(l[0])
    aux = lis

    for e in l[1:]:
        lis.next = ListNode(e)
        lis = lis.next

    # How much padding do we need?
    if padding > 0:
        p, padding = ListNode(0), padding-1
        head = p
        for i in range(padding):
            p.next = ListNode(0)
            p = p.next
        p.next = aux
        return head
    else:
        return aux

class Solution(object):
    def addTwoNumbers(self, l1, l2):
        """
        :type l1: ListNode
        :type l2: ListNode
        :rtype: ListNode
        """
        n1, p1 = sll_to_list(l1)
        n2, p2 = sll_to_list(l2)
        l = list(str(list_to_int(n1)+list_to_int(n2)))
        # We need padding
        m = max(len(n1), len(n2))
        if len(l) < m:
            padding = m - len(l)
        else:
            padding = 0

        a = list_to_sll(l, padding)
        return a
