
import java.util.ArrayList;

// @lc code=start
class Solution {
    public ArrayList<String> removeComments(String[] source) {
       ArrayList<String> solution = new ArrayList<>();
        boolean inBlock = false;
        /*
         * I think I'm gonna need the stringbuilder for that one test case
         * where a block comment starts with like a/* comment here (asterisk)/b and 
           ends with the letter b, it says it wants the output to be "ab" in that case     
         */
        StringBuilder delimLine = new StringBuilder();
        for (String line : source) {
            if(!inBlock){
                delimLine = new StringBuilder();
            }
            for (int i = 0; i < line.length(); i++) {
                boolean b = i <line.length()-1;
                char c = line.charAt(i);
                if(!inBlock){
                    if(c == '/'){
                        //now I need to check if it's either
                        //A start of a regular comment done
                        //B start of a block comment done
                        //C end of a block comment done
                        //D something else done
                        if(b){
                            if(c==line.charAt(i+1)){//I already know c is '/'
                                break; //don't need the rest of the line
                            }else if(line.charAt(i+1)=='*'){
                                i++; //don't need to read the *
                                inBlock = true; //can't break
                            }else{ //it's likely an operand, so add it to the builder
                                delimLine.append(c);
                            }
                        }else delimLine.append(c); 
                    }else{
                     delimLine.append(c);   
                    } 
                }else{
                   //now we check if we are still in a block comment
                   
                      if(c=='*'){
                        if(i<line.length()-1 && line.charAt(i+1)=='/'){
                              inBlock = false;
                              i++;
                        }
                    }

                }
               
            } if (delimLine.length() != 0 && !inBlock) {
                  solution.add(delimLine.toString()); 
                } 
        }
        return solution;              
    }
}
// @lc code=end

