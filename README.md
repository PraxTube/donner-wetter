# Wetter CLI

This repo will contain a program written in rust to display weather info.

## TODO

The following steps will outline the rough development process of this application.

- [x] Firstly, make some very basic API calls to `https://github.com/Pirate-Weather/pirateweather` and print the data to terminal
- [ ] Now make a bit more specific calls about Berlin and get the next 3 days forcast for temperature and percipitation (this means rain)
- [ ] Take the temperature data from the response and plot them in the terminal, I would suggest to try out `https://github.com/loony-bean/textplots-rs` at first, although you might want to implement the whole UI using `https://docs.rs/ratatui/latest/ratatui/widgets/struct.Chart.html`. I personally think for this project both approaches would work, it's down to personal preference (ratatui is much more versatile but might be a bit overkill, I would try both and see which one works better. You can also take a completely different one if you find something better suited)
- [ ] After that plot the percipitation forcast for the next 3 days as well (below the first plot)
- [ ] Finally also plot the wind forcast. This is a bit tricky as I would also like to know the wind direction, perhaps use color plots to visual the direction (map different colors to different direction)? Though I don't know how easy that is with the given plotting libraries. If this is too hard then just plot the wind strength without the direction
