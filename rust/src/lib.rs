#![warn(unused)]
use std::cell::RefCell;
use std::rc::Rc;

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

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}



//big chungus

mod add_binary_67;
mod add_one_row_to_tree_623;
mod boats_to_save_people_881;
mod build_array_from_permutation_1920;
mod compare_version_numbers_165;
mod contains_duplicate_217;
mod count_elements_with_max_frequency_3005;
mod custom_sort_string_791;
mod design_parking_system_1603;
mod divide_array_into_arrays_with_max_difference_2966;
mod double_a_number_represented_as_a_linked_list_2816;
mod evaluate_reverse_polish_notation_150;
mod even_odd_tree_1609;
mod find_all_duplicates_in_array_442;
mod find_bottom_left_tree_value_513;
mod find_first_palindromic_string_in_the_array_2108;
mod find_index_of_first_occurence_in_string_28;
mod find_pivot_index_724;
mod find_pivot_integer_2485;
mod find_the_difference_389;
mod find_the_duplicate_number_287;
mod first_missing_positive_41;
mod first_unique_character_in_a_string_387;
mod flip_string_to_monotone_increasing_926;
mod group_anagrams_49;
mod house_robber_198;
mod implement_queue_using_stacks_232;
mod insert_delete_get_random_380;
mod insert_interval_57;
mod integer_to_roman_12;
mod intersection_of_two_arrays_349;
mod isomorphic_strings_205;
mod kth_smallest_prime_fraction_786;
mod largest_local_values_in_a_matrix_2373;
mod least_number_of_unique_integers_after_k_removals_1481;
mod length_of_last_word_58;
mod longest_common_subsequence_1143;
mod longest_ideal_subsequence_2370;
mod longest_path_with_different_adjacent_characters_2246;
mod majority_element_169;
mod make_the_string_great_1544;
mod maximal_rectangle_85;
mod maximize_happiness_of_selected_children_3075;
mod maximum_nesting_depth_of_parenthesis_1614;
mod merge_strings_alternately_1768;
mod min_operations_to_make_arr_empty_2870;
mod minimum_common_value_2540;
mod minimum_length_of_string_after_deleting_similar_ends_1750;
mod missing_number_268;
mod nodes_in_subtree_with_same_label_1519;
mod nth_tribonacci_number_1137;
mod number_of_students_unable_to_eat_lunch_1700;
mod open_the_lock_752;
mod palindrome_number_9;
mod palindromic_strings_647;
mod range_sum_of_bst_938;
mod rearrange_array_elements_by_sign_2149;
mod relative_ranks_506;
mod remove_nodes_from_a_linked_list_2487;
mod reverse_prefix_of_word_2000;
mod richest_customer_wealth_1672;
mod roman_to_integer_13;
mod root_equals_sum_of_children_2236;
mod running_sum_of_1_d_array_1480;
mod sequential_digits_1291;
mod set_mismatch_645;
mod sign_of_the_product_of_an_array_1822;
mod sort_characters_by_frequency_451;
mod subarray_sums_divisible_by_k_974;
mod sum_of_left_leaves_404;
mod sum_root_to_leaf_numbers_129;
mod top_k_frequent_elements_347;
mod two_sum_1;
mod valid_anagram_242;
mod valid_palindrome_125;
mod valid_parenthesis_string_678;
mod word_pattern_290;
mod word_search_79;
mod x_of_a_kind_in_a_deck_of_cards_914;
mod path_with_maximum_gold_1219;
mod find_the_safest_path_in_grid_2812;
mod evaluate_boolean_binary_tree_2331;
mod subsets_78;
mod reverse_string_344;
mod score_of_a_string_3110;
mod sort_code_75;
mod minimum_operations_to_make_the_array_increasing_1827;
mod minimum_increment_to_make_array_unique_945;
mod minimum_difference_between_largest_and_smallest_value_in_three_moves_1509;
mod pass_the_pillow_2582;
mod water_bottles_1518;
mod merge_nodes_in_between_zeros_2181;
mod find_the_winner_of_the_circular_game_1823;
mod average_waiting_time_1701;
mod sort_the_people_2418;
mod find_words_containing_character_2942;
mod type_of_triangle_3024;
mod minimum_string_length_after_removing_substrings_2696;
mod check_if_n_and_its_double_exist_1346;
mod check_if_a_word_occurs_as_a_prefix_of_any_word_in_a_sentence_1455;
mod two_1;
mod add_two_numbers_2;
mod find_building_where_alice_and_bob_can_meet_2940;
mod valid_sudoku_36;
mod find_largest_value_in_each_tree_row_515;
mod count_good_triplets_1534;
mod count_special_quadruplets_1995;
mod remove_element_27;
mod move_zeroes_283;
mod final_value_of_variable_after_performing_operations_2011;
mod merge_sorted_array_88;
