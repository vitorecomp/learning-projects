// Package classification of Product API
//
// Documentation for Product API
//
// Schemes: http
// BasePath: /products
// Version: 1.0.0
//
// Consumes:
// - application/json
//
// Produces:
// - application/json
// swagger:meta
package handlers

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"strconv"

	"github.com/gorilla/mux"
	"github.com/vitorecomp/learning.projects.gorilla.rest/data"
)

type Products struct {
	l *log.Logger
}

func NewProduct(l *log.Logger) *Products {
	return &Products{l}
}

// A list of products returned in a the response
// swagger:response productsResponse
type productsResponse struct {
	// All products in the system
	// in: body
	Body []data.Product
}

// swagger:route GET /products products listProducts
// Return a list of products
// responses:
// 200: productsResponse

// GetProducts returns the list of products from the data storage
func (p *Products) GetProducts(w http.ResponseWriter, r *http.Request) {
	p.l.Println("Handle GET product")
	lp := data.GetProducts()
	err := lp.ToJson(w)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}

func (p *Products) AddProduct(w http.ResponseWriter, r *http.Request) {
	p.l.Println("Handle POST product")
	prod := r.Context().Value(KeyProduct{}).(data.Product)
	data.AddProduct(&prod)
}

func (p *Products) UpdateProduct(w http.ResponseWriter, r *http.Request) {
	p.l.Println("Handle PUT product")
	vars := mux.Vars(r)
	id, _ := strconv.Atoi(vars["id"])
	prod := r.Context().Value(KeyProduct{}).(data.Product)

	err := data.UpdateProduct(id, &prod)
	if err != nil {
		http.Error(w, "Product not found", http.StatusNotFound)
	}
}

func (p *Products) DeleteProduct(w http.ResponseWriter, r *http.Request) {
	p.l.Println("Handle DELETE product")
	vars := mux.Vars(r)
	id, _ := strconv.Atoi(vars["id"])

	err := data.DeleteProduct(id)
	if err != nil {
		http.Error(w, "Product not found", http.StatusNotFound)
	}
}

type KeyProduct struct{}

func (p Products) MiddlewareValidateProduct(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		prod := data.Product{}
		err := prod.FromJson(r.Body)
		if err != nil {
			p.l.Println("Error on product deserialization", err)
			http.Error(w, err.Error(), http.StatusBadRequest)
			return
		}
		err = prod.Validate()
		if err != nil {
			p.l.Println("Error on product validation", err)
			http.Error(w,
				fmt.Sprintf("Error on product validation %s", err),
				http.StatusBadRequest)
			return
		}

		ctx := context.WithValue(r.Context(), KeyProduct{}, prod)
		req := r.WithContext(ctx)
		next.ServeHTTP(w, req)
	})
}
