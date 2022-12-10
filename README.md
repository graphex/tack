Tack
====
# tack
Local UDP data to InfluxDB + other utility stuff for Weatherflow Tempest weather stations

This is mostly a Rust language learning exercise, one of the first ones I started with in 2020.

This is a utility written in Rust to parse the UDP broadcast from Tempest weather stations into InfluxDB LPR format and send it to an InfluxDB database for storage and visualization.
with the TLS being terminated in the ingress.

The intent is mostly to have a way to stream the data out to InfluxDB, but I'm also using it to exercise a local installation of Influx IOx and foster potential new API development (gRPC, etc.).
```shell
curl --request POST \
"https://influxdb.gfx.tc/api/v2/write?org=$INFLUX_ORG_ID&bucket=tempest&precision=s" \
  --header "Authorization: Token $INFLUX_TOKEN" \
  --header "Content-Type: text/plain; charset=utf-8" \
  --header "Accept: application/json" \
  --data-binary '
    ObsSt,serial_number=ST-00029213,hub_sn=HB-00022471,firmware_revision=156 wind_lull=1.21,wind_avg=1.66,wind_gust=2.02,wind_direction=350.00,wind_sample_interval=3,air_temperature=4.25,relative_humidity=63.12,illuminance=0,uv=0.00,solar_radiation=0,precip_acc=0.00,precip_type=0,strike_avg_distance=0,strike_count=0,battery=2.68,report_interval=1,station_pressure=832.01 1670207840
    RapidWind,serial_number=ST-00029213,hub_sn=HB-00022471 wind_speed=1.72,wind_direction=28 1670209024
    HubStatus,hub_sn=HB-00022471,firmware_revision=171 uptime=16359638,rssi=-34,seq=1634688 1670209304
    '
```
This project uses Actix Actors to allow the different parts to execute concurrently and so that it is able to be easily adapted for the other uses I have in mind, like displaying temperature and wind data on my nixie tube clock.
