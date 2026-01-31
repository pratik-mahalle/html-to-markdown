import json
import subprocess
import sys


def load_outdated_packages(project_path: str) -> list[tuple[str, str]]:
    result = subprocess.run(  # noqa: S603
        ["dotnet", "list", project_path, "package", "--outdated", "--format", "json"],
        check=True,
        capture_output=True,
        text=True,
    )
    data = json.loads(result.stdout)
    packages: dict[str, str] = {}
    for project in data.get("projects", []):
        for framework in project.get("frameworks", []) or []:
            for package in framework.get("topLevelPackages", []) or []:
                latest = package.get("latestVersion")
                if not latest:
                    continue
                resolved = package.get("resolvedVersion") or package.get("requestedVersion")
                if resolved == latest:
                    continue
                package_id = package.get("id")
                if package_id:
                    packages[package_id] = latest
    return sorted(packages.items())


def update_project(project_path: str) -> bool:
    updated = False
    for package_id, latest_version in load_outdated_packages(project_path):
        subprocess.run(  # noqa: S603
            [
                "dotnet",
                "add",
                project_path,
                "package",
                package_id,
                "--version",
                latest_version,
            ],
            check=True,
        )
        updated = True
    return updated


def main() -> int:
    if len(sys.argv) < 2:
        print("Usage: update_dotnet_packages.py <path> [<path>...]", file=sys.stderr)
        return 2
    any_updates = False
    for project_path in sys.argv[1:]:
        if update_project(project_path):
            any_updates = True
    if not any_updates:
        print("No .NET package updates found.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
