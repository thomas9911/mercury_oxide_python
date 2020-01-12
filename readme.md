

## Examples


```python
>>> import mercury_oxide
>>> mercury_oxide.render("Hallo {{person | capitalize}}", {"person": "luke"})
'Hallo Luke'

```

Or with objects

```python
>>> import mercury_oxide
>>> mercury_oxide.render("Hallo {{person.name | capitalize}}", {"person": {"name": "luke", "sirname": "skywalker"}})
'Hallo Luke'

```

Or even with partials

```python
>>> import mercury_oxide
>>> partial = "Hallo {{person | capitalize}}"
>>> mercury_oxide.render("{% include 'greeting' %}, have a nice day!", {"person": "luke"}, {"greeting": partial})
'Hallo Luke, have a nice day!'

```