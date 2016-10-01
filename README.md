# bandcamp scraper

this is a tiny proof-of-concept to see whether/how i could scrape bandcamp album pages for metadata
and (eventually) stream links. i'd like to make a bandcamp client eventually and this is me
tinkering with pieces that move me in that direction.

## running it

```sh
git clone https://github.com/QuietMisdreavus/bandcamp-load
cd bandcamp-load
cargo run -- $ALBUM_PAGE
```

...with `$ALBUM_PAGE` being any URL that links to a bandcamp album, like [this
one][] or [that one][].

[this one]: https://trackedmusic.bandcamp.com/album/chip-zone-003
[that one]: https://andrewhuang.bandcamp.com/album/pintxos

it will print some information about the album to the console.
