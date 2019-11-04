package inputs

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/require"
)

func TestOpen(t *testing.T) {
	allDays := []int{1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31}
	for _, tt := range []struct {
		year int
		days []int
	}{
		{year: 2015, days: allDays},
		{year: 2016, days: allDays},
		{year: 2017, days: allDays},
		{year: 2018, days: allDays},
	} {
		tt := tt
		for _, day := range tt.days {
			t.Run(fmt.Sprintf("%d/%02d", tt.year, day), func(t *testing.T) {
				_, err := Open(tt.year, day)
				require.NoError(t, err)
			})
		}
	}
}
