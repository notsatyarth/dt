### Introduction:
A rust learning project to scratch an itch.  It can transform a "YYYY-MM-DD HH:MM:SS" between timezones

### Options:
```bash
dt 0.1.0
Transform dates to different timezones using timezone abbreviations

USAGE:
    dt [OPTIONS]

OPTIONS:
    -d, --date <DATE>    The time string to parse [default: now]
    -f, --from <FROM>    The timezone abbreviation  to convert from [default: local]
    -h, --help           Print help information
    -t, --to <TO>        The timezone abbreviation  plto convert to [default: local]
    -V, --version        Print version information
```

### Next Up
- [ ] Make it more CLI like
- [ ] Parse  naive date and naive time 
- [ ] Add some parsing tests
- [ ] Set some env variable based defaults