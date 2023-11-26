# Changelog 

# Unreleased

* Expose inner function so users can use inner latlongs w/o temp allocations

# v0.11.0 (2023-11-23)

* Speed up when reading just nodes/ways/relations.
* `.object_type()` moved to method on `OSMObjBase`

# v0.10.0 (2023-11-14)

* Internal refactoring to speed up
* Can now read a PBF file and just get the node ids & positions

# v0.9.0 (2023-10-31)

* Relicence from AGPL to MIT/Apache2.
* PBF reader returns StringOSMObj

# v0.8.1 (2023-07-25)

* Added `ChangesetReader::from_bz2_reader` method

# v0.8.0 (2023-07-25)

* Internal refactoring, and dependency updates
* `ChangesetReader` has methods `get_ref` & `into_inner` to access the original
  source reader

# v0.7.0 (2022-02-25)

* Add a `prelude` to make it easier to use
* Add `osmio::read_pbf`/`_bz2` to easily read file paths
* Can convert from Arc\* → String\* objects
* Improve doc comments, modernise style & formatting
* Many objects now derive serde's Serialize & Deserialize

# v0.6.0 (2021-08-29)

* Added conveniece methods to make Node lat/lons easier to work with & more
  ergonomic
* Refactored the `osc` output, which fixes bug with amperstands in usernames
  not being encoded.

# v0.5.0 (2021-08-14)

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
