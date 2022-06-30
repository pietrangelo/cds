# CDS - Content Delivery Server

## Scope

The scope of this service is to add a new infrastructure layer to serve static resources to Entando's components (de-app, appbuilder),
in particular when we need to scale-out the de-app or in multitenancy configurations.

The CDS service exposes two ports:

- Internal Port: 8080, everything under `/api/v1` path and needs authorization (bearertoken)
- Public Port: 8081, which needs to be exposed by an Ingress object


## Documentation

To see the documentation:
1. Clone the project
2. Run `cargo doc --open`

## Notes

The order of the parameters is very important for the `api/v1/upload/` api:
1. path
2. protected
3. filename
4. file

```bash
curl --location --request POST 'https://cds.domain.com/api/v1/upload/' \
--header 'Authorization: Bearer eyJhbG...K3unA' \
--form 'path="test"' \
--form 'protected="true"' \
--form 'filename="file.png"' \
--form 'file=@"/home/user/Images/Screenshot_20220526_143107.png"'
```


