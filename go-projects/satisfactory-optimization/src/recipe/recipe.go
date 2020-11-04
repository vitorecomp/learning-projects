package recipe

import "fmt"

type Recipe struct {
	
}

func GetRecipe(element string) Recipe{
	fmt.Println(element)
	recipe := Recipe{}
	return recipe
}