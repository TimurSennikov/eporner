
# eporner

A rule 34 wrapper for downloading posts by tags written in rust.
## Usage/Examples

### Download all images with tag 'futurama' excluding the ones with 'ai_generated' tag in the out directory
```shell
eporner tags futurama exclude ai_generated output out
```

### Download all images with tag 'futurama' excluding the ones with 'ai_generated' and 'philip_j_fry' tags in the out directory
```shell
eporner tags futurama exclude ai_generated,philip_j_fry output out
```

### Download all images with tags 'futurama' and 'rick_and_morty' excluding the ones with 'ai_generated', 'jerry' and 'philip_j_fry' tags in the out directory
```shell
eporner tags futurama,rick_and_morty exclude ai_generated,jerry,philip_j_fry output out
```

## Authors

- [@TimurSennikov](https://www.github.com/TimurSennikov)

## License

[gnu-pl](https://choosealicense.com/licenses/gpl-3.0/)

