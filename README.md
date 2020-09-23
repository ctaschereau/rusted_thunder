# rusted_thunder
Desktop app (eventually also for PinePhone) to interact with the Tesla API. This could eventually be used as a replacement for the official Tesla Android or iOS app.

## Config
A config file is required for this app to function. At the moment, the file must be named *.teslac* and must reside in the user's home folder.
Also, it must be a valid .toml file and contain the following properties :
```
[global]
api_token = "____SOME_API_TOKEN_OBTAINED_FROM_A_LOGIN_ENDPOINT____"
default_vehicle = "CAR_NAME_GOES_HERE"
``` 
At the moment, the api_token needs to be generated from outside of this app.

##TODO list:
- Wire up current action buttons
- Implement "Driving 40km/h" feature
- Finish general layout of app
    - Add all missing buttons and sections
    - Find open source image for car
    - Find better way to show open doors/windows
    - Fix CSS that does not work for now
- Implement refresh button/action and/or auto refresh
- Implement login and saving the resulting API token
- Use proper Rust error handling instead of ugly unwraps everywhere
- Allow multiple configuration path/file options
- Implement preferred unit (metric or imperial)
