#![warn(unused)]
use std::cell::RefCell;
use std::rc::Rc;

mod build_array_from_permutation_1920;
mod find_all_duplicates_in_array_442;
mod length_of_last_word_58;
mod first_missing_positive_41;
mod find_the_duplicate_number_287;
mod top_k_frequent_elements_347;
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
mod intersection_of_two_arrays_349;
mod roman_to_integer_13;
mod root_equals_sum_of_children_2236;
mod running_sum_of_1_d_array_1480;
mod find_pivot_integer_2485;
mod sign_of_the_product_of_an_array_1822;
mod subarray_sums_divisible_by_k_974;
mod word_pattern_290;
mod x_of_a_kind_in_a_deck_of_cards_914;
mod insert_delete_get_random_380;
mod range_sum_of_bst_938;
mod set_mismatch_645;
mod house_robber_198;
mod longest_common_subsequence_1143;
mod implement_queue_using_stacks_232;
mod evaluate_reverse_polish_notation_150;
mod add_binary_67;
mod sequential_digits_1291;
mod divide_array_into_arrays_with_max_difference_2966;
mod first_unique_character_in_a_string_387;
mod group_anagrams_49;
mod sort_characters_by_frequency_451;
mod palindromic_strings_647;
mod contains_duplicate_217;
mod valid_anagram_242;
mod two_sum_1;
mod majority_element_169;
mod find_first_palindromic_string_in_the_array_2108;
mod find_bottom_left_tree_value_513;
mod minimum_length_of_string_after_deleting_similar_ends_1750;
mod rearrange_array_elements_by_sign_2149;
mod least_number_of_unique_integers_after_k_removals_1481;   
mod missing_number_268; 
mod even_odd_tree_1609;
mod valid_palindrome_125;
mod count_elements_with_max_frequency_3005;
mod minimum_common_value_2540;
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



struct listnode<T> {
    item: T,
    next:  Option<Box listnode<T>>
}

struct SinglyLinkedList<T> {
    list
}
