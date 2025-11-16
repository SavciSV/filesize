# filesize

Filesize is a command-line utility that helps you get the file size of a file from its direct link.

For example you wanna download something but you're not sure about its size, you cannot determine its size without trying to download it and see the size in your download utility.

With this tool, you can just give it the link of the file, and it'll tell you its size

**Good Tip**: If the app gave you a very small size than the expected one, it's likely that the link you gave it is not a direct link to the file itself.

___

## Usage

```sh
filesize <URL>
```

## Example

Getting the current version of debain 13 net-installation ISO size:

```sh
filesize 'https://saimei.ftp.acc.umu.se/debian-cd/current/amd64/iso-cd/debian-13.2.0-amd64-netinst.iso'
784MiB
```
