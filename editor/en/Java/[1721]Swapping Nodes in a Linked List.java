package Java;


  import java.util.ArrayList;
  import java.util.Arrays;
  import java.util.LinkedList;
  import java.util.List;
  import java.util.function.Consumer;

  class SwappingNodesInALinkedList{
      public static void main(String[] args) {

           Solution solution = new SwappingNodesInALinkedList().new Solution();

      }
      public class ListNode {
          int val;
          ListNode next;
          ListNode() {}
          ListNode(int val) { this.val = val; }
          ListNode(int val, ListNode next) { this.val = val; this.next = next; }
      }
      //leetcode submit region begin(Prohibit modification and deletion)




class Solution {
    public ListNode swapNodes(ListNode head, int k) {
        if(head.next == null){
            return head;
        }
        LinkedList<ListNode> list = new LinkedList();
        byte iterations = 0;
        ListNode firstK = new ListNode();
        ArrayList<ListNode> balls = new ArrayList<>();
        ListNode temp = head;
        while (temp.next != null) {
            balls.add(temp);
            list.add(temp);
            temp = temp.next;
        };
        int length = balls.size();
        int loink = list.get(k).val;
        list.get(k-1).val = list.get(length-k).val;
        list.get(length-k).val = loink;

        return head;
    }
}
//leetcode submit region end(Prohibit modification and deletion)

  }