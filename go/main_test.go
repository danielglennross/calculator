package main

import "testing"

func TestCalculator(t *testing.T) {
	testCases := []struct {
		Name, Expr string
		Want       int
	}{
		{Name: "Simple1", Expr: "1 + 2 * 3 + 4 - 5 * 6 + 7", Want: -26},
		{Name: "Simple2", Expr: "1 + 2 - 3 * 4 * 5 - 6 + 7", Want: -70},
		{Name: "Simple3", Expr: "1 * 2 + 3 + 4 - 5 - 6 * 7", Want: -38},
		{Name: "Brackets1", Expr: "1 + 2 * (3 + 4) - 5 * (6 * (7 + 8))", Want: -435},
	}

	for _, tc := range testCases {
		tc := tc
		t.Run(tc.Name, func(t *testing.T) {
			t.Parallel()
			res := Calculate(tc.Expr)
			if tc.Want != res {
				t.Errorf("for %v; got %v; want %v", tc.Expr, res, tc.Want)
			}
		})
	}
}
