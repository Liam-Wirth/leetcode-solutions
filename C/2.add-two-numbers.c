/**
 * Definition for singly-linked list.
 */
#include <stdlib.h>
struct ListNode {
  int val;
  struct ListNode *next;
};

struct ListNode *addTwoNumbers(struct ListNode *l1, struct ListNode *l2) {
  int c = 0; // carry
  struct ListNode *dummy =
      (struct ListNode *)malloc(sizeof(struct ListNode)); // dummy node
  struct ListNode *tail = dummy; // tail pointer to build the result

  while (l1 != NULL || l2 != NULL || c > 0) {
    int sum = c;

    if (l1 != NULL) {
      sum += l1->val;
      l1 = l1->next;
    }
    if (l2 != NULL) {
      sum += l2->val;
      l2 = l2->next;
    }

    c = sum / 10;   // calculate new carry
    sum = sum % 10; // calculate digit to store

    // Create new node and link it
    struct ListNode *newNode =
        (struct ListNode *)malloc(sizeof(struct ListNode));
    newNode->val = sum;
    newNode->next = NULL;

    // Link the new node
    tail->next = newNode;
    // Move tail to the new node
    tail = tail->next;
  }

  // Get the real head (skip dummy node)
  struct ListNode *result = dummy->next;
  free(dummy); // Free the dummy node
  return result;
}
