# media-dj

media-dj comes from dissatisfaction with Tdarr's closed source nature, and horrifying UI. I am hoping this will become a viable (completely open-source) alternative written in Rust, with many additional extra features. Note, this project is also serving as a way to expand my knowledge of the Rust language.

## Development

### Backend

1. Make sure you have a [Rust installation](https://www.rust-lang.org/tools/install)

#### for WSL (Ubuntu 22.04), macOS (14.5), *nix

- Grab it: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Select default installation: `1` or `return`
- *Restart current shell*
- Test installation with: `rustc --version`
- *My environment is*

```shell
rustc 1.80.1 (3f5fd8dd4 2024-08-06)
```

- Start backend server with:
- ``

### Frontend

> [!NOTE]
> I am using [Bun](https://bun.sh/docs/installation) as my preferred Javascript toolkit and in the following documentation.

- Install front-end dependencies

   `cd media-dj-fe && bun install`

### Database

- Install development database (using [Homebrew](https://brew.sh/) for `macos`)

- `brew tap mongodb/brew`

- `brew install mongodb-community`

- `brew services start mongodb/brew/mongodb-community`

## Feature Implementations

- [ ] Core API: Library Parsing and Statistics
  - [ ] Umbrella stats - total media files per type, percentages of total volvume space, etc.
  - [ ] File stats - file size, codec information, container type, bitrate, media track information, etc.
  - [ ] Scanning system (manual scanning, inode watchdog)
    - [ ] How to handle unsafe scans (file gets deleted while scan is happening or something - lots of potential moving parts with applications like Sonarr and Transmission)
- [ ] Global Error Handling
  - [ ] Research Rust libs for proper patterns in this regard
- [ ] Proper patterns for media file permissions (non-root user implementation)
- [ ] Core API: FFMpeg transcoding (similar to Tdarr)
  - [ ] Rulesets/thresholds for transcoding
  - [ ] I like Tdarr's idea for having "flows" of tasks; maybe to get rid of certain subtitles, audio tracks, etc.
- [ ] Front-end: "windirstat"-style view of libraries/media
- [ ] Front-end: Library displays, tables, etc. - what do we want it to look like?
- [ ] Database: NoSQL vs SQL
  - [ ] NoSQL seems appealing as it is highly adaptable to the countless different media types that we would be dealing with
  - [ ] Document-strucutre could support nested information about tracks and codecs and that kind of thing.
- [ ] Database: Schema design
  - [ ] Mediafiles are uniquely identified by inode+path hash (?)
    - Directory hierarchy implies relationship, ie:
      - library root (ex. `/Movies`), all files in `/Movies` are _movies_
      - All dirs/files in `/Movies/Some Movie` are related to `Some Movie`
      - same idea should be applied to other library types, such as photos or audiobooks
  - [ ] Tracking changes to unique file overtime
    - Example scenarios to consider:
    1. Initial library scan completes. You go in and change the filename only of some Movie. Inode should stay the same, the only thing that changes is the path. The movie object in the DB should be _updated_, not removed and recreated.
    2. When does an entry cease to exist? Should not be when the fields' contents change
