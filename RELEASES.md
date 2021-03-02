Unreleased: guion 0.4.0 (2021-03-0X)
=========================

https://github.com/FerionVE/guion/compare/53dcae61...master

API changes (major)
-------------------

- Style reconcept: Style and Selector (#17)
- Rework resolving to use Path fns instead of relying on path fragments (#20)
- GuionError: Informative resolve/traitcast errors (#21)
- Widget trait method based tabulate, possible for trait implementations to customize e.g. the next child in a specific direction (#23)

API changes (minor)
-------------------

- Relax mandatory trait bounds (#25)
- Widget: improve debug_type_name (d3fccbc7)

Fixes
-----

- StdPath::tip: fix panic (c6c9141d)
- widgets: event: fix response Button and TextBox (#24)

Misc
----

- Improved Doc