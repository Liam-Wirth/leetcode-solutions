/**
 * Definition for singly-linked list.
 */
#include <stdlib.h>
struct ListNode {
  int val;
  struct ListNode *next;
};
struct ListNode *doubleIt(struct ListNode *head) {
  struct ListNode *temp1 = head;
  struct ListNode *temp2 = head->next;
  head->val = head->val * 2;
  if (head->val >= 10) {
    struct ListNode *new_node =
        (struct ListNode *)(malloc(sizeof(struct ListNode)));
    new_node->next = head;
    head->val = head->val % 10;
    new_node->val = 1;
    head = new_node;
  }
  while (temp2) {
    temp2->val = temp2->val * 2;
    temp1->val = temp1->val + (temp2->val / 10);
    temp2->val = temp2->val % 10;
    temp1 = temp1->next;
    temp2 = temp2->next;
  }
  return (head);
}
