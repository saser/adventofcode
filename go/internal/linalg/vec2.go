package linalg

type Vec2 struct {
	X,
	Y int
}

func (v *Vec2) Add(v2 *Vec2) *Vec2 {
	v.X += v2.X
	v.Y += v2.Y
	return v
}

func (v *Vec2) Sub(v2 *Vec2) *Vec2 {
	v.X -= v2.X
	v.Y -= v2.Y
	return v
}

func (v *Vec2) Mul(i int) *Vec2 {
	v.X *= i
	v.Y *= i
	return v
}

func (v *Vec2) Div(i int) *Vec2 {
	v.X /= i
	v.Y /= i
	return v
}

type Mat2 struct {
	X1, Y1,
	X2, Y2 int
}

func (v *Vec2) MatMul(m Mat2) *Vec2 {
	x := m.X1*v.X + m.Y1*v.Y
	y := m.X2*v.X + m.Y2*v.Y
	v.X = x
	v.Y = y
	return v
}
