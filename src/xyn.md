
variant Expr {
    Integer {
        i: i32
    }
    Binary {
        left: ref Expr,
        op: OpKind,
        right: ref Expr
    }
}

impl Expr {
    pub fn to_i32(self) i32 {
        return match self {
            Self::Integer => self.i,
            // refinement here
            Self::Binary => match self.op {
                OpKind::Add => self.left.to_i32() + self.right.to_i32(),
                OpKind::Sub => self.left.to_i32() - self.right.to_i32(),
                OpKind::Mul => self.left.to_i32() * self.right.to_i32(),
                OpKind::Div => self.left.to_i32() / self.right.to_i32(),
            }
        };
    }
}