FROM alpine:latest

RUN apk add --no-cache oath-toolkit-oathtool

ENTRYPOINT ["oathtool", "--totp"]
