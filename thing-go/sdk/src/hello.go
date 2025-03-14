package main

import (
	"context"
	"fmt"

	"os"

	extism "github.com/extism/go-sdk"
	"github.com/tetratelabs/wazero"

	_ "embed"
)

//go:embed module.wasm
var wasmModule []byte

func main() {
	fmt.Println("Initing plugin...")

	manifest := extism.Manifest{
		Config: map[string]string{
			"api_key": "supersecret",
			"config":  "{ \"a\": 1, \"b\": 3 }",
		},
		Wasm: []extism.Wasm{
			extism.WasmData{Data: wasmModule},
		},
	}

	ctx := context.Background()
	moduleConfig := wazero.NewModuleConfig().
		WithStdout(os.Stdout).
		WithName("lib-thing").
		WithStderr(os.Stdout)
	config := extism.PluginConfig{
		EnableWasi:   true,
		ModuleConfig: moduleConfig,
	}
	plugin, err := extism.NewPlugin(ctx, manifest, config, []extism.HostFunction{})
	fmt.Println("Calling plugin...")

	fmt.Fprintln(os.Stdout, "yoyo")

	if err != nil {
		fmt.Printf("Failed to initialize plugin: %v\n", err)
		os.Exit(1)
	}

	data := []byte("{\"x\":10}")
	exit, out, err := plugin.Call("calc_series", data)
	response := string(out)
	fmt.Println(response)
	if err != nil {
		fmt.Printf("Err: %v\n", err)
		os.Exit(int(exit))
	}
}
