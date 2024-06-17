import os
import pytest
import subprocess


@pytest.fixture
def ccwc_path():
    return (
        os.environ["CCWC_PATH"]
        if os.environ.get("CCWC_PATH")
        else "./target/release/ccwc-rust.exe"
    )


@pytest.fixture
def test_data_path():
    return (
        os.environ["TEST_DATA_PATH"]
        if os.environ.get("TEST_DATA_PATH")
        else "./integration_tests/test.txt"
    )


def test_ccwc_rust_bytes(ccwc_path, test_data_path):
    result = subprocess.run([ccwc_path, "-c", test_data_path], stdout=subprocess.PIPE)

    assert result.stdout.strip() == "342190 {0}".format(test_data_path).encode(
        "utf-8"
    )


def test_ccwc_rust_lines(ccwc_path, test_data_path):
    result = subprocess.run([ccwc_path, "-l", test_data_path], stdout=subprocess.PIPE)

    assert result.stdout.strip() == "7145 {0}".format(test_data_path).encode("utf-8")


def test_ccwc_rust_words(ccwc_path, test_data_path):
    result = subprocess.run([ccwc_path, "-w", test_data_path], stdout=subprocess.PIPE)

    assert result.stdout.strip() == "58164 {0}".format(test_data_path).encode("utf-8")


def test_ccwc_rust_characters(ccwc_path, test_data_path):
    with open(test_data_path, encoding="utf-8", newline="") as testFile:
        testContents = testFile.read()

    result = subprocess.run([ccwc_path, "-m", test_data_path], stdout=subprocess.PIPE)

    assert result.stdout.strip() == "{0} {1}".format(
        len(testContents), test_data_path
    ).encode("utf-8")


def test_ccwc_rust_filename_only(ccwc_path, test_data_path):
    result = subprocess.run([ccwc_path, test_data_path], stdout=subprocess.PIPE)

    assert result.stdout.strip() == "7145 58164 342190 {0}".format(
        test_data_path
    ).encode("utf-8")


def test_ccwc_rust_lines_from_stdin(ccwc_path, test_data_path):
    with open(test_data_path) as testContents:
        result = subprocess.run(
            [ccwc_path, "-l"],
            stdout=subprocess.PIPE,
            stdin=testContents,
        )

    assert result.stdout.strip() == "7145".encode("utf-8")
