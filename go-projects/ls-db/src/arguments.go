package src

struct Flags {}

func ProcessFlags () {
	flags = get
	flag.Parse()
	if len(flag.Args()) > 0 {
		fmt.Printf("Passou o agumento do de path sem flag")
	} else {
		fmt.Printf("Procuta pelo argumento pela flag\n")
		path := flag.String("path", "./", "path for ls")
		fmt.Printf(*path)
	}	
}