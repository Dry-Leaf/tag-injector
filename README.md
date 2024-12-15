A utility to add tags from a booru to their most common file formats(JPEG, PNG, GIF). 
This allows locally stored images to be searched for based on content.

Uses XMP Dublin Core tags, which Windows is natively capable of reading from JPEG files.
With the installation of [this dll](https://gitgud.io/nvtelen/xmp_property_extension), 
reading support is added to PNG, GIF and JXL files. 

## Usage
Make sure config.toml is in the same folder as the executable.

Run the executable in a terminal and pass a file or folder path to it. 
Pass --help instead to see further options.

Add your accouts' API access credentials to the configuration file's api url values if desired.

Additional boorus can be added provided the correct values are added to the configuration file.
