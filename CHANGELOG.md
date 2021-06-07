# v0.5.0

* BREAKING: Reduced numerical error in Lat/Lon representation. osmio now
  matches OpenStreetMap's internal precision model, storing location as a
  32-bit integer of 100 nano-degree units. If you need decimal degrees,
  convert a Lat/Lon to f64 with `lat.degrees()`.
  This reduces the numerical error of the representation from a worst case of
  about 1 meter to worst case of about 1 centimeter (see
  https://wiki.openstreetmap.org/wiki/Node).
* ADDED: `way.is_closed()` and `way.is_area()` to work with `Way`s which
  represent two dimensional shapes.

# v0.4

* PBF reader returns `Arc`

# v0.2.1 (2020-01-21)

* Refactor PBF objects internally to be more effecient

# v0.2.0 (2020-01-06)

* ISO timeformats are more clearer without milliseconds
* `OSMObjType` now implemented `Display` (`"node"`/`"way"`/`"relation"`) and `Debug` implemention changed to be single character representing type (`"n"`/`"w"`/`"r"`), and are sortable.
* `tags` (& `members`) for objects are now an `ExactSizedIterator`
* New helper methods: `tagged` & `untagged`
* Object readers now have `.inner()`, returning a ref to their inner `Read` object they're reading from (not fully implemented)

# v0.1.0 (2019-10-06)

* Initial work
