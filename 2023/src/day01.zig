const std = @import("std");
const ascii = std.ascii;
const fs = std.fs;
const io = std.io;

const input_path = "input/day01.txt";
const max_input_size = 1024;

fn starts_with(string: *const []const u8, offset: usize, prefix: *const []const u8) bool {
    if (string.*.len < prefix.*.len + offset) {
        return false;
    }
    for (prefix.*, 0..) |char, i| {
        if(char != string.*[i+offset]) {
            return false;
        }
    }
    return true;
}

fn get_calibration_value_part(line: *const []const u8, part: u8) usize {
    const str_digits = [_][]const u8 {
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    };
    var digit1: usize = 10;
    var digit2: usize = 10;
    for (line.*, 0..) |char, i| {
        if (ascii.isDigit(char)) {
            const digit = char - '0';
            if (digit1 == 10) {
                digit1 = digit;
            }
            else {
                digit2 = digit;
            }
        }
        else if(2 == part) {
            for (str_digits, 0..) |str_digit, j| {
                if (starts_with(line, i, &str_digit)) {
                    if (digit1 == 10) {
                        digit1 = j;
                    }
                    else {
                        digit2 = j;
                    }
                }
            }
        }
    }
    if (digit2 == 10) {
        digit2 = digit1;
    }
    const calibration_value = digit1 * 10 + digit2;
    return calibration_value;
}

pub fn main() !void {
    var file = try std.fs.cwd().openFile(input_path, .{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    var buf: [max_input_size]u8 = undefined;
    var sum1: usize = 0;
    var sum2: usize = 0;
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        sum1 += get_calibration_value_part(&line, 1);
        sum2 += get_calibration_value_part(&line, 2);
    }
    std.debug.print("Part 1: Calibration values sum: {d}\n", .{ sum1 });
    std.debug.print("Part 2: Calibration values sum: {d}\n", .{ sum2 });
}
