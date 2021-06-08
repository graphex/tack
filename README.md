# tack
Local UDP data to InfluxDB + other utility stuff for Weatherflow Tempest weather stations

This is a utility written in Rust to parse the UDP broadcast from Tempest weather stations into InfluxDB LPR format and send it to an InfluxDB database for storage and visualization.

The intent is mostly to have a way to stream the data out to InfluxDB, but I'm also using it to exercise a local installation of Influx IOx and foster potential new API development (gRPC, etc.).

This project uses Actix Actors to allow the different parts to execute concurrently and so that it is able to be easily adapted for the other uses I have in mind, like displaying temperature and wind data on my nixie tube clock.
