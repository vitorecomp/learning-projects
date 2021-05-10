package recipe

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"os"
	"github.com/getlantern/deepcopy"
)


type Recipe struct {
	Result string `json:"result"`
	Name string `json:"name"`
	Amount float64 `json:"amount"`
	Recipes []Recipe `json:"resources"`
}

const fileName = "./resources/recipes.json"

func readRecipeFile() []Recipe {
	// Open our jsonFile
	jsonFile, err := os.Open(fileName)
	// if we os.Open returns an error then handle it
	if err != nil {
		fmt.Println(err)
	}
	fmt.Println("Successfully Opened " + fileName)
	// defer the closing of our jsonFile
	defer jsonFile.Close()

	//read file and open json
	byteValue, errRead := ioutil.ReadAll(jsonFile)
	if errRead != nil {
		log.Fatal(errRead)
		os.Exit(1)
	}

	recipes := []Recipe{}
	parseError := json.Unmarshal(byteValue, &recipes)
	if parseError != nil {
		log.Fatal(parseError)
		os.Exit(1)
	}

	return recipes
}


func GetRecipe() []Recipe{
	recipes := readRecipeFile()
	return recipes
}

func Prepare(desired string, recipes []Recipe, resources []Recipe) (float64, Recipe){
	max := 0.0
	maxRecipe := Recipe{}

	for _, recipe := range(recipes) {
		if(recipe.Result == desired){
			interMax, interRecipe := prepareNestedRecipe(recipe, recipes, resources)
			if(interMax > max){
				max = interMax
				maxRecipe = interRecipe
			}
		}
	}

	return max, maxRecipe
}

func prepareNestedRecipe(actualRecipe Recipe, recipes []Recipe, resources []Recipe) (float64, Recipe){
	missingItens := getDifferences(actualRecipe.Recipes, resources)
	if(len(missingItens) == 0){
		return evaluateRecipe(actualRecipe, resources), actualRecipe
	}else{
		//get the all the recipes the will result in the missing item
		missingRecipes := []Recipe{}
		for _, recipe := range(recipes){
			if(recipe.Result == missingItens[0].Result){
				missingRecipes = append(missingRecipes, recipe)
			}
		}
		
		max := 0.0
		maxRecipe := Recipe{}

		for _, missingRecipe := range(missingRecipes) {
			//overwrite the missing item for the necessary recipe			
			interRecipe := overwriteResources(actualRecipe, missingRecipe)
			fmt.Println("New recipe")
			fmt.Println(interRecipe)
			
			//call prepare
			interMax, interRecipe := prepareNestedRecipe(interRecipe, recipes, resources)
			if(interMax > max){
				max = interMax
				maxRecipe = interRecipe
			}

		}
		
		return max, maxRecipe
	}
}

func overwriteResources(recipe Recipe, missingRecipe Recipe) Recipe{
	fmt.Println("Start the resource overwrite")
	interMissingRecipe := Recipe{}
	deepcopy.Copy(&interMissingRecipe, &missingRecipe)
	fmt.Println("Item to overwrite")
	fmt.Println(interMissingRecipe)
	
	interRecipe := Recipe{}
	deepcopy.Copy(&interRecipe, &recipe)
	fmt.Println("Recipe to be overwrite")
	fmt.Println(interRecipe)
	
	for i := range(interMissingRecipe.Recipes){
		interMissingRecipe.Recipes[i].Amount = interMissingRecipe.Recipes[i].Amount/interMissingRecipe.Amount
	}
	fmt.Println("For to overwrite the resource in every body")
	for i, recipeResource := range(interRecipe.Recipes){
		if(recipeResource.Result == interMissingRecipe.Result){
			//remove o item necessario
			interRecipe.Recipes = append(interRecipe.Recipes[:i], interRecipe.Recipes[i+1:]...)
			interRecipe.Recipes = append(interRecipe.Recipes, interMissingRecipe.Recipes...)
			fmt.Println("Overwirte result")
			fmt.Println(interRecipe)
		}
	}
	fmt.Println("Final result of overwrite")
	fmt.Println(interRecipe)
	return interRecipe
}


func getDifferences(desireds []Recipe, resources []Recipe) []Recipe {
	missingItens := []Recipe{}
	missedItem:
	for _, desired := range(desireds){
		for _, resource := range(resources) {
			if(desired.Result == resource.Result){
				continue missedItem
			}
		}
		missingItens = append(missingItens, desired)
	}
	return missingItens
}

func evaluateRecipe(actualRecipe Recipe, resources []Recipe) float64{
	fmt.Println("Fullfill the recipe: ", actualRecipe.Name)
	fmt.Println("Recipe: ", actualRecipe)
	min := 1000000.0
	for _, neededResource := range(actualRecipe.Recipes){
		available := 0.0
		for _, resource := range(resources) {
			if(neededResource.Result == resource.Result){
				available = resource.Amount
			}
		}
		if(min > available/neededResource.Amount){
			min = available/neededResource.Amount
		}
	}
	fmt.Println("Total result: ", min*actualRecipe.Amount)
	return min*actualRecipe.Amount
}