package handlers

import (
	"errors"
	"log"
	"net/http"
	"regexp"
	"strconv"

	"github.com/vitorecomp/learning.projects.simple.rest/data"
)

type Products struct {
	l *log.Logger
}

func NewProduct(l *log.Logger) *Products {
	return &Products{l}
}

var ErrorMoreThanOneProduct = errors.New("more than one id on url")

func getIdFromURL(r *http.Request) (int, error) {
	reg := regexp.MustCompile(`([0-9]+)`)
	g := reg.FindAllStringSubmatch(r.URL.Path, 3)
	if len(g) != 1 {
		return 0, ErrorMoreThanOneProduct
	}

	return strconv.Atoi(g[0][1])
}

func (p *Products) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	if r.Method == http.MethodGet {
		p.getProducts(w, r)
		return
	}

	if r.Method == http.MethodPost {
		p.addProduct(w, r)
		return
	}

	if r.Method == http.MethodPut {
		//expect the id in the url
		id, err := getIdFromURL(r)
		if err != nil {
			http.Error(w, err.Error(), http.StatusNotFound)
			return
		}
		p.l.Println("Working ion id:", id)
		p.updateProduct(id, w, r)
		return
	}

	if r.Method == http.MethodDelete {
		//expect the id in the url
		id, err := getIdFromURL(r)
		if err != nil {
			http.Error(w, err.Error(), http.StatusNotFound)
			return
		}
		p.l.Println("Working ion id:", id)

		p.deleteProduct(id, w, r)
		return
	}

	w.WriteHeader(http.StatusNotImplemented)
}

func (p *Products) getProducts(w http.ResponseWriter, r *http.Request) {
	p.l.Println("Handle GET product")
	lp := data.GetProducts()
	err := lp.ToJson(w)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}

func (p *Products) addProduct(w http.ResponseWriter, r *http.Request) {
	p.l.Println("Handle POST product")
	prod := &data.Product{}
	err := prod.FromJson(r.Body)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
	}
	data.AddProduct(prod)
}

func (p *Products) updateProduct(id int, w http.ResponseWriter, r *http.Request) {
	p.l.Println("Handle PUT product")
	prod := &data.Product{}
	err := prod.FromJson(r.Body)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
	}

	err = data.UpdateProduct(id, prod)
	if err != nil {
		http.Error(w, "Product not found", http.StatusNotFound)
	}
}

func (p *Products) deleteProduct(id int, w http.ResponseWriter, r *http.Request) {
	p.l.Println("Handle DELETE product")
	err := data.DeleteProduct(id)
	if err != nil {
		http.Error(w, "Product not found", http.StatusNotFound)
	}
}
