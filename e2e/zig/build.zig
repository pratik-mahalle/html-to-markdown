const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const test_step = b.step("test", "Run tests");

    const conversion_module = b.createModule(.{
        .root_source_file = b.path("src/conversion_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const conversion_tests = b.addTest(.{
        .root_module = conversion_module,
    });
    const conversion_run = b.addRunArtifact(conversion_tests);
    test_step.dependOn(&conversion_run.step);

    const edge_cases_module = b.createModule(.{
        .root_source_file = b.path("src/edge_cases_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const edge_cases_tests = b.addTest(.{
        .root_module = edge_cases_module,
    });
    const edge_cases_run = b.addRunArtifact(edge_cases_tests);
    test_step.dependOn(&edge_cases_run.step);

    const metadata_module = b.createModule(.{
        .root_source_file = b.path("src/metadata_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const metadata_tests = b.addTest(.{
        .root_module = metadata_module,
    });
    const metadata_run = b.addRunArtifact(metadata_tests);
    test_step.dependOn(&metadata_run.step);

    const options_module = b.createModule(.{
        .root_source_file = b.path("src/options_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const options_tests = b.addTest(.{
        .root_module = options_module,
    });
    const options_run = b.addRunArtifact(options_tests);
    test_step.dependOn(&options_run.step);

    const real_world_module = b.createModule(.{
        .root_source_file = b.path("src/real_world_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const real_world_tests = b.addTest(.{
        .root_module = real_world_module,
    });
    const real_world_run = b.addRunArtifact(real_world_tests);
    test_step.dependOn(&real_world_run.step);

    const result_module = b.createModule(.{
        .root_source_file = b.path("src/result_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const result_tests = b.addTest(.{
        .root_module = result_module,
    });
    const result_run = b.addRunArtifact(result_tests);
    test_step.dependOn(&result_run.step);

    const smoke_module = b.createModule(.{
        .root_source_file = b.path("src/smoke_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const smoke_tests = b.addTest(.{
        .root_module = smoke_module,
    });
    const smoke_run = b.addRunArtifact(smoke_tests);
    test_step.dependOn(&smoke_run.step);

    const structure_module = b.createModule(.{
        .root_source_file = b.path("src/structure_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const structure_tests = b.addTest(.{
        .root_module = structure_module,
    });
    const structure_run = b.addRunArtifact(structure_tests);
    test_step.dependOn(&structure_run.step);

    const visitor_module = b.createModule(.{
        .root_source_file = b.path("src/visitor_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const visitor_tests = b.addTest(.{
        .root_module = visitor_module,
    });
    const visitor_run = b.addRunArtifact(visitor_tests);
    test_step.dependOn(&visitor_run.step);

}
