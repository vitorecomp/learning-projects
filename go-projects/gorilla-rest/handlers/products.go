package handlers

import (
	"context"
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
			http.Error(w, err.Error(), http.StatusBadRequest)
			return
		}
		ctx := context.WithValue(r.Context(), KeyProduct{}, prod)
		req := r.WithContext(ctx)
		next.ServeHTTP(w, req)
	})
}
