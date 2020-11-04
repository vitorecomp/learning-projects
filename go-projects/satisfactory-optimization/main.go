package main
import "fmt"
import "github.com/vitorecomp/satisfactory-optimization/src/recipe"

func main(){
	fmt.Print("Aqui\n")
	var ironOre, copperOre int = 100, 150;
	var desired string = "iron-ingot"
	
	//parse the recipes
	recipe.GetRecipe(desired)
	
	//interate
	//define the recipe path
	//calculate the max output
	//if bigger print
	fmt.Println(ironOre)
	fmt.Println(copperOre)
}