# SDD notes

## Packages needed (Arch)
- pandoc-cli
- wkhtmltopdf

## Template
[default.html4](./templates/default.html4) is unused.

### Font
Possible addition to [styles.html](./templates/styles.html) if the font doesn't work. My understanding is that the `src` are checked in order and it uses the first one that succeeds.

```
@font-face{
  font-family: "Noto Serif";
  src: local("Noto Serif") format("tff");
  src: url(file:///usr/share/fonts/noto/NotoSerif-Regular.tff) format("tff");
  src: url(https://github.com/google/fonts/blob/main/ofl/notoserif/NotoSerif-Regular.ttf) format("tff");  
}
```

### Pandoc HTML Templates
Pandoc changed the default HTML template in version 2.11, resulting in different output on different computers. I copied the relevant files into [pandoc-default-files](./pandoc-default-files) for reference.
