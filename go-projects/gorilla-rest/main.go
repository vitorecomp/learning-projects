package main

import (
	"context"
	"log"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/gorilla/mux"

	"github.com/vitorecomp/learning.projects.gorilla.rest/handlers"
)

func main() {

	l := log.New(os.Stdout, "product-api => ", log.LstdFlags)
	productHandler := handlers.NewProduct(l)

	serverMux := mux.NewRouter()

	getRouter := serverMux.Methods(http.MethodGet).Subrouter()
	getRouter.HandleFunc("/products", productHandler.GetProducts)

	putRouter := serverMux.Methods(http.MethodPut).Subrouter()
	putRouter.HandleFunc("/products/{id:[0-9]+}", productHandler.UpdateProduct)
	putRouter.Use(productHandler.MiddlewareValidateProduct)

	postRouter := serverMux.Methods(http.MethodPost).Subrouter()
	postRouter.HandleFunc("/products", productHandler.AddProduct)
	postRouter.Use(productHandler.MiddlewareValidateProduct)

	deleteRouter := serverMux.Methods(http.MethodDelete).Subrouter()
	deleteRouter.HandleFunc("/products/{id:[0-9]+}", productHandler.DeleteProduct)

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
