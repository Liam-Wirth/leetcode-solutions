//TODO: Solve this
/*
 * @lc app=leetcode id=1603 lang=rust
 *
 * [1603] Design Parking System
 *
 * https://leetcode.com/problems/design-parking-system/description/
 *
 * algorithms
 * Easy (87.71%)
 * Likes:    708
 * Dislikes: 289
 * Total Accepted:    126.2K
 * Total Submissions: 143.9K
 * Testcase Example:  '["ParkingSystem","addCar","addCar","addCar","addCar"]\n' +
  '[[1,1,0],[1],[2],[3],[1]]'
 *
 * Design a parking system for a parking lot. The parking lot has three kinds
 * of parking spaces: big, medium, and small, with a fixed number of slots for
 * each size.
 *
 * Implement the ParkingSystem class:
 *
 *
 * ParkingSystem(int big, int medium, int small) Initializes object of the
 * ParkingSystem class. The number of slots for each parking space are given as
 * part of the constructor.
 * bool addCar(int carType) Checks whether there is a parking space of carType
 * for the car that wants to get into the parking lot. carType can be of three
 * kinds: big, medium, or small, which are represented by 1, 2, and 3
 * respectively. A car can only park in a parking space of its carType. If
 * there is no space available, return false, else park the car in that size
 * space and return true.
 *
 *
 *
 * Example 1:
 *
 *
 * Input
 * ["ParkingSystem", "addCar", "addCar", "addCar", "addCar"]
 * [[1, 1, 0], [1], [2], [3], [1]]
 * Output
 * [null, true, true, false, false]
 *
 * Explanation
 * ParkingSystem parkingSystem = new ParkingSystem(1, 1, 0);
 * parkingSystem.addCar(1); // return true because there is 1 available slot
 * for a big car
 * parkingSystem.addCar(2); // return true because there is 1 available slot
 * for a medium car
 * parkingSystem.addCar(3); // return false because there is no available slot
 * for a small car
 * parkingSystem.addCar(1); // return false because there is no available slot
 * for a big car. It is already occupied.
 *
 *
 *
 * Constraints:
 *
 *
 * 0 <= big, medium, small <= 1000
 * carType is 1, 2, or 3
 * At most 1000 calls will be made to addCar
 *
 *
 */

// @lc code=start
struct ParkingSystem {
    big: u32,
    medium: u32,
    small: u32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ParkingSystem {

fn new(big: i32, medium: i32, small: i32) -> Self {
        let mut parkingsystem = ParkingSystem {
            big: big as u32,
            medium: medium as u32,
            small: small as u32,
        };
    parkingsystem

    }

    fn add_car(&self, car_type: i32) -> bool {
        false
    }
}


