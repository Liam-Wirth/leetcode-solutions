  
  package com.liam.leetcode;

  import java.util.ArrayList;
  import java.util.Collections;
  import java.util.LinkedList;

  class ListNode {
      int val;
      ListNode next;
      ListNode() {}
      ListNode(int val) { this.val = val; }
      ListNode(int val, ListNode next) { this.val = val; this.next = next; }
  }


  class AddTwoNumbers{
      public static void main(String[] args) {
           Solution solution = new AddTwoNumbers().new Solution();
      }
      //trying floats first to see how it affects memmory/speed and then I'll refactor things to short and see how it affects.
      //leetcode submit region begin(Prohibit modification and deletion)
class Solution {
    public ListNode addTwoNumbers(ListNode l1, ListNode l2) {
      ListNode temp = l1;
      ArrayList<Integer> tempList = new ArrayList<>();
      short num1 = 0, num2 = 0;
        for (int i = 0; i < 2; i++) {
            while(temp.next!= null){
                tempList.add(temp.val);
            }
            Collections.reverse(tempList);
            if(i == 0)
                num1 = Short.parseShort(tempList.toString());
            else
                num2 = Short.parseShort(tempList.toString());
            tempList.clear();
            temp = l2;
        }
        String out = String.valueOf(num1+num2);
        LinkedList<ListNode> outlist = new LinkedList<ListNode>();
        for (int i : out.toCharArray()) {
            outlist.push(new ListNode(i));
        }
        return outlist.getFirst();
        }

}
//leetcode submit region end(Prohibit modification and deletion)

  }