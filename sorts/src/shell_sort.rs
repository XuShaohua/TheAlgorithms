// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

/// Shell sort is a simple extension to insertion sort that allows exchanging
/// elements that far apart. It produces partially sorted array (h-sorted array).
///
/// 拆解成由 h 个元素隔开的序列, 依次降低h间隔的值, 直到其为1.
/// 主要是为了减少元素交换的次数.
/// 最差情况下 O(N^(3/2))
///
/// 这里的 h 值是由大到小变化的, 就是说, 每次移动的步长是h, 就是为了减少元素被
/// 移动的次数, 当 h = 1 时, 整个序列就完成排序了.
pub fn shell_sort<T>(list: &mut [T])
where
    T: PartialOrd,
{
    let len = list.len();

    // Calculate the first h value,
    let mut h = 1;
    while h < len / 3 {
        h = 3 * h + 1;
    }
    println!("h0 = {h}, len: {len}");

    while h >= 1 {
        // h-sort the array
        for i in h..len {
            // for (j = i; j >= h; j -= h)
            let mut j = i;
            while j >= h && list[j - h] > list[j] {
                list.swap(j - h, j);
                j -= h;
            }
        }

        h /= 3;
        println!("h = {h}");
    }
}

#[cfg(test)]
mod tests {
    use super::shell_sort;

    #[test]
    fn test_shell_sort() {
        let mut list = [0, 5, 3, 2, 2];
        shell_sort(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        shell_sort(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998166, -996360, -995703, -995238, -995066, -994740, -992987, -983833, -987905,
            -980069, -977640,
        ];
        shell_sort(&mut list);
        assert_eq!(
            list,
            [
                -998166, -996360, -995703, -995238, -995066, -994740, -992987, -987905, -983833,
                -980069, -977640,
            ]
        );

        let mut list = "EASYQUESTION".chars().collect::<Vec<_>>();
        shell_sort(&mut list);
        assert_eq!(
            list,
            ['A', 'E', 'E', 'I', 'N', 'O', 'Q', 'S', 'S', 'T', 'U', 'Y']
        );
    }
}
