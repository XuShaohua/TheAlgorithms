#!/usr/bin/env python3

def solution(digits):
    carrier = 1
    for i in range(len(digits) - 1, -1, -1):
        digit = digits[i] + carrier
        if digit < 10:
            digits[i] = digit
            break
        else:
            #digits[i] = digit % 10
            #carrier = digit // 10
            digits[i] = digit - 10
            carrier = 1
    if carrier != 0:
        digits.insert(0, carrier)
    return digits

def main():
    digits = [1, 2, 3]
    ret = solution(digits)
    print(ret)

if __name__ == "__main__":
    main()
