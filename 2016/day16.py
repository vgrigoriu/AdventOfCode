def reverse(data: str) -> str:
    return str(data[::-1])


def test_reverse() -> None:
    assert reverse("") == ""
    assert reverse("0") == "0"
    assert reverse("1") == "1"
    assert reverse("01") == "10"
    assert reverse("10") == "01"
    assert reverse("11") == "11"
    assert reverse("100") == "001"
    assert reverse("101") == "101"


def invert(data: str) -> str:
    return data.translate({ord("0"): "1", ord("1"): "0"})


def test_invert() -> None:
    assert invert("") == ""
    assert invert("0") == "1"
    assert invert("1") == "0"
    assert invert("01") == "10"
    assert invert("10") == "01"
    assert invert("100") == "011"
    assert invert("00010") == "11101"


def generate(data: str) -> str:
    return data + "0" + invert(reverse(data))


def test_generate() -> None:
    assert generate("1") == "100"
    assert generate("0") == "001"
    assert generate("11111") == "11111000000"
    assert generate("111100001010") == "1111000010100101011110000"


def checksum_one_step(data: str) -> str:
    return "".join(
        "1" if b1 == b2 else "0" for b1, b2 in zip(data[::2], data[1::2], strict=True)
    )


def test_checksum_one_step() -> None:
    assert checksum_one_step("110010110100") == "110101"


def checksum(data: str) -> str:
    result = checksum_one_step(data)

    while len(result) % 2 == 0:
        result = checksum_one_step(result)

    return result


def test_checksum() -> None:
    assert checksum("110010110100") == "100"


def fill_disk_and_get_checksum(disk_length: 20, state: str) -> str:
    data = state
    while len(data) < disk_length:
        data = generate(data)

    data = data[:disk_length]

    return checksum(data)


def test_fill_etc() -> None:
    assert fill_disk_and_get_checksum(20, "10000") == "01100"


print(fill_disk_and_get_checksum(272, "10011111011011001"))
print(fill_disk_and_get_checksum(35651584, "10011111011011001"))
