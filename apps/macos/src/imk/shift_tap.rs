//! 单击 Shift 的判定（配置 `[shortcut] shift_switches_english`）。
//!
//! 与 Windows 端 `ShiftTap` 同一套规矩：按下 Shift 到抬起之间没有别的键插进来，才算一次单击。
//! macOS 这边的事件源是 `flagsChanged`——控制器实现了 [`recognizedEvents:`] 把它一并要过来
//! （IMK 缺省只送 `keyDown`）。修饰键事件不分按下与抬起，只能看新的 `modifierFlags` 里还有没有 Shift。
//!
//! 状态是进程级的：IMK 每个文本框建一个控制器实例，而「Shift 按下时按了别的键没有」跨实例也得连得上
//! （按下 Shift 时焦点还在原来的框里）。IMK 的回调都在主线程，`Cell` 够用。

use std::cell::Cell;

thread_local! {
    /// 按下 Shift 后还没有别的键插进来。
    static ALONE: Cell<bool> = const { Cell::new(false) };
}

/// 非修饰键按下：这次 Shift 不再是单独按的（`⇧2`、`⇧+数字`删候选、`⇧Tab` 翻页都走这里）。
pub fn note_key_down() {
    ALONE.set(false);
}

/// 修饰键状态变了。`shift` 是新状态里 Shift 还按不按着，`others` 是有没有别的修饰键按着。
/// Shift 单独抬起时返回 `true`，一次抬起只算一次。
pub fn flags_changed(shift: bool, others: bool) -> bool {
    if others {
        // ⇧⌘ 这类组合：Shift 不算单独按的，另一个修饰键抬起时也不能算成单击
        ALONE.set(false);
        return false;
    }
    if shift {
        ALONE.set(true);
        false
    } else {
        ALONE.replace(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 每个用例开头把跨用例残留的状态清掉（同一线程跑多个用例）。
    fn reset() {
        ALONE.set(false);
    }

    #[test]
    fn shift_down_then_up_alone_is_a_tap() {
        reset();
        assert!(!flags_changed(true, false));
        assert!(flags_changed(false, false));
    }

    #[test]
    fn a_key_in_between_cancels_it() {
        reset();
        assert!(!flags_changed(true, false));
        note_key_down();
        assert!(!flags_changed(false, false));
    }

    #[test]
    fn one_release_counts_once() {
        reset();
        assert!(!flags_changed(true, false));
        assert!(flags_changed(false, false));
        // 别的修饰键抬起时又来一次 flagsChanged，不能再算一次单击
        assert!(!flags_changed(false, false));
    }

    #[test]
    fn combinations_with_other_modifiers_are_not_taps() {
        reset();
        assert!(!flags_changed(true, true));
        assert!(!flags_changed(false, false));
    }
}
