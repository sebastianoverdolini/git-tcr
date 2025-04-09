package main

import (
	"os/exec"
)

func ls() (string, error) {
	cmd := exec.Command("ls", "-l")

	output, err := cmd.CombinedOutput()

	if err != nil {
		return "", err
	}

	return string(output), nil
}
