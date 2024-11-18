# tiffy
A poorly written, not customizable tiff viewer. The main goal was to quickly view DEM rasters without needing to open QGIS. Plans for the future may include adding a few color ramps, and band selection (1 band viewing only for now)

## usage
```tiffy <Options> <INPUT_PATH>```

### Options
```-c, --color <COLOR> [default: elevation] [possible: elevation, ryg]```
<br><br>
```-s --stretch <STRETCH_TYPE> [default: min-max] [possible: min-max, iqr, sd, mad]```
<br><br>
```-q --quant <QUANTITATIVENESS> [default: continuous] [possible: continuous, discrete]```

## special thanks
Hopefully in the future, I can simply include the geotiff crate, but it is under construction by the wonderful georust team. Thank you to them, whose old drivers were used to make this. 

