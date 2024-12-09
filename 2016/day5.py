import hashlib
from input import read_aoc_input

def md5(s: str) -> str:
    hash = hashlib.md5()
    hash.update(s.encode("utf-8"))
    return hash.hexdigest()

door_id = read_aoc_input()

password = []
i = 0
while len(password) < 8:
    hash = md5(f"{door_id}{i}")
    if hash.startswith("00000"):
        password.append(hash[5])

    i += 1

print("".join(password))

password = ["_" for _ in range(8)]
i = 0
while "_" in password:
    hash = md5(f"{door_id}{i}")
    if hash.startswith("00000") and hash[5].isdigit():
        pos = int(hash[5])
        if 0 <= pos < len(password) and password[pos] == "_":
            password[pos] = hash[6]

    i += 1

print("".join(password))