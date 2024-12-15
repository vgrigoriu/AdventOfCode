import re

from input import read_aoc_input


def has_abba(sequence: str) -> bool:
    m = re.match(".*(.)(?!\\1)(.)\\2\\1.*", sequence)
    return m is not None

def supports_tls(ip_address: str) -> bool:
    """Must have xyyx sequence, but not inside square brackets"""
    result = False

    sequences = re.split("[\\[\\]]", ip_address)
    for i, seq in enumerate(sequences):
        is_hypernet = i % 2 == 1
        seq_has_abba = has_abba(seq)
        if is_hypernet and seq_has_abba:
            return False
        if seq_has_abba:
            result = True

    return result

ip_addresses = read_aoc_input()

ip_addresses_with_tls = [
    addr for addr in ip_addresses if supports_tls(addr)
]

print(len(ip_addresses_with_tls))