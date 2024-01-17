#![warn(unused)]
use std::cell::RefCell;
use std::rc::Rc;

mod build_array_from_permutation_1920;
mod design_parking_system_1603;
mod find_pivot_index_724;
mod flip_string_to_monotone_increasing_926;
mod insert_interval_57;
mod integer_to_roman_12;
mod longest_path_with_different_adjacent_characters_2246;
mod palindrome_number_9;
mod min_operations_to_make_arr_empty_2870;
mod nodes_in_subtree_with_same_label_1519;
mod nth_tribonacci_number_1137;
mod richest_customer_wealth_1672;
mod roman_to_integer_13;
mod root_equals_sum_of_children_2236;
mod running_sum_of_1_d_array_1480;
mod sign_of_the_product_of_an_array_1822;
mod subarray_sums_divisible_by_k_974;
mod valid_palindrome_125;
mod word_pattern_290;
mod range_sum_of_BST_938;
mod insert_delete_get_random_380;
<<<<<<< HEAD
mod leaf_similar_trees_872;
=======
mod x_of_a_kind_in_a_deck_of_cards_914;
>>>>>>> 318a1be3ba72e05c0e1bd7e008d1c67f05acb1d0

 #[derive(Debug)]
pub struct Solution {}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
