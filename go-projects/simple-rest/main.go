package main

import (
	"context"
	"log"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/vitorecomp/learning.projects.simple.rest/handlers"
)

func main() {

	l := log.New(os.Stdout, "product-api => ", log.LstdFlags)
	productHandler := handlers.NewProduct(l)

	serverMux := http.NewServeMux()
	serverMux.Handle("/", productHandler)

	server := &http.Server{
		Addr:         ":9090",
		Handler:      serverMux,
		IdleTimeout:  120 * time.Second,
		ReadTimeout:  10 * time.Second,
		WriteTimeout: 10 * time.Second,
	}

	go func() {
		err := server.ListenAndServe()
		if err != nil {
			l.Fatal(err)
		}
	}()

	sigChan := make(chan os.Signal, 10)
	signal.Notify(sigChan, os.Interrupt)
	signal.Notify(sigChan, syscall.SIGTERM)

	sig := <-sigChan
	l.Println("Received a signal to shutdown", sig)

	timeToShutdown, _ := context.WithTimeout(context.Background(), 30*time.Second)
	server.Shutdown(timeToShutdown)
}
