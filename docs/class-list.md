# List of Classes
* Parameter
  * class for different command line flags
  * specifies name and functions for parsing command line arguments
  * may be best implemented as a trait? specific commands can implement the trait
* DataManagerCLI
  * primary controller for the CLI
  * manages the logger
  * workflow: determine the sub-command being invoked, parse subsequent arguments, and call the appropriate function with those arguments
  * uses argparse to parse command line arguments and match arguments to appropriate functions
* DataManager
  * primary controller for the internal data manager application
* StagedFile
  * class representation for a staged file
  * contains origin, target and hash
* Manifest
  * main data structure is a Dict(str, Dict(str, Union(str, int)))
  * (relative path, (checksum, size))
  * need to check list of existing files against manifest, fast lookup is required
* Mission
* Dataset
* Metadata

# Questions for Redesign
* Test data vs. field data - do we want to have different configurations for those? 
  * Test data: most likely single-day, multiple devices possible, multiple trials for a signle device possible
  * Field data: likely multiple days, with multiple deployments across multiple devices in a single day
  * potential solution: tool can distinguish between tests, missions and field deployments (init_test, init_mission, init_deployment). These are ordered from least amount of required detail to most
* does file metadata have attached timezones? 
* should the configuration be stored in multiple files in a single folder instead of just one big file? (example: git)
