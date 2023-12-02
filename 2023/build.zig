const Builder = @import("std").build.Builder;

pub fn build(b: *Builder) void {
    const exe_day01 = b.addExecutable(.{
        .name = "day01",
        .root_source_file = .{ .path = "src/day01.zig" },
    });
    b.installArtifact(exe_day01);
    const run_day01 = b.addRunArtifact(exe_day01);
    const run_day01_step = b.step("run", "Run the compiled application for day 1");
    run_day01_step.dependOn(&run_day01.step);
}