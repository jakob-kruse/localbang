# localbang

Cross-platform, cross-browser, cross-search-engine duckduckgo-like bangs

# What are "bangs"??

Bangs are a way to define where to search inside the query itself. For example you could have a bang with the keyword "!gh" and the url "https://github.com/search?q=%s".
Now, whenever your search query contains "!gh" the rest of the query will replace the "%s" in the url. 

Example:

"!gh localbang" will result in "https://github.com/search?q=localbang"

## Installation

### Package Manger

#### AUR

    yay -S localbang

### Binary release

Download the latest binary release from [here](https://github.com/jakob-kruse/localbang/releases) and place it in a folder thats in $PATH e.g. /usr/local/bin.

### Cargo intall

This is easier, but you have to build the binary yourself.

    cargo install localbang


### DIY

    git clone https://github.com/jakob-kruse/localbang
    cd localbang
    cargo build --release

The `localbang` binary will be place inside `target/release`;
## Usage

    localbang <shortcuts_file> <search_engine>

This will start a local webserver on port 8000. Every path (e.g. http://localhost:8000/search?q=foo) will be redirected to the search engine (with the query "foo") provided as a second argument (defaults to google search).

### Sysyemd

You can find a unit file in [localbang.service](example/localbang.service) that can be used to start localbang in systemd. Place this in `/etc/systemd/user` or `~/.config/systemd/user/` and then `systemctl --user enable --now localbang.service` to enable it.

### Shortcuts file

The shortcuts file defined the bangs, which are available. It is a CSV file with two rows: the `keyword` and the `url`. [Examle Shortcuts File](example/shortcuts.csv)

#### keyword

This is the word or letter (could be anything really) that will be used to trigger the search engine specified in the `url` column.

#### url

The search engine to use for the bang/keyword. This must include a `%s` as the placeholder for the query.

### Browser integration

Note: Opensearch Integration is planned for the future.

#### Chrome

In Chrome you can directly add a custom search engine.

- Open the settings
- Select "Search engine" on the left
- Click "Manage search engines"
- Click "Add"
- Enter the name (e.g. "localbang")
- Enter any keyword. This only matters, if you are not going to use localbang as the default engine.
- Enter the url http://localhost:8000/search?q=%s
- Localbang should appear in the list under "Other search engines"
- (optional) Click the three dot menu and click "Make default", otherwise use the provided keyword.

#### Firefox

The easiest way is to use the [Add custom search engine](https://addons.mozilla.org/en-US/firefox/addon/add-custom-search-engine/) extension.

- Click the extension icon
- Enter any name (e.g. localbang)
- Enter the URL http://localhost:8000/search?q=%s
- Click "Add custom search engine"
- Follow the on screen steps (right-clicking the search bar and selecting it in the settings)