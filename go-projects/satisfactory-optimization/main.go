package main
import "fmt"
import "github.com/vitorecomp/satisfactory-optimization/src/recipe"

const desired string = "iron-plate"
var resources []recipe.Recipe = []recipe.Recipe{
	recipe.Recipe {
		Result: "iron-ore", 
		Amount: 100,
	},
	recipe.Recipe {
		Result: "copper-ore", 
		Amount: 150,
	},
};

func main(){
	//parse the recipes
	recipes := recipe.GetRecipe()
	max, recipe := recipe.Prepare(desired, recipes, resources)
	fmt.Println("Resultado final")
	fmt.Println(max)
	fmt.Println(recipe)
}