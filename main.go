package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type binary struct {
	left, right interface{}
	op          string
}

func (b *binary) String() string {
	if b == nil {
		return ""
	}
	return fmt.Sprintf("{%v %v %v}", b.left, b.op, b.right)
}

func Calculate(input string) int {
	tokenMap, charIndex := map[string][]string{}, -1

	chars := toChars(input)
	tokenizedChars := tokenizeBrackets(&charIndex, chars, tokenMap, tokenGenerator())

	groups := createBinaryGroups(tokenizedChars, tokenMap)
	result := buildComputeFrom(
		computeBinary("*"),
		computeBinary("/"),
		computeBinary("+"),
		computeBinary("-"),
	)(groups)

	return result
}

func buildComputeFrom(compute ...func([]*binary) []*binary) func([]*binary) int {
	return func(ops []*binary) int {
		acc := compute[0](ops)
		for _, c := range compute[1:] {
			acc = c(acc)
		}
		// we should be left with only 1 group
		return evalBinary(acc[0])
	}
}

func tokenizeBrackets(charIndex *int, chars []string, tokens map[string][]string, nextToken func() string) []string {
	var charsWithTokens []string
	i := 0
	for i < len(chars) {
		*charIndex++
		c := chars[i]
		if c == ")" {
			return charsWithTokens
		}
		if c == "(" {
			t := nextToken()
			tokens[t] = tokenizeBrackets(charIndex, chars[i+1:], tokens, nextToken)
			charsWithTokens = append(charsWithTokens, t)
			i = *charIndex + 1
		} else {
			charsWithTokens = append(charsWithTokens, c)
			i++
		}
	}
	return charsWithTokens
}

func createBinaryGroups(chars []string, tokenMap map[string][]string) []*binary {
	var ops []*binary
	for i, c := range chars {
		if !strings.ContainsAny(c, "*/+-") {
			continue
		}

		var left interface{}
		left, err := strconv.Atoi(chars[i-1])
		if err != nil {
			left = createBinaryGroups(tokenMap[chars[i-1]], tokenMap)
		}

		var right interface{}
		right, err = strconv.Atoi(chars[i+1])
		if err != nil {
			right = createBinaryGroups(tokenMap[chars[i+1]], tokenMap)
		}

		ops = append(ops, &binary{
			left:  left,
			right: right,
			op:    c,
		})
	}
	return ops
}

func computeBinary(symbol string) func([]*binary) []*binary {
	maybeSetPrevBinary := func(l int, ops []*binary, res int) {
		for l > 0 {
			if ops[l-1] != nil {
				leftSide := ops[l-1]
				leftSide.right = res
				break
			}
			l--
		}
	}

	maybeSetNextBinary := func(r int, ops []*binary, res int) {
		for r+1 < len(ops) {
			if ops[r+1] != nil {
				rightSide := ops[r+1]
				rightSide.left = res
				break
			}
			r++
		}
	}

	return func(ops []*binary) []*binary {
		debug := debugOps(symbol, ops)

		debug("input", "")

		printIndex := 0
		for i, b := range ops {
			if isSingleBinaryRemaining(ops) {
				break
			}
			if b.op != symbol {
				continue
			}

			res := evalBinary(b)
			ops[i] = nil

			maybeSetPrevBinary(i, ops, res)
			maybeSetNextBinary(i, ops, res)

			debug(strconv.Itoa(printIndex), "")
			printIndex++
		}

		ops = filterNils(ops)

		debug("output", "\n")

		return ops
	}
}

func evalBinary(b *binary) int {
	compute := buildComputeFrom(
		computeBinary("*"),
		computeBinary("/"),
		computeBinary("+"),
		computeBinary("-"),
	)

	var left int
	if l, ok := b.left.([]*binary); ok {
		left = compute(l)
	} else {
		left = b.left.(int)
	}

	var right int
	if r, ok := b.right.([]*binary); ok {
		right = compute(r)
	} else {
		right = b.right.(int)
	}

	switch b.op {
	case "*":
		return left * right
	case "/":
		return left / right
	case "+":
		return left + right
	case "-":
		return left - right
	default:
		return 0
	}
}

func toChars(input string) []string {
	var chars []string
	for _, s := range input {
		if strings.Trim(string(s), " ") == "" {
			continue
		}
		chars = append(chars, string(s))
	}
	return chars
}

func filterNils(ops []*binary) []*binary {
	var newOps []*binary
	for _, b := range ops {
		if b != nil {
			newOps = append(newOps, b)
		}
	}
	return newOps
}

func isSingleBinaryRemaining(ops []*binary) bool {
	var i int
	for _, b := range ops {
		if b == nil {
			i++
		}
	}
	return i == len(ops)-1
}

func tokenGenerator() func() string {
	tokens, i, r := "abcdefghijklmnopqrstuvwxyz", 0, 0
	return func() string {
		defer func() { i++ }()
		if i%len(tokens) == 0 {
			i = 0
			r++
		}
		return strings.Repeat(string(tokens[i]), r)
	}
}

func debugOps(symbol string, ops []*binary) func(string, string) {
	if debug := os.Getenv("DEBUG"); debug == "" || strings.ToLower(debug) == "false" {
		return func(_, _ string) {}
	}
	return func(prefix, postfix string) {
		fmt.Printf("\n %v %-6v [", symbol, prefix)
		for _, b := range ops {
			fmt.Print(b)
		}
		fmt.Printf("] %v", postfix)
	}
}
