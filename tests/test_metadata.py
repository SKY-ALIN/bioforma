from bioforma import __version__, __authors__


def test_metadata():
    assert __version__ == '0.0.0'
    assert __authors__ == ['Vladimir Alinsky']
