package Java;
import java.util.HashMap;
import java.util.Map;
import java.util.TreeMap;


// @lc code=start
class TimeMap {
    private static final String DEFAULT_VALUE = "";
    private final HashMap<String, TreeMap<Integer, String>> map;

    /** Initialize your data structure here. */
    public TimeMap() {
        map = new HashMap<>();
    }
    
    public void set(String key, String value, int timestamp) {
        TreeMap<Integer, String> timeMap;
        if (map.containsKey(key)) {
            timeMap = map.get(key);
        } else {
            timeMap = new TreeMap<>();
            map.put(key, timeMap);
        }
        timeMap.put(timestamp, value);
    }
    
    public String get(String key, int timestamp) {
        if (map.containsKey(key)) {
            TreeMap<Integer, String> timeMap = map.get(key);
            Integer floorKey = timeMap.floorKey(timestamp);
            if (floorKey != null) {
                return timeMap.get(floorKey);
            }
        }
        return DEFAULT_VALUE;
    }
}
/**
 * Your TimeMap object will be instantiated and called as such:
 * TimeMap obj = new TimeMap();
 * obj.set(key,value,timestamp);
 * String param_2 = obj.get(key,timestamp);
 */
// @lc code=end

