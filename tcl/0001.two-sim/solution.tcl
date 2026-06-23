# Created by zhang-mo-peng-mo-peng-mo-mo-peng at 2026/06/21 00:18
# leetgo: 1.4.17
# https://leetcode.cn/problems/two-sum/
proc twoSum {nums target} {
    array set map {}

    lassign [list 0] i
    foreach num $nums {
        set complement [expr {$target - $num}]

        if {[info exists map($complement)]} {
            return [list $map($complement) $i]
        }

        set map($num) $i

        incr i
    }
}

puts [twoSum {2 7 11 15} 9]
puts [twoSum {3 2 4} 6]
puts [twoSum {3 3} 6]
