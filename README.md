# outsider
If you have multiple projects depend use [gitlab-art](https://github.com/kosma/gitlab-art),
handle updating them locally instead of pulling the files from the CI.

## Example
### project A `artifacts.yml`

The following projects depends on project: `kosma/foobar-firmware`, and we have changes
locally that we want to test before pushing to the gitlab repo. We _could_ just `cp` them,
but `outsider` removes that complexity and uses the `artifacts.yml` file.

```yml
- project: kosma/foobar-firmware
  ref: 1.4.0
  job: firmware-8051
  install:
    build/8051/release/firmware.bin: blobs/firmware-8051.blob
```

### project B `kosma/foobar-firmware`
We have now updated the firmware, good thing we can easily push these changes to the other repo :)
```
# build build/8051/release/firmware.bin
(kosma/foobar-firmware) $ make

# move artifacts to the correct placement in project-a
(kosma/foobar-firmware) $ outsider ./project-a/artifacts.yml --project kosma/foobar-firmware --source-dir .
```
