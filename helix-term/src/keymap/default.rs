use std::collections::HashMap;

use super::macros::keymap;
use super::{KeyTrie, Mode};
use helix_core::hashmap;

pub fn default() -> HashMap<Mode, KeyTrie> {
    let normal = keymap!({ "Normal mode"
        // "" => add_newline_above,
        // "" => add_newline_below,
        // "" => align_selections,
        // "" => align_view_bottom,
        // "" => align_view_center,
        // "" => align_view_middle,
        // "" => align_view_top,
        // "" => append_mode,
        "C-lt" => buffer_picker,
        // "" => change_selection,
        "c" => change_selection_noyank,
        "A-ret" => code_action,
        // "" => collapse_selection,
        ":" => command_mode,
        ";" => command_palette,
        // "" => commit_undo_checkpoint,
        // "" => completion, // make sense in insert mode
        "C-S-down" => copy_selection_on_next_line, // multi-cursor
        "C-S-up" => copy_selection_on_prev_line, // multi-cursor
        "F8" => dap_continue,
        // "" => dap_disable_exceptions,
        // "" => dap_edit_condition,
        // "" => dap_edit_log,
        // "" => dap_enable_exceptions,
        "F5" => dap_launch,
        "F10" => dap_next,
        // "" => dap_pause,
        "C-S-F5" => dap_restart,
        "F11" => dap_step_in,
        "S-F11" => dap_step_out,
        // "" => dap_switch_stack_frame,
        // "" => dap_switch_thread,
        "S-F5" => dap_terminate,
        "F9" => dap_toggle_breakpoint,
        // "" => dap_variables,
        // "" => decrement,
        "backspace" => delete_char_backward,
        // "" => delete_char_forward,
        "C-x" => delete_selection, // cut
        "del" => delete_selection_noyank,
        "A-backspace" | "C-backspace" => delete_word_backward,
        "A-del" | "C-del" => delete_word_forward,
        "A-6" => diagnostics_picker,
        // "" => earlier,
        // "" => ensure_selections_forward,
        // "" => expand_selection,
        "S-left" => extend_char_left,
        "S-right" => extend_char_right,
        // "" => extend_line,
        // "" => extend_line_above,
        // "" => extend_line_below,
        "S-down" => extend_line_down,
        "S-up" => extend_line_up,
        // "" => extend_next_char,
        // "" => extend_next_long_word_end,
        "A-C-S-right" => extend_next_long_word_start,
        // "" => extend_next_word_end,
        "C-S-right" => extend_next_word_start,
        // "" => extend_parent_node_end,
        // "" => extend_parent_node_start,
        // "" => extend_prev_char,
        "A-C-S-left" => extend_prev_long_word_end,
        // "" => extend_prev_long_word_start,
        "C-S-left" => extend_prev_word_end,
        // "" => extend_prev_word_start,
        "A-j" => extend_search_next,
        "A-S-j" => extend_search_prev,
        // "" => extend_till_char,
        // "" => extend_till_prev_char,
        "S-home" => extend_to_first_nonwhitespace,
        "X" => extend_to_line_bounds,
        "S-end" => extend_to_line_end,
        // "" => extend_to_line_end_newline,
        // "" => extend_to_line_start,
        // "" => extend_visual_line_down,
        // "" => extend_visual_line_up,
        // "" => file_picker, // seems redundant
        "C-." => file_picker_in_current_buffer_directory,
        "A-1" | "C-," | "C-gt" => file_picker_in_current_directory,
        "f" => find_next_char,
        "F" => find_prev_char,
        // "" => find_till_char,
        // "" => flip_selections,
        "C-t" => format_selections,
        "C-F" => global_search,
        "C-F12" => goto_declaration,
        "F12" => goto_definition,
        // "" => goto_file,
        "C-end" => goto_file_end,
        // "" => goto_file_hsplit,
        "C-home" => goto_file_start,
        // "" => goto_file_vsplit,
        // "" => goto_first_change,
        // "" => goto_first_diag,
        "home" => goto_first_nonwhitespace,
        // "" => goto_implementation,
        // "" => goto_last_accessed_file,
        // "" => goto_last_change,
        // "" => goto_last_diag,
        // "" => goto_last_line,
        "A-C-." => goto_last_modification,
        // "" => goto_last_modified_file,
        "C-g" => goto_line,
        "end" => goto_line_end,
        // "" => goto_line_end_newline,
        // "" => goto_line_start,
        "C-tab" => goto_next_buffer,
        "A-C-g" => goto_next_change, // g: git
        "A-C-c" => goto_next_class,
        "A-C-/" => goto_next_comment,
        "A-C-d" => goto_next_diag,
        "A-C-f" => goto_next_function,
        "A-C-p" => goto_next_paragraph,
        "A-C-a" => goto_next_parameter, // a: argument
        "A-C-t" => goto_next_test, // failed: try other lsp
        "A-C-G" => goto_prev_change,
        "A-C-C" => goto_prev_class,
        "A-C-S-/" => goto_prev_comment,
        "A-C-D" => goto_prev_diag,
        "A-C-F" => goto_prev_function,
        "A-C-P" => goto_prev_paragraph,
        "A-C-A" => goto_prev_parameter,
        "A-C-T" => goto_prev_test,
        "C-S-tab" => goto_previous_buffer,
        "S-F12" => goto_reference,
        // "" => goto_type_definition,
        // "" => goto_window_bottom,
        // "" => goto_window_center,
        // "" => goto_window_top,
        // "" => half_page_down,
        // "" => half_page_up,
        "F1" => hover,
        // "" => hsplit,
        // "" => hsplit_new,
        // "" => increment,
        "tab" => indent,
        "A" => insert_at_line_end,
        "I" => insert_at_line_start,
        "ins" | "i" => insert_mode,
        // "" => insert_newline,
        // "" => insert_register,
        // "" => insert_tab,
        // "" => join_selections, // put all in a single line
        // "" => join_selections_space,
        "A-left" => jump_backward,
        "A-right" => jump_forward,
        // "" => jump_view_down,
        // "" => jump_view_left,
        // "" => jump_view_right,
        // "" => jump_view_up,
        "A-2" => jumplist_picker,
        // "" => keep_primary_selection,
        // "" => keep_selections,
        // "" => kill_to_line_end,
        // "" => kill_to_line_start,
        // "" => last_picker, // too weird
        // "" => later,
        // "" => make_search_word_bounded,
        // "" => match_brackets,
        // "" => merge_consecutive_selections,
        "esc" | "A-minus" => merge_selections, // cancel multi-cursor + keeps selection
        "left" => move_char_left,
        "right" => move_char_right,
        "down" => move_line_down,
        "up" => move_line_up,
        // "" => move_next_long_word_end,
        "A-C-right" => move_next_long_word_start,
        // "" => move_next_word_end,
        "C-right" | "w" => move_next_word_start,
        "A-C-pagedown" => move_parent_node_end,
        "A-C-pageup" => move_parent_node_start,
        "A-C-left" => move_prev_long_word_end,
        // "" => move_prev_long_word_start,
        "C-left" => move_prev_word_end,
        // "" => move_prev_word_start,
        // "" => move_visual_line_down,
        // "" => move_visual_line_up,
        "O" | "C-ret"=> open_above,
        "o" | "ret" => open_below,
        "pagedown" => page_cursor_down,
        // "" => page_cursor_half_down,
        // "" => page_cursor_half_up,
        "pageup" => page_cursor_up,
        // "" => page_down,
        // "" => page_up,
        // "" => paste_after,
        "C-v" => paste_before,
        // "" => paste_clipboard_after,
        // "" => paste_clipboard_before,
        // "" => paste_primary_clipboard_after,
        "S-ins" => paste_primary_clipboard_before,
        // "" => record_macro,
        "C-y" | "U" => redo,
        // "" => remove_primary_selection,
        // "" => remove_selections,
        "F2" => rename_symbol,
        // "" => repeat_last_motion, // too dangerous
        // "" => replace, // replaces with typed character (so weird)
        // "" => replace_selections_with_clipboard,
        // "" => replace_selections_with_primary_clipboard,
        // "" => replace_with_yanked,
        // "" => replay_macro,
        // "" => reverse_selection_contents,
        // "" => rotate_selection_contents_backward,
        // "" => rotate_selection_contents_forward,
        // "" => rotate_selections_backward,
        // "" => rotate_selections_forward,
        // "" => rotate_view,
        // "" => rotate_view_reverse,
        // "" => rsearch,
        // "" => save_selection,
        "C-down" => scroll_down,
        "C-up" => scroll_up,
        "C-f" | "/" => search,
        "F3" | "n" => search_next,
        "S-F3" | "N" => search_prev,
        "C-F3" => search_selection,
        "C-a" => select_all,
        // "" => select_mode,
        // "" => select_next_sibling, // short it
        // "" => select_prev_sibling,
        "h" => select_references_to_symbol_under_cursor, // h: highlight
        // "" => select_regex,
        // "" => select_register,
        // "" => select_textobject_around,
        // "" => select_textobject_inner,
        // "" => shell_append_output,
        // "" => shell_insert_output,
        // "" => shell_keep_pipe,
        // "" => shell_pipe,
        // "" => shell_pipe_to,
        // "" => shrink_selection,
        // "" => shrink_to_line_bounds, // oposite to Shift + Xi (seems not needed)
        // "" => signature_help, // fail
        // "" => smart_tab,
        // "" => split_selection,
        // "" => split_selection_on_newline,
        // "" => surround_add,
        // "" => surround_delete,
        // "" => surround_replace,
        // "" => suspend,
        // "" => swap_view_down,
        // "" => swap_view_left,
        // "" => swap_view_right,
        // "" => swap_view_up,
        "C-u" => switch_case,
        "A-C-u" => switch_to_lowercase,
        "A-C-S-u" => switch_to_uppercase,
        // "" => symbol_picker, // i dont like this
        // "" => till_prev_char,
        // "" => toggle_block_comments,
        "C-/" => toggle_comments,
        // "" => toggle_line_comments,
        // "" => transpose_view,
        // "" => trim_selections,
        "C-z" | "u" => undo,
        "S-tab" => unindent,
        // "" => vsplit,
        // "" => vsplit_new,
        // "" => wclose,
        // "" => wonly,
        "A-^" => workspace_diagnostics_picker, // Alt + Shift + 6
        // "" => workspace_symbol_picker,
        "C-c" => yank,
        // "" => yank_joined,
        // "" => yank_joined_to_clipboard,
        // "" => yank_joined_to_primary_clipboard,
        // "" => yank_main_selection_to_clipboard,
        // "" => yank_main_selection_to_primary_clipboard,
        "C-ins" => yank_to_clipboard,
        // "" => yank_to_primary_clipboard,

        "C-F1" => { " 🔰 CHEAT SHEET - FREQUENT KEYBINDS "
           "b" => { " buffer  "
                "C-lt" => buffer_picker,
            },
            "c" => { " change, code, command, copy  "
                "c" => change_selection_noyank,
                "A-ret" => code_action,
                ":" => command_mode,
                ";" => command_palette,
                "C-S-down" => copy_selection_on_next_line, // multi-cursor
                "C-S-up" => copy_selection_on_prev_line, // multi-cursor
            },
            "d" => { " debug  "
                "F8" => dap_continue,
                "F5" => dap_launch,
                "F10" => dap_next,
                "C-S-F5" => dap_restart,
                "F11" => dap_step_in,
                "S-F11" => dap_step_out,
                "S-F5" => dap_terminate,
                "F9" => dap_toggle_breakpoint,
            },
            "D" => { " DELETE, DIAGNOSTICS  "
                "backspace" => delete_char_backward,
                "C-x" => delete_selection, // cut
                "del" => delete_selection_noyank,
                "A-backspace" | "C-backspace" => delete_word_backward,
                "A-del" | "C-del" => delete_word_forward,
                "A-6" => diagnostics_picker,
            },
            "e" => { " extend  "
                "S-left" => extend_char_left,
                "S-right" => extend_char_right,
                "S-down" => extend_line_down,
                "S-up" => extend_line_up,
                "A-C-S-right" => extend_next_long_word_start,
                "C-S-right" => extend_next_word_start,
                "A-C-S-left" => extend_prev_long_word_end,
                "C-S-left" => extend_prev_word_end,
                "A-j" => extend_search_next,
                "A-S-j" => extend_search_prev,
                "S-home" => extend_to_first_nonwhitespace,
                "X" => extend_to_line_bounds,
                "S-end" => extend_to_line_end,
            },
            "f" => { " file, find, format  "
                "C-." => file_picker_in_current_buffer_directory,
                "A-1" | "C-," | "C-gt" => file_picker_in_current_directory,
                "f" => find_next_char,
                "F" => find_prev_char,
                "C-t" => format_selections,
            },
            "g" => { " global  "
                "C-F" => global_search,
            },
            "G" => { " GOTO  "
                "C-F12" => goto_declaration,
                "F12" => goto_definition,
                "C-end" => goto_file_end,
                "C-home" => goto_file_start,
                "home" => goto_first_nonwhitespace,
                "A-C-." => goto_last_modification,
                "C-g" => goto_line,
                "end" => goto_line_end,
                "C-tab" => goto_next_buffer,
                "A-C-g" => goto_next_change, // g: git
                "A-C-c" => goto_next_class,
                "A-C-/" => goto_next_comment,
                "A-C-d" => goto_next_diag,
                "A-C-f" => goto_next_function,
                "A-C-p" => goto_next_paragraph,
                "A-C-a" => goto_next_parameter, // a: argument
                "A-C-t" => goto_next_test, // failed: try other lsp
                "A-C-G" => goto_prev_change,
                "A-C-C" => goto_prev_class,
                "A-C-S-/" => goto_prev_comment,
                "A-C-D" => goto_prev_diag,
                "A-C-F" => goto_prev_function,
                "A-C-P" => goto_prev_paragraph,
                "A-C-A" => goto_prev_parameter,
                "A-C-T" => goto_prev_test,
                "C-S-tab" => goto_previous_buffer,
                "S-F12" => goto_reference,
            },
            "h" => { " hover  "
                "F1" => hover,
            },
            "i" => { " indent, insert  "
                "tab" => indent,
                "A" => insert_at_line_end,
                "I" => insert_at_line_start,
                "ins" | "i" => insert_mode,
            },
            "j" => { " jump, jumplist  "
                "A-left" => jump_backward,
                "A-right" => jump_forward,
                "A-2" => jumplist_picker,
            },
            "m" => { " merge, move  "
                "esc" | "A-minus" => merge_selections, // cancel multi-cursor + keeps selection
                "left" => move_char_left,
                "right" => move_char_right,
                "down" => move_line_down,
                "up" => move_line_up,
                "A-C-right" => move_next_long_word_start,
                "C-right" | "w" => move_next_word_start,
                "A-C-pagedown" => move_parent_node_end,
                "A-C-pageup" => move_parent_node_start,
                "A-C-left" => move_prev_long_word_end,
                "C-left" => move_prev_word_end,
            },
            "o" => { " open  "
                "O" | "C-ret"=> open_above,
                "o" | "ret" => open_below,
            },
            "p" => { " page, paste  "
                "pagedown" => page_cursor_down,
                "pageup" => page_cursor_up,
                "C-v" => paste_before,
                "S-ins" => paste_primary_clipboard_before,
            },
            "r" => { " redo, rename  "
                "C-y" | "U" => redo,
                "F2" => rename_symbol,
            },
            "s" => { " scroll, search, select, switch  "
                "C-down" => scroll_down,
                "C-up" => scroll_up,
                "C-f" | "/" => search,
                "F3" | "n" => search_next,
                "S-F3" | "N" => search_prev,
                "C-F3" => search_selection,
                "C-a" => select_all,
                "h" => select_references_to_symbol_under_cursor, // h: highlight
                "C-u" => switch_case,
                "A-C-u" => switch_to_lowercase,
                "A-C-S-u" => switch_to_uppercase,
            },
            "t" => { " toggle  "
                "C-/" => toggle_comments,
            },
            "u" => { " undo, unindent  "
                "C-z" | "u" => undo,
                "S-tab" => unindent,
            },
            "w" => { " workspace  "
                "A-^" => workspace_diagnostics_picker, // Alt + Shift + 6
            },
            "y" => { " yank  "
                "C-c" => yank,
                "C-ins" => yank_to_clipboard,
            },
            "┈" => _menu_divider,
            "space" => _menu_sos_commands,
        },

        "space" => { " 🆘 COMMANDS - RARE KEYBINDS "
            "a" => { " add, align, append  "
                "N" => add_newline_above,
                "n" => add_newline_below,
                "A" => align_selections,
                "b" => align_view_bottom,
                "c" => align_view_center,
                "m" => align_view_middle,
                "t" => align_view_top,
                "a" => append_mode,
            },
            "c" => { " change, collapse  "
                "c" => change_selection,
                "C" => collapse_selection,
                "u" => commit_undo_checkpoint,
                "C-space" => completion, // make sense in insert mode
            },
            "d" => { " debug  "
                "x" => dap_disable_exceptions,
                "c" => dap_edit_condition,
                "l" => dap_edit_log,
                "X" => dap_enable_exceptions,
                "p" => dap_pause,
                "s" => dap_switch_stack_frame,
                "t" => dap_switch_thread,
                "v" => dap_variables,
            },
            "D" => { " DECREMENT, DELETE  "
                "d" => decrement,
                "del" => delete_char_forward,
            },
            "e" => { " earlier, ensure, expand  "
                "e" => earlier,
                "f" => ensure_selections_forward,
                "s" => expand_selection,
            },
            "E" => { " EXTEND  "
                "l" => extend_line,
                "S-up" => extend_line_above,
                "S-down" => extend_line_below,
                "right" => extend_next_char,
                "A-C-S-right" => extend_next_long_word_end,
                "C-S-right" => extend_next_word_end,
                "S-pagedown" => extend_parent_node_end,
                "S-pageup" => extend_parent_node_start,
                "S-left" => extend_prev_char,
                "A-C-S-left" => extend_prev_long_word_start,
                "C-S-left" => extend_prev_word_start,
                "t" => extend_till_char,
                "T" => extend_till_prev_char,
                "S-end" => extend_to_line_end_newline,
                "S-home" => extend_to_line_start,
                "down" => extend_visual_line_down,
                "up" => extend_visual_line_up,
            },
            "f" => { " file, find, flip  "
                "p" => file_picker, // seems redundant
                "t" => find_till_char,
                "f" => flip_selections,
            },
            "g" => { " goto  "
                "f" => goto_file,
                "v" => goto_file_vsplit,
                "A-C-S-g" => goto_first_change,
                "A-C-d" => goto_first_diag,
                "S-F12" => goto_implementation,
                "C-S-." => goto_last_accessed_file,
                "A-C-g" => goto_last_change,
                "A-C-S-d" => goto_last_diag,
                "C-end" => goto_last_line,
                "A-C-." => goto_last_modified_file,
                "end" => goto_line_end_newline,
                "home" => goto_line_start,
                "F12" => goto_type_definition,
                "b" => goto_window_bottom,
                "c" => goto_window_center,
                "t" => goto_window_top,
            },
            "h" => { " half, hsplit  "
                "pagedown" => half_page_down,
                "pageup" => half_page_up,
                "h" => hsplit,
                "H" => hsplit_new,
            },
            "i" => { " increment, insert  "
                "I" => increment,
                "i" => insert_newline,
                "r" => insert_register,
                "t" => insert_tab,
            },
            "j" => { " join, jump  "
                "j" => join_selections, // put all in a single line
                "J" => join_selections_space,
                "down" => jump_view_down,
                "left" => jump_view_left,
                "right" => jump_view_right,
                "up" => jump_view_up,
            },
            "k" => { " keep, kill  "
                "k" => keep_primary_selection,
                "K" => keep_selections,
                "end" => kill_to_line_end,
                "home" => kill_to_line_start,
            },
            "l" => { " last  "
                "p" => last_picker, // too weird
                "l" => later,
            },
            "m" => { " make, match, merge, move  "
                 "w" => make_search_word_bounded,
                 "b" => match_brackets,
                 "c" => merge_consecutive_selections,
                 "A-C-right" => move_next_long_word_end,
                 "C-right" => move_next_word_end,
                 "A-C-left" => move_prev_long_word_start,
                 "C-left" => move_prev_word_start,
                 "down" => move_visual_line_down,
                 "up" => move_visual_line_up,
            },
            "p" => { " page, paste "
                "C-pagedown" => page_cursor_half_down,
                "C-pageup" => page_cursor_half_up,
                "pagedown" => page_down,
                "pageup" => page_up,
                "C-v" => paste_after,
                "A-C-V" => paste_clipboard_after,
                "A-C-v" => paste_clipboard_before,
                "C-ins" => paste_primary_clipboard_after,
            },
            "r" => { " record, remove, repeat, replace  "
                "q" => record_macro,
                "r" => remove_primary_selection,
                "R" => remove_selections,
                "." => repeat_last_motion, // too dangerous
                "C-r" => replace, // replaces with typed character (so weird)
                "C-R" => replace_selections_with_clipboard,
                "A-C-r" => replace_selections_with_primary_clipboard,
                "C-v" => replace_with_yanked,
            },
            "R" => { " REPLAY, REVERSE, ROTATE  "
                "." => replay_macro,
                "r" => reverse_selection_contents,
                "C-left" => rotate_selection_contents_backward,
                "C-right" => rotate_selection_contents_forward,
                "A-C-left" => rotate_selections_backward,
                "A-C-right" => rotate_selections_forward,
                "v" => rotate_view,
                "R" => rotate_view_reverse,
                "/" => rsearch,
            },
            "s" => { " save, select, shell, shrink, signature  "
                "s" => save_selection,
                "v" => select_mode,
                "right" => select_next_sibling, // short it
                "left" => select_prev_sibling,
                "r" => select_regex,
                "R" => select_register,
                "a" => select_textobject_around,
                "i" => select_textobject_inner,
                "C-a" => shell_append_output,
                "C-ins" => shell_insert_output,
                "C-k" => shell_keep_pipe,
                "C-p" => shell_pipe,
                "C-P" => shell_pipe_to,
                "S" => shrink_selection,
                "B" => shrink_to_line_bounds, // oposite to Shift + Xi (seems not needed)
                "F1" => signature_help, // fail
            },
            "S" => { " SMART, SPLIT, SURROUND, SUSPEND, SWAP, SYMBOL  "
                "tab" => smart_tab,
                "s" => split_selection,
                "S" => split_selection_on_newline,
                "C-a" => surround_add,
                "C-d" => surround_delete,
                "C-r" => surround_replace,
                "C-s" => suspend,
                "down" => swap_view_down,
                "left" => swap_view_left,
                "right" => swap_view_right,
                "up" => swap_view_up,
                "t" => symbol_picker, // i dont like this
            },
            "t" => { " till, toggle, transpose, trim  "
                "T" => till_prev_char,
                "A-C-/" => toggle_block_comments,
                "C-/" => toggle_line_comments,
                "t" => transpose_view,
                "C-t" => trim_selections,
            },
            "v" => { " vsplit  "
                "v" => vsplit,
                "V" => vsplit_new,
            },
            "w" => { " wclose, wonly, workspace  "
                "w" => wclose,
                "C-w" => wonly,
                "T" => workspace_symbol_picker,
            },
            "y" => { " yank  "
                "j" => yank_joined,
                "J" => yank_joined_to_clipboard,
                "C-j" => yank_joined_to_primary_clipboard,
                "m" => yank_main_selection_to_clipboard,
                "M" => yank_main_selection_to_primary_clipboard,
                "C-m" => yank_to_primary_clipboard,
            },
            "┈" => _menu_divider,
            "C-F1" => _menu_cheat_sheet,
        },

        "m" => { " 󰾹 Match "
            "esc" => normal_mode,
        },

        "[" => { " ⮬ Goto "
            "esc" => normal_mode,
        },

        "]" => { " Goto ⮯ "
            "esc" => normal_mode,
        },
    });

    let mut select = normal.clone();
    select.merge_nodes(keymap!({ "Select mode"
        "esc" => exit_select_mode,

        // Menu (select mode)
        "g" => { "Goto"
            "esc" => exit_select_mode,
            //"" => extend_line_up,
            //"" => extend_line_down,
        },
    }));

    let insert = keymap!({ "Insert mode"
        "esc" => normal_mode,
        // "" => add_newline_above,
        "ret" => add_newline_below,
        // "" => align_selections,
        // "" => align_view_bottom,
        // "" => align_view_center,
        // "" => align_view_middle,
        // "" => align_view_top,
        // "" => append_mode,
        "C-lt" => buffer_picker,
        // "" => change_selection,
        // "" => change_selection_noyank,
        "A-ret" => code_action,
        // "" => collapse_selection,
        // "" => command_mode,
        // "" => command_palette,
        // "" => commit_undo_checkpoint,
        // "" => completion, // make sense in insert mode
        "C-S-down" => copy_selection_on_next_line, // multi-cursor
        "C-S-up" => copy_selection_on_prev_line, // multi-cursor
        "F8" => dap_continue,
        // "" => dap_disable_exceptions,
        // "" => dap_edit_condition,
        // "" => dap_edit_log,
        // "" => dap_enable_exceptions,
        "F5" => dap_launch,
        "F10" => dap_next,
        // "" => dap_pause,
        "C-S-F5" => dap_restart,
        "F11" => dap_step_in,
        "S-F11" => dap_step_out,
        // "" => dap_switch_stack_frame,
        // "" => dap_switch_thread,
        "S-F5" => dap_terminate,
        "F9" => dap_toggle_breakpoint,
        // "" => dap_variables,
        // "" => decrement,
        "backspace" => delete_char_backward,
        // "" => delete_char_forward,
        "C-x" => delete_selection, // cut
        "del" => delete_selection_noyank,
        "A-backspace" | "C-backspace" => delete_word_backward,
        "A-del" | "C-del" => delete_word_forward,
        "A-6" => diagnostics_picker,
        // "" => earlier,
        // "" => ensure_selections_forward,
        // "" => expand_selection,
        "S-left" => extend_char_left,
        "S-right" => extend_char_right,
        // "" => extend_line,
        // "" => extend_line_above,
        // "" => extend_line_below,
        "S-down" => extend_line_down,
        "S-up" => extend_line_up,
        // "" => extend_next_char,
        // "" => extend_next_long_word_end,
        "A-C-S-right" => extend_next_long_word_start,
        // "" => extend_next_word_end,
        "C-S-right" => extend_next_word_start,
        // "" => extend_parent_node_end,
        // "" => extend_parent_node_start,
        // "" => extend_prev_char,
        "A-C-S-left" => extend_prev_long_word_end,
        // "" => extend_prev_long_word_start,
        "C-S-left" => extend_prev_word_end,
        // "" => extend_prev_word_start,
        "A-j" => extend_search_next,
        "A-S-j" => extend_search_prev,
        // "" => extend_till_char,
        // "" => extend_till_prev_char,
        "S-home" => extend_to_first_nonwhitespace,
        // "" => extend_to_line_bounds,
        "S-end" => extend_to_line_end,
        // "" => extend_to_line_end_newline,
        // "" => extend_to_line_start,
        // "" => extend_visual_line_down,
        // "" => extend_visual_line_up,
        // "" => file_picker, // seems redundant
        "C-." => file_picker_in_current_buffer_directory,
        "A-1" | "C-," | "C-gt" => file_picker_in_current_directory,
        // "" => find_next_char,
        // "" => find_prev_char,
        // "" => find_till_char,
        // "" => flip_selections,
        "C-t" => format_selections,
        "C-F" => global_search,
        "C-F12" => goto_declaration,
        "F12" => goto_definition,
        // "" => goto_file,
        "C-end" => goto_file_end,
        // "" => goto_file_hsplit,
        "C-home" => goto_file_start,
        // "" => goto_file_vsplit,
        // "" => goto_first_change,
        // "" => goto_first_diag,
        "home" => goto_first_nonwhitespace,
        // "" => goto_implementation,
        // "" => goto_last_accessed_file,
        // "" => goto_last_change,
        // "" => goto_last_diag,
        // "" => goto_last_line,
        "A-C-." => goto_last_modification,
        // "" => goto_last_modified_file,
        "C-g" => goto_line,
        "end" => goto_line_end,
        // "" => goto_line_end_newline,
        // "" => goto_line_start,
        "C-tab" => goto_next_buffer,
        "A-C-g" => goto_next_change, // g: git
        "A-C-c" => goto_next_class,
        "A-C-/" => goto_next_comment,
        "A-C-d" => goto_next_diag,
        "A-C-f" => goto_next_function,
        "A-C-p" => goto_next_paragraph,
        "A-C-a" => goto_next_parameter, // a: argument
        "A-C-t" => goto_next_test, // failed: try other lsp
        "A-C-G" => goto_prev_change,
        "A-C-C" => goto_prev_class,
        "A-C-S-/" => goto_prev_comment,
        "A-C-D" => goto_prev_diag,
        "A-C-F" => goto_prev_function,
        "A-C-P" => goto_prev_paragraph,
        "A-C-A" => goto_prev_parameter,
        "A-C-T" => goto_prev_test,
        "C-S-tab" => goto_previous_buffer,
        "S-F12" => goto_reference,
        // "" => goto_type_definition,
        // "" => goto_window_bottom,
        // "" => goto_window_center,
        // "" => goto_window_top,
        // "" => half_page_down,
        // "" => half_page_up,
        "F1" => hover,
        // "" => hsplit,
        // "" => hsplit_new,
        // "" => increment,
        "tab" => indent,
        // "" => insert_at_line_end,
        // "" => insert_at_line_start,
        // "" => insert_mode,
        // "" => insert_newline,
        // "" => insert_register,
        // "" => insert_tab,
        // "" => join_selections, // put all in a single line
        // "" => join_selections_space,
        "A-left" => jump_backward,
        "A-right" => jump_forward,
        // "" => jump_view_down,
        // "" => jump_view_left,
        // "" => jump_view_right,
        // "" => jump_view_up,
        "A-2" => jumplist_picker,
        // "" => keep_primary_selection,
        // "" => keep_selections,
        // "" => kill_to_line_end,
        // "" => kill_to_line_start,
        // "" => last_picker, // too weird
        // "" => later,
        // "" => make_search_word_bounded,
        // "" => match_brackets,
        // "" => merge_consecutive_selections,
        "A-minus" => merge_selections, // cancel multi-cursor + keeps selection
        "left" => move_char_left,
        "right" => move_char_right,
        "down" => move_line_down,
        "up" => move_line_up,
        // "" => move_next_long_word_end,
        "A-C-right" => move_next_long_word_start,
        // "" => move_next_word_end,
        "C-right" => move_next_word_start,
        "A-C-pagedown" => move_parent_node_end,
        "A-C-pageup" => move_parent_node_start,
        "A-C-left" => move_prev_long_word_end,
        // "" => move_prev_long_word_start,
        "C-left" => move_prev_word_end,
        // "" => move_prev_word_start,
        // "" => move_visual_line_down,
        // "" => move_visual_line_up,
        // ""=> open_above,
        // "" => open_below,
        "pagedown" => page_cursor_down,
        // "" => page_cursor_half_down,
        // "" => page_cursor_half_up,
        "pageup" => page_cursor_up,
        // "" => page_down,
        // "" => page_up,
        // "" => paste_after,
        "C-v" => paste_before,
        // "" => paste_clipboard_after,
        // "" => paste_clipboard_before,
        // "" => paste_primary_clipboard_after,
        "S-ins" => paste_primary_clipboard_before,
        // "" => record_macro,
        "C-y" => redo,
        // "" => remove_primary_selection,
        // "" => remove_selections,
        "F2" => rename_symbol,
        // "" => repeat_last_motion, // too dangerous
        // "" => replace, // replaces with typed character (so weird)
        // "" => replace_selections_with_clipboard,
        // "" => replace_selections_with_primary_clipboard,
        // "" => replace_with_yanked,
        // "" => replay_macro,
        // "" => reverse_selection_contents,
        // "" => rotate_selection_contents_backward,
        // "" => rotate_selection_contents_forward,
        // "" => rotate_selections_backward,
        // "" => rotate_selections_forward,
        // "" => rotate_view,
        // "" => rotate_view_reverse,
        // "" => rsearch,
        // "" => save_selection,
        "C-down" => scroll_down,
        "C-up" => scroll_up,
        "C-f" | "/" => search,
        "F3" => search_next,
        "S-F3" => search_prev,
        "C-F3" => search_selection,
        "C-a" => select_all,
        // "" => select_mode,
        // "" => select_next_sibling, // short it
        // "" => select_prev_sibling,
        // "" => select_references_to_symbol_under_cursor, // h: highlight
        // "" => select_regex,
        // "" => select_register,
        // "" => select_textobject_around,
        // "" => select_textobject_inner,
        // "" => shell_append_output,
        // "" => shell_insert_output,
        // "" => shell_keep_pipe,
        // "" => shell_pipe,
        // "" => shell_pipe_to,
        // "" => shrink_selection,
        // "" => shrink_to_line_bounds, // oposite to Shift + Xi (seems not needed)
        // "" => signature_help, // fail
        // "" => smart_tab,
        // "" => split_selection,
        // "" => split_selection_on_newline,
        // "" => surround_add,
        // "" => surround_delete,
        // "" => surround_replace,
        // "" => suspend,
        // "" => swap_view_down,
        // "" => swap_view_left,
        // "" => swap_view_right,
        // "" => swap_view_up,
        "C-u" => switch_case,
        "A-C-u" => switch_to_lowercase,
        "A-C-S-u" => switch_to_uppercase,
        // "" => symbol_picker, // i dont like this
        // "" => till_prev_char,
        // "" => toggle_block_comments,
        "C-/" => toggle_comments,
        // "" => toggle_line_comments,
        // "" => transpose_view,
        // "" => trim_selections,
        "C-z" => undo,
        "S-tab" => unindent,
        // "" => vsplit,
        // "" => vsplit_new,
        // "" => wclose,
        // "" => wonly,
        "A-^" => workspace_diagnostics_picker, // Alt + Shift + 6
        // "" => workspace_symbol_picker,
        "C-c" => yank,
        // "" => yank_joined,
        // "" => yank_joined_to_clipboard,
        // "" => yank_joined_to_primary_clipboard,
        // "" => yank_main_selection_to_clipboard,
        // "" => yank_main_selection_to_primary_clipboard,
        "C-ins" => yank_to_clipboard,
        // "" => yank_to_primary_clipboard,
    });
    hashmap!(
        Mode::Normal => normal,
        Mode::Select => select,
        Mode::Insert => insert,
    )
}
