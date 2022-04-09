# Diesel 2.0.0 RC-0

This release marks the first release candidate for the upcoming Diesel 2.0 Release. 
Diesel 2.0.0 contains the contributions of more than 130 people. More than 1700 commits were submitted 
over a span of 3 years.

As part of this release we introduced numerous new features and rewrote large parts of the internal structure.
Checkout our [changelog for a complete list of changes](https://github.com/diesel-rs/diesel/blob/master/CHANGELOG.md)
As this is a new major diesel release it contains a number of breaking changes. Checkout 
our [draft migration guide](https://github.com/diesel-rs/diesel/blob/master/guide_drafts/migration_guide.md) for details
about how to handle those breaking changes.

This release marks a first prerelease of the upcoming Diesel 2.0 release. We ask you for your help to finalise the release.
Checkout the "Timeline for a diesel 2.0 release" section for details about how you can help us finishing the release.

# Features
As a highlight diesel 2.0.0 adds support for the following features:

* Fully type checked `GROUP BY` support
* Support for table aliasing
* Support for defining select clauses via a corresponding type
* Support for `UNION`/`INTERSECT` queries

## Fully type checked `GROUP BY` support

Diesel 2.0 adds support for `GROUP BY` clauses for select queries. 

This means queries like the following one will just work.

```rust
 users::table.inner_join(posts::table)
    .group_by(users::id)
    .select((users::name, count(posts::id)))
```

As this is the case for all other diesel built-in query dsl, this construct is fully checked at compile time. This means diesel 
will ensure that the `GROUP BY` clause is valid for the current query and it will also ensure that expressions appearing inside 
of your `SELECT` clause will match the aggregation rules provided by the current `GROUP BY` clause.

## Table aliasing

Diesel 2.0 adds support for table aliasing. This enables users to write queries, where a table appears more than once in the corresponding
`FROM` clause. For this diesel provides a `diesel::alias!` macro that allows to define new alias for existing tables.

The following query demonstrates the support for this feature:

```rust
// Define new table alias for the existing `users` table
let users1 = diesel::alias!(schema::users as user1);

// Use the corresponding alias inside any existing query
users::table
    .inner_join(users1.on(users::id).eq(users1.field(users::id))))
    .select((users::id, users::name, users1.field(users::name)))
    .order_by(users1.field(users::id))
```

Again all of this is checked at compile time. So similar to a normal table, columns from aliases are only allowed to appear if
the corresponding query actually uses the alias.

## Selectable

Diesel 2.0 features a new `Selectable` trait and derive that lets users declare that a type expects a certain kind of select clause. 
The major use case for this feature is to ensure that columns from a specific query are always requested in the right order
for a corresponding type implementing `Queryable`. This also works for complex queries involving joins or other kinds of nesting.

```rust
#[derive(Queryable, PartialEq, Debug, Selectable)]
struct User {
    id: i32,
    name: String,
}

let first_user = users.select(User::as_select()).first(connection)?;
```

Diesel enforces at type system level that once you provided such a select clause via `User::as_select()` you are only allowed 
to construct this type from the returned result of the corresponding query. This means there is no need to specify the `User` type 
twice in the query above.

## Support for `UNION`/`INTERSECT`/`EXCEPT` queries

Diesel 2.0 extents the query builder to support query combinations via `UNION`/`INTERSECT`/`EXCEPT`. This allows you 
to easily chain multiple queries together as long as they return fields of the same type. Queries like the following 
one are now supported:

```rust
 users.select(user_name.nullable())
    .union(animals.select(animal_name).filter(animal_name.is_not_null()))
```

As always this is checked at compile time to reject invalid queries, like for example that ones containing select
clauses with different fields.

# Timeline for a diesel 2.0.0 release

We consider this release candidate of diesel 2.0 as feature complete. 
Before finally releasing a stable 2.0 release we want to address the following points:

* Updating the guides on our web page to use the new diesel 2.0 release
* Improve the migration guide to cover missed details or changes
* Improve our documentation to include more details on new features

You can help us rounding up this release by:

* Updating your code to the RC release and report problems [here](TODO: add link to discussion thread)
* Help us improving the documentation by submitting reports and improvements
* Help us porting our guides to the new release by submitting a PR [here](TODO: add a link to a 2.0 branch on the webpage)

# Thanks 

Thank you to everyone who helped make this release happen through bug reports, and discussion on Gitter. While we don't have a way to collect stats on that form of contribution, it's greatly appreciated.

In addition to the Diesel core team, 133 people contributed code to this release. A huge thank you to:

* Alessandro Menezes 
* Alexander 'z33ky' Hirsch
* Alexei Pastuchov
* Alice Ryhl 
* Amila Welihinda 
* Andre Braga Reis 
* Andreas Runfalk 
* Andrew Safigan
* Andrew Speed 
* Andy Russell 
* Artem Vorotnikov 
* Arve Seljebu 
* Billy Chan 
* Blas Rodriguez Irizar 
* Bryan Henry 
* Callym 
* Caroline Glassberg-Powell
* Cassie Jones 
* Chenxi Yuan
* Chris Hanks 
* Chris Maddox 
* Chris West (Faux) 
* Clouds Flowing 
* Corentin Henry 
* Daniel Buse 
* Danilo Bargen
* David Teller
* David Tulig 
* DebugSteven 
* Diggory Blake 
* Dmitriy Pleshevskiy 
* Dusty Mabe 
* EclipsedSolari
* Emile Fugulin 
* Emm
* Emmanuel Surleau 
* Erlend Langseth
* Felix Watts 
* Filip Gospodinov 
* Garrett Thornburg 
* Giorgio Gambino 
* Grégory Obanos 
* Hal Gentz
* Han Xu 
* Henk van der Laan 
* Henry Boisdequin
* Hirokazu Hata 
* Iban Eguia (Razican)
* Igor Raits 
* Ivan Tham
* JR Heard 
* Jeremy Stucki 
* Jiří Sejkora 
* Joel Parker Henderson 
* John Brandt 
* Jonas Platte 
* Jonas Schievink
* Joshua Koudys 
* Juhasz Sandor 
* Justice4Joffrey 
* Katharina Fey 
* Kevin King 
* Kevin Kirchner 
* Khionu Sybiern 
* Kitsu 
* Koisell 
* Kononnable 
* Leonardo Yvens
* Lukas Markeffsky
* Maccesch 
* Marc-Stefan Cassola
* Martell Malone
* Martijn Groeneveldt
* Matthew Kuo
* Matthieu Guillemot 
* Mcat12
* Meven 
* Mike Cronce
* Mr Ceperka 
* Nafiul Islam 
* Nathan Papapietro
* Nicholas Yang
* Oliver Cooper
* Otto Castle 
* Pankaj Jangid 
* Paolo Barbolini
* Paul Le Corre 
* Paul Martensen 
* Pavan Kumar Sunkara
* Paweł Przeniczny 
* Philip Trauner
* Raphael Arias 
* Roman 
* Ryan Leckey  
* Sarthak Singh 
* Scott Driggers 
* Sean Klein 
* Simon Ertl 
* Spencer Taylor 
* Steven Chu
* Storm Timmermans
* Sébastien Santoro
* Takayuki Maeda 
* Thomas Constantine Moore 
* Thomas Eizinger
* Thomas Etter 
* Tom MacWright
* Tuetuopay
* Urhengulas 
* Vanio Begic 
* WebeWizard 
* William Myers 
* Yin Jifeng 
* Yuki Okushi 
* Zane Duffield 
* blackghost1987
* czotomo
* dchenk 
* ejc Drobnič 
* gorbit99 
* hasezoey 
* hi-rustin 
* kevinpoitra 
* kpcyrd 
* matthew-dowdell
* ode79 
* ropottnik
* theredfish
* zoewithabang 
* Émile Fugulin
* κeen
* 二手掉包工程师 
* 棒棒彬_Binboy 
