import java.util.Arrays;
import java.util.HashMap;

public class tester {
    HashMap<Character, Integer> map = new HashMap<Character, Integer>();
    public boolean isValid(String s) {
     //NVM I changed my mind, I want to solve this problem a different way
     char schars[] = s.toCharArray();
     System.out.println(schars);
     int arrlength = schars.length;
     System.out.println(arrlength);
     //get some base cases out of the way
     //if the amount of characters is not an even number, then there HAS to be an unclosed pair
     if((arrlength)%2!=0) return false;
    //just thought of a stupid ez way to solve the problem
     char firstHalf[] = Arrays.copyOfRange(schars, 0, arrlength/2);
     char otherHalf[] = Arrays.copyOfRange(schars,arrlength/2, arrlength);
     System.out.println(firstHalf);
     System.out.println(otherHalf);
     return false;
    }

public static void main(String[] args) {
    tester balls = new tester();
    balls.isValid("({[]})");

    }

}

