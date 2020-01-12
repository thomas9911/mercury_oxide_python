import mercury_oxide


def test_works_with_vars_as_strings():
    assert "hey test" == mercury_oxide.render("hey {{ ok }}", {"ok": "test"})


def test_works_with_vars_as_map():
    assert "hey nested" == mercury_oxide.render(
        "hey {{ ok.test }}", {"ok": {"test": "nested"}}
    )


def test_works_with_vars_as_strings_and_templates():
    assert "hey test" == mercury_oxide.render(
        "{% include 'partial' %}", {"ok": "test"}, {"partial": "hey {{ ok }}"}
    )
