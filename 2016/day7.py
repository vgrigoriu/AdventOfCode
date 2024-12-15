import re

from input import read_aoc_input


def has_abba(sequence: str) -> bool:
    m = re.match(".*(.)(?!\\1)(.)\\2\\1.*", sequence)
    return m is not None


def find_all_abas(sequence: str) -> list[(str, str)]:
    aba_regex = r"(.)(?!\1)(.)\1"
    matches = re.finditer(f"(?={aba_regex})", sequence)
    return [(m[1], m[2]) for m in matches]


def find_all_babs(sequence: str) -> list[(str, str)]:
    return [(b, a) for a, b in find_all_abas(sequence)]


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


def supports_ssl(ip_address: str) -> bool:
    sequences = re.split("[\\[\\]]", ip_address)
    supernets = sequences[0::2]
    hypernets = sequences[1::2]

    abas = {aba for seq in supernets for aba in find_all_abas(seq)}
    babs = {bab for seq in hypernets for bab in find_all_babs(seq)}

    return bool(abas.intersection(babs))


ip_addresses = read_aoc_input()

ip_addresses_with_tls = [addr for addr in ip_addresses if supports_tls(addr)]
print(len(ip_addresses_with_tls))

ip_addreses_with_ssl = [addr for addr in ip_addresses if supports_ssl(addr)]
print(len(ip_addreses_with_ssl))
