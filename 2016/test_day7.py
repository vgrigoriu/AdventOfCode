from day7 import find_all_abas, has_abba, supports_tls


def test_has_no_abba() -> None:
    assert has_abba("abcd") is False

def test_is_just_abba() -> None:
    assert has_abba("weew")

def test_has_abba_inside_other_stuff() -> None:
    assert has_abba("abcdfggfhijkl")

def test_no_abba_if_same_char() -> None:
    assert not has_abba("aaaa")

def test_abba_after_aaaa() -> None:
    assert has_abba("aaaabccb")

def test_abba_before_aaaa() -> None:
    assert has_abba("bccbaaaa")

def test_supports_tls_simple_abba() -> None:
    assert supports_tls("xyyx")

def test_does_not_support_tls_if_no_abba() -> None:
    assert not supports_tls("abcde")

def test_no_tls_if_abba_in_hypernet() -> None:
    assert not supports_tls("abba[cddc]")

def test_tls_if_abba_after_hypernet() -> None:
    assert supports_tls("abcd[defghi]jkkj")

def test_tls_with_two_hypernets() -> None:
    assert supports_tls("abcdef[ghijklm]noon[pqrs]tuvw")

def test_all_abas_no_aba() -> None:
    assert find_all_abas("abcd") == []

def test_find_single_abba() -> None:
    assert find_all_abas("abcbdef") == [("b", "c")]

def test_aaa_is_not_aba() -> None:
    assert find_all_abas("abcccdef") == []

def test_multiple_abas() -> None:
    assert find_all_abas("abacdefeghijiklmn") == [("a", "b"), ("e", "f"), ("i", "j")]

def test_ovelapping_abas() -> None:
    assert find_all_abas("abab") == [("a", "b"), ("b", "a")]