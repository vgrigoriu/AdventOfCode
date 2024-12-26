import hashlib
import re
from collections import defaultdict


def md5hash(salt: str, index: int) -> str:
    return hashlib.md5(f"{salt}{index}".encode()).hexdigest()

def md5hash_2016(salt: str, index: int) -> str:
    result = f"{salt}{index}"
    for _ in range(2017):
        result = hashlib.md5(result.encode()).hexdigest()

    return result

def test_md5hash() -> None:
    assert "888" in md5hash("abc", 18)
    assert "eee" in md5hash("abc", 39)
    assert "eeeee" in md5hash("abc", 816)

def test_md5hash_2016() -> None:
    assert "222" in md5hash_2016("abc", 5)
    assert "eee" in md5hash_2016("abc", 10)
    assert "fffff" in md5hash_2016("abc", 22859)

def first_triplet(hash: str) -> str | None:
    if m := re.search(r"([a-f0-9])\1\1", hash):
        return m.group(1)
    return None

def all_fivers(hash: str) -> list[str]:
    return [m.group(1) for m in re.finditer(r"([a-f0-9])\1\1\1\1", hash)]

def test_first_triplet() -> None:
    assert first_triplet(md5hash("abc", 18)) == "8"
    assert first_triplet(md5hash("abc", 39)) == "e"

    # no triplet from 0 to 17
    for index in range(18):
        assert first_triplet(md5hash("abc", index)) is None

def test_all_fivers() -> None:
    assert all_fivers("") == []
    assert all_fivers("11111") == ["1"]
    assert all_fivers("bbbbb") == ["b"]
    assert all_fivers("cccc") == []
    assert all_fivers("00000ccccc") == ["0", "c"]
    assert all_fivers("qqqqq") == []
    assert all_fivers("abc22222def333330124444eeeee") == ["2", "3", "e"]

salt = "ahsbgdzn"

# keep indexes of keys as a set, will sort at the end
key_indexes = set()

# dictionary with key -> char that appears 3 times and value -> list of indexes where it
# appears 3 times
candidates = defaultdict(list)

index = 0
keys_to_find = 64
while len(key_indexes) < keys_to_find:
    # change to md5hash() for part 1
    hash = md5hash_2016(salt, index)

    fivers = all_fivers(hash)
    for ch in fivers:
        # eliminate candidates more than 1000 ago
        candidates[ch] = [i for i in candidates[ch] if index - i <= 1000]
        # eliminate what's already found
        keys = set(candidates[ch]).difference(key_indexes)
        # what's left are new keys
        key_indexes.update(keys)

    # add new candidate after we checked, so it does not confirm itself
    if ch := first_triplet(hash):
        candidates[ch].append(index)

    index += 1

# keep searching, we might find some more keys, but don't create new candidates
for _ in range(1000):
    hash = md5hash_2016(salt, index)

    fivers = all_fivers(hash)
    for ch in fivers:
        # eliminate candidates more than 1000 ago
        candidates[ch] = [i for i in candidates[ch] if index - i <= 1000]
        # eliminate what's already found
        keys = set(candidates[ch]).difference(key_indexes)
        # what's left are new keys
        key_indexes.update(keys)

    index += 1

print(sorted(key_indexes)[keys_to_find - 1])