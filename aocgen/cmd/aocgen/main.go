package main

import (
	"flag"
	"fmt"
	"os"
	"path"
	"text/template"
)

var (
	fYear        = flag.Uint("year", 0, "specifies year")
	fDay         = flag.Uint("day", 0, "specifies day")
	fLang        = flag.String("lang", "", "programming language of solutions")
	fBasedir     = flag.String("basedir", "", `base directory of solutions (default "../<value of -lang>")`)
	fTemplatedir = flag.String("templatedir", "", `directory of template files for solutions (default "templates/<value of -lang>")`)
)

type templateData struct {
	Year      uint
	FullYear  string
	Day       uint
	PaddedDay string
	FullDay   string
}

func imain() int {
	flag.Parse()

	lang := *fLang
	if lang == "" {
		fmt.Println("a programming language must be specified with the -lang flag")
		return 1
	}

	year := *fYear
	if year == 0 {
		fmt.Println("a year must be specified with the -year flag")
		return 1
	}
	if year < 2015 || year > 2019 {
		fmt.Printf("invalid year %d: the year must be a year on which an AoC event was held\n", year)
		return 1
	}

	day := *fDay
	if day == 0 {
		fmt.Println("a day must be specified with the -day flag")
		return 1
	}
	if day > 25 {
		fmt.Printf("invalid day %d: the day must be in the range 1-25 (both inclusive)\n", day)
		return 1
	}

	basedir := *fBasedir
	if basedir == "" {
		basedir = fmt.Sprintf("../%s", lang)
	}

	templatedir := *fTemplatedir
	if templatedir == "" {
		templatedir = fmt.Sprintf("templates/%s", lang)
	}

	fullYear := fmt.Sprintf("year%d", year)
	fullDay := fmt.Sprintf("day%02d", day)
	data := templateData{
		Year:      year,
		FullYear:  fullYear,
		Day:       day,
		PaddedDay: fmt.Sprintf("%02d", day),
		FullDay:   fullDay,
	}

	outputdir := path.Join(basedir, fullYear, fullDay)
	if _, err := os.Stat(outputdir); os.IsNotExist(err) {
		if err := os.MkdirAll(outputdir, os.ModePerm); err != nil {
			fmt.Printf("error creating output directory %s: %+v\n", outputdir, err)
			return 2
		}
	}

	for _, tt := range []struct {
		name   string
		output string
	}{
		{name: "BUILD.bazel"},
		{name: "dayXX.h", output: fmt.Sprintf("%s.h", fullDay)},
		{name: "dayXX.cc", output: fmt.Sprintf("%s.cc", fullDay)},
		{name: "test.cc"},
		{name: "benchmark.cc"},
	} {
		templatePath := path.Join(templatedir, tt.name)
		tmpl, err := template.ParseFiles(templatePath)
		if err != nil {
			fmt.Printf("error parsing template %s: %+v\n", templatePath, err)
			return 2
		}
		outputFilename := tt.output
		if outputFilename == "" {
			outputFilename = tt.name
		}
		outputPath := path.Join(outputdir, outputFilename)
		templateFile, err := os.Create(outputPath)
		if err != nil {
			fmt.Printf("error creating output file %s: %+v\n", outputPath, err)
			return 2
		}
		defer func() {
			if err := templateFile.Close(); err != nil {
				fmt.Printf("error closing output file %s: %+v\n", outputPath, err)
			}
		}()
		tmpl.Execute(templateFile, data)
	}

	return 0
}

func main() {
	os.Exit(imain())
}
