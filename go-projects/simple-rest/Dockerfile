FROM golang:1.16.2-buster

ENV TZ=America/Sao_Paulo

WORKDIR /app
COPY go.mod go.sum ./
RUN go mod download
COPY . .

## Our project will now successfully build with the necessary go libraries included.
RUN go build -o main .


CMD ["/app/main"]