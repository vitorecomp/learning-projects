package data

import (
	"encoding/json"
	"errors"
	"io"
)

func UnmarshalProduct(data []byte) (Product, error) {
	var r Product
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *Product) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func (p *Products) ToJson(w io.Writer) error {
	e := json.NewEncoder(w)
	return e.Encode(p)
}

func (p *Product) FromJson(r io.Reader) error {
	e := json.NewDecoder(r)
	return e.Decode(p)
}

type Product struct {
	ID   int    `json:"id"`
	Name string `json:"name"`
}

type Products []*Product

func GetProducts() Products {
	return productList
}

func AddProduct(prod *Product) {
	productList = append(productList, prod)
}

func UpdateProduct(id int, prod *Product) error {
	pos, err := findProduct(id)
	if err != nil {
		return err
	}
	prod.ID = id
	productList[pos] = prod
	return nil
}

func DeleteProduct(id int) error {
	pos, err := findProduct(id)
	if err != nil {
		return err
	}
	// Remove the element at index i from a.
	productList[pos] = productList[len(productList)-1]
	productList[len(productList)-1] = nil
	productList = productList[:len(productList)-1]
	return nil
}

var ErrProductNotFound = errors.New("product not found")

func findProduct(id int) (int, error) {
	for i, p := range productList {
		if p.ID == id {
			return i, nil
		}
	}
	return 0, ErrProductNotFound
}

var productList = []*Product{}
