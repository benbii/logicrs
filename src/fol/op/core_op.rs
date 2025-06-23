use super::define::define_core_op;
use super::{Sort, Term, TermResult, TermVec};
use crate::fol::BvConst;
use crate::{DagCnf, Lit, LitVvec};

#[inline]
fn bool_sort(_terms: &[Term]) -> Sort {
    Sort::Bv(1)
}

define_core_op!(Not, 1, bitblast: not_bitblast, cnf_encode: not_cnf_encode, simplify: not_simplify);
fn not_simplify(terms: &[Term]) -> TermResult {
    let x = &terms[0];
    if let Some(op) = x.try_op()
        && op.op == Not
    {
        return TermResult::Some(op[0].clone());
    }
    if let Some(xc) = x.try_bv_const() {
        return TermResult::Some(Term::bv_const(!xc));
    }
    TermResult::None
}
fn not_bitblast(terms: &[TermVec]) -> TermVec {
    terms[0].iter().map(|t| !t).collect()
}
fn not_cnf_encode(_dc: &mut DagCnf, terms: &[Lit]) -> Lit {
    !terms[0]
}

define_core_op!(And, 2, bitblast: and_bitblast, cnf_encode: and_cnf_encode, simplify: and_simplify);
fn and_simplify(terms: &[Term]) -> TermResult {
    let x = &terms[0];
    let y = &terms[1];
    let simp = |a: &Term, b: &Term| {
        if let Some(ac) = a.try_bv_const() {
            if ac.is_ones() {
                return TermResult::Some(b.clone());
            }
            if ac.is_zero() {
                return TermResult::Some(a.clone());
            }
        }
        if a == b {
            return TermResult::Some(a.clone());
        }
        if a == &!b {
            return TermResult::Some(a.mk_bv_const_zero());
        }
        if let Some(aop) = a.try_op() {
            if aop.op == And {
                if let Some(bop) = b.try_op()
                    && bop.op == And
                {
                    if aop[0] == bop[0] {
                        return TermResult::Some(&aop[0] & &aop[1] & &bop[1]);
                    }
                    if aop[0] == bop[1] {
                        return TermResult::Some(&aop[0] & &aop[1] & &bop[0]);
                    }
                }
                if b == &aop[0] {
                    return TermResult::Some(b & &aop[1]);
                }
                if b == &aop[1] {
                    return TermResult::Some(b & &aop[0]);
                }
            }
            if aop.op == Not
                && let Some(bop) = b.try_op()
                && bop.op == Not
            {
                return TermResult::Some(!(&aop[0] | &bop[0]));
            }
            if aop.op == Or
                && let Some(bop) = b.try_op()
                && bop.op == Or
            {
                if aop[0] == bop[0] {
                    return TermResult::Some(&aop[0] | (&aop[1] & &bop[1]));
                }
                if aop[0] == bop[1] {
                    return TermResult::Some(&aop[0] | (&aop[1] & &bop[0]));
                }
            }
        }
        TermResult::None
    };
    simp(x, y)?;
    simp(y, x)
}
fn and_bitblast(terms: &[TermVec]) -> TermVec {
    Term::new_op_elementwise(And, &terms[0], &terms[1])
}
fn and_cnf_encode(dc: &mut DagCnf, terms: &[Lit]) -> Lit {
    let l = dc.new_var().lit();
    dc.add_rel(l.var(), &LitVvec::cnf_and(l, terms));
    l
}

define_core_op!(Or, 2, bitblast: or_bitblast, cnf_encode: or_cnf_encode, simplify: or_simplify);
fn or_simplify(terms: &[Term]) -> TermResult {
    let x = &terms[0];
    let y = &terms[1];
    let simp = |a: &Term, b: &Term| {
        if let Some(ac) = a.try_bv_const() {
            if ac.is_ones() {
                return TermResult::Some(a.clone());
            }
            if ac.is_zero() {
                return TermResult::Some(b.clone());
            }
        }
        if a == b {
            return TermResult::Some(a.clone());
        }
        if a == &!b {
            return TermResult::Some(a.mk_bv_const_ones());
        }
        if let Some(aop) = a.try_op() {
            if aop.op == Or {
                if b == &aop[0] {
                    return TermResult::Some(b | &aop[1]);
                }
                if b == &aop[1] {
                    return TermResult::Some(b | &aop[0]);
                }
            }
            if aop.op == Not
                && let Some(bop) = b.try_op()
                && bop.op == Not
            {
                return TermResult::Some(!(&aop[0] & &bop[0]));
            }
            if aop.op == Ite {
                if b == &aop[0] {
                    return TermResult::Some(b | &aop[2]);
                }
                if b == &!&aop[0] {
                    return TermResult::Some(b | &aop[1]);
                }
            }
            if aop.op == And
                && let Some(bop) = b.try_op()
                && bop.op == And
            {
                if aop[0] == bop[0] {
                    return TermResult::Some(&aop[0] & (&aop[1] | &bop[1]));
                }
                if aop[0] == bop[1] {
                    return TermResult::Some(&aop[0] & (&aop[1] | &bop[0]));
                }
            }
        }
        TermResult::None
    };
    simp(x, y)?;
    simp(y, x)
}
fn or_bitblast(terms: &[TermVec]) -> TermVec {
    Term::new_op_elementwise(Or, &terms[0], &terms[1])
}
fn or_cnf_encode(dc: &mut DagCnf, terms: &[Lit]) -> Lit {
    let l = dc.new_var().lit();
    dc.add_rel(l.var(), &LitVvec::cnf_or(l, terms));
    l
}

define_core_op!(Xor, 2, bitblast: xor_bitblast, cnf_encode: xor_cnf_encode, simplify: xor_simplify);
fn xor_simplify(terms: &[Term]) -> TermResult {
    let x = &terms[0];
    let y = &terms[1];
    let simp = |a: &Term, b: &Term| {
        if let Some(ac) = a.try_bv_const() {
            if ac.is_ones() {
                return TermResult::Some(!b.clone());
            }
            if ac.is_zero() {
                return TermResult::Some(b.clone());
            }
        }
        if a == b {
            return TermResult::Some(a.mk_bv_const_zero());
        }
        if a == &!b {
            return TermResult::Some(a.mk_bv_const_ones());
        }
        TermResult::None
    };
    simp(x, y)?;
    simp(y, x)
}
fn xor_bitblast(terms: &[TermVec]) -> TermVec {
    Term::new_op_elementwise(Xor, &terms[0], &terms[1])
}
fn xor_cnf_encode(dc: &mut DagCnf, terms: &[Lit]) -> Lit {
    let l = dc.new_var().lit();
    dc.add_rel(l.var(), &LitVvec::cnf_xor(l, terms[0], terms[1]));
    l
}

define_core_op!(Eq, 2, sort: bool_sort, bitblast: eq_bitblast, cnf_encode: eq_cnf_encode, simplify: eq_simplify);
fn eq_simplify(terms: &[Term]) -> TermResult {
    let x = &terms[0];
    let y = &terms[1];
    let simp = |a: &Term, b: &Term| {
        if a.is_bool()
            && let TermResult::Some(s) = xor_simplify(terms)
        {
            return TermResult::Some(!s);
        }
        if a == b {
            return TermResult::Some(Term::bool_const(true));
        }
        if a == &!b {
            return TermResult::Some(Term::bool_const(false));
        }
        TermResult::None
    };
    simp(x, y)?;
    simp(y, x)
}
fn eq_bitblast(terms: &[TermVec]) -> TermVec {
    let neqs = Term::new_op_elementwise(Eq, &terms[0], &terms[1]);
    TermVec::from([Term::new_op_fold(And, &neqs)])
}
fn eq_cnf_encode(dc: &mut DagCnf, terms: &[Lit]) -> Lit {
    let l = dc.new_var().lit();
    dc.add_rel(l.var(), &LitVvec::cnf_xnor(l, terms[0], terms[1]));
    l
}

define_core_op!(Ult, 2, sort: bool_sort, bitblast: ult_bitblast, simplify: ult_simplify);
fn ult_simplify(terms: &[Term]) -> TermResult {
    let x = &terms[0];
    let y = &terms[1];
    if let Some(xc) = x.try_bv_const() {
        if xc.is_zero() {
            return TermResult::Some(!x.op1(Eq, y));
        }
        if xc.is_ones() {
            return TermResult::Some(Term::bool_const(false));
        }
    }
    if let Some(yc) = y.try_bv_const() {
        if yc.is_zero() {
            return TermResult::Some(Term::bool_const(false));
        }
        if yc.is_ones() {
            return TermResult::Some(!x.op1(Eq, y));
        }
    }
    TermResult::None
}
fn ult_bitblast(terms: &[TermVec]) -> TermVec {
    let mut res = Term::bool_const(false);
    for (x, y) in terms[0].iter().zip(terms[1].iter()) {
        res = (!x & y) | ((!x | y) & res)
    }
    TermVec::from([res])
}

define_core_op!(Slt, 2, sort: bool_sort, bitblast: slt_bitblast);
fn slt_bitblast(terms: &[TermVec]) -> TermVec {
    let x = &terms[0];
    let y = &terms[1];
    let len = x.len();
    let (xr, xs) = (&x[..len - 1], &x[len - 1]);
    let (yr, ys) = (&y[..len - 1], &y[len - 1]);
    let ls = xs & !ys;
    let eqs = xs.op1(Eq, ys);
    let mut el = Term::bool_const(false);
    for (x, y) in xr.iter().zip(yr.iter()) {
        el = (!x & y) | ((!x | y) & el)
    }
    TermVec::from([ls | (eqs & el)])
}

define_core_op!(Sll, 2, bitblast: sll_bitblast);
fn sll_bitblast(terms: &[TermVec]) -> TermVec {
    let (x, y) = (&terms[0], &terms[1]);
    assert!(x.len() == y.len());
    if terms[0].len() == 1 {
        return TermVec::from([&x[0] & !&y[0]]);
    }
    let width = x.len();
    let mut res = x.clone();
    for shift_bit in 0..width {
        let shift_step = 1 << shift_bit;
        let shift = &y[shift_bit];
        let mut nres = TermVec::new();
        for j in 0..shift_step.min(width) {
            nres.push(&!shift & &res[j]);
        }
        for j in shift_step..width {
            nres.push(Term::new_op(Ite, [shift, &res[j - shift_step], &res[j]]));
        }
        res = nres;
    }
    res
}

define_core_op!(Srl, 2, bitblast: srl_bitblast);
fn srl_bitblast(terms: &[TermVec]) -> TermVec {
    let (x, y) = (&terms[0], &terms[1]);
    assert!(x.len() == y.len());
    if terms[0].len() == 1 {
        return TermVec::from([&x[0] & !&y[0]]);
    }
    let width = x.len();
    let mut res = x.clone();
    for shift_bit in 0..width {
        let shift_step = 1 << shift_bit;
        let shift = &y[shift_bit];
        let mut nres = TermVec::new();
        let c = width.saturating_sub(shift_step);
        for j in 0..c {
            nres.push(Term::new_op(Ite, [shift, &res[j + shift_step], &res[j]]));
        }
        for j in c..width {
            nres.push(&!shift & &res[j]);
        }
        res = nres;
    }
    res
}

define_core_op!(Sra, 2, bitblast: sra_bitblast);
fn sra_bitblast(terms: &[TermVec]) -> TermVec {
    let (x, y) = (&terms[0], &terms[1]);
    assert!(x.len() == y.len());
    if terms[0].len() == 1 {
        return x.clone();
    }
    let width = x.len();
    let mut res = x.clone();
    for shift_bit in 0..width {
        let shift_step = 1 << shift_bit;
        let c = width.saturating_sub(shift_step);
        let shift = &y[shift_bit];
        let mut nres = TermVec::new();
        for j in 0..c {
            nres.push(Term::new_op(Ite, [shift, &res[j + shift_step], &res[j]]));
        }
        for j in c..width {
            nres.push(Term::new_op(Ite, [shift, &res[width - 1], &res[j]]));
        }
        res = nres;
    }
    res
}

define_core_op!(Ite, 3, sort: ite_sort, bitblast: ite_bitblast, cnf_encode: ite_cnf_encode, simplify: ite_simplify);
fn ite_sort(terms: &[Term]) -> Sort {
    terms[1].sort()
}
fn ite_simplify(terms: &[Term]) -> TermResult {
    let (c, t, e) = (&terms[0], &terms[1], &terms[2]);
    if let Some(cc) = c.try_bv_const() {
        if cc.is_ones() {
            return TermResult::Some(t.clone());
        } else {
            return TermResult::Some(e.clone());
        }
    }
    if t == e {
        return TermResult::Some(t.clone());
    }
    if let Some(cop) = c.try_op()
        && cop.op == Not
    {
        return TermResult::Some(cop[0].ite(e, t));
    }
    if t.is_bool() {
        if let Some(ec) = e.try_bv_const() {
            if ec.is_zero() {
                return TermResult::Some(c & t);
            }
            if ec.is_ones() {
                return TermResult::Some(!c | t);
            }
        }
        if let Some(tc) = t.try_bv_const() {
            if tc.is_zero() {
                return TermResult::Some(!c & e);
            }
            if tc.is_ones() {
                return TermResult::Some(c | e);
            }
        }
    }
    TermResult::None
}
fn ite_bitblast(terms: &[TermVec]) -> TermVec {
    let mut res = TermVec::new();
    for (x, y) in terms[1].iter().zip(terms[2].iter()) {
        res.push(terms[0][0].op2(Ite, x, y));
    }
    res
}
fn ite_cnf_encode(dc: &mut DagCnf, terms: &[Lit]) -> Lit {
    let l = dc.new_var().lit();
    dc.add_rel(l.var(), &LitVvec::cnf_ite(l, terms[0], terms[1], terms[2]));
    l
}

define_core_op!(Concat, 2, sort: concat_sort, bitblast: concat_bitblast, simplify: concat_simplify);
fn concat_simplify(terms: &[Term]) -> TermResult {
    let x = &terms[0];
    let y = &terms[1];
    if let (Some(xc), Some(yc)) = (x.try_bv_const(), y.try_bv_const()) {
        let mut c = yc.to_vec();
        c.extend_from_slice(xc);
        return TermResult::Some(Term::bv_const(BvConst::new(&c)));
    }
    TermResult::None
}
fn concat_sort(terms: &[Term]) -> Sort {
    Sort::Bv(terms[0].bv_len() + terms[1].bv_len())
}
fn concat_bitblast(terms: &[TermVec]) -> TermVec {
    let mut res = terms[1].clone();
    res.extend_from_slice(&terms[0]);
    res
}

define_core_op!(Sext, 2, sort: sext_sort, bitblast: sext_bitblast);
fn sext_sort(terms: &[Term]) -> Sort {
    Sort::Bv(terms[0].bv_len() + terms[1].bv_len())
}
fn sext_bitblast(terms: &[TermVec]) -> TermVec {
    let x = &terms[0];
    let mut res = x.clone();
    let ext = vec![x[x.len() - 1].clone(); terms[1].len()];
    res.extend(ext);
    res
}

define_core_op!(Slice, 3, sort: slice_sort, bitblast: slice_bitblast, simplify: slice_simplify);
fn slice_simplify(_terms: &[Term]) -> TermResult {
    TermResult::None
}
fn slice_sort(terms: &[Term]) -> Sort {
    Sort::Bv(terms[1].bv_len() - terms[2].bv_len() + 1)
}
fn slice_bitblast(terms: &[TermVec]) -> TermVec {
    let l = terms[2].len();
    let h = terms[1].len();
    terms[0][l..=h].iter().cloned().collect()
}

define_core_op!(Redxor, 1, sort: bool_sort, bitblast: redxor_bitblast);
fn redxor_bitblast(terms: &[TermVec]) -> TermVec {
    TermVec::from([Term::new_op_fold(Xor, terms[0].iter())])
}

#[inline]
fn full_adder(x: &Term, y: &Term, c: &Term) -> (Term, Term) {
    let r = Term::new_op_fold(Xor, [x, y, c]);
    let xy = x & y;
    let xc = x & c;
    let yc = y & c;
    let c = Term::new_op_fold(Or, [&xy, &xc, &yc]);
    (r, c)
}

define_core_op!(Add, 2, bitblast: add_bitblast);
fn add_bitblast(terms: &[TermVec]) -> TermVec {
    let mut r;
    let mut c = Term::bool_const(false);
    let mut res = TermVec::new();
    for (x, y) in terms[0].iter().zip(terms[1].iter()) {
        (r, c) = full_adder(x, y, &c);
        res.push(r);
    }
    res
}

define_core_op!(Mul, 2, bitblast: mul_bitblast);
fn mul_bitblast(terms: &[TermVec]) -> TermVec {
    let x = &terms[0];
    let y = &terms[1];
    assert!(x.len() == y.len());
    let len = x.len();
    let mut res: TermVec = x.iter().map(|t| t & &y[0]).collect();
    for i in 1..len {
        let mut c = Term::bool_const(false);
        for j in i..len {
            let add = &y[i] & &x[j - i];
            (res[j], c) = full_adder(&res[j], &add, &c);
        }
    }
    res
}

define_core_op!(Read, 2, sort: read_sort, bitblast: read_bitblast);
fn read_sort(terms: &[Term]) -> Sort {
    let (_, e) = terms[0].sort().array();
    Sort::Bv(e)
}

fn onehot_encode(x: &[Term]) -> TermVec {
    let len = 1_usize.checked_shl(x.len() as u32).unwrap();
    let mut res = vec![Term::bool_const(false); len];
    res[0] = Term::bool_const(true);
    for (sb, shift) in x.iter().enumerate() {
        let ss = 1 << sb;
        for rj in &mut res[0..ss] {
            *rj = !shift & &rj;
        }
        for j in ss..len {
            res[j] = shift.ite(&res[j - ss], &res[j]);
        }
    }
    TermVec::from(res.as_slice())
}

fn read_bitblast(terms: &[TermVec]) -> TermVec {
    let (array, index) = (&terms[0], &terms[1]);
    let index_len = index.len();
    let array_len = array.len();
    let index_range = 1_usize.checked_shl(index_len as u32).unwrap();
    let element_len = array_len / index_range;
    let onehot = onehot_encode(index);
    let mut res = TermVec::new();
    for i in 0..element_len {
        let mut r = Term::bool_const(false);
        for j in 0..index_range {
            r = onehot[j].ite(&array[element_len * j + i], &r);
        }
        res.push(r);
    }
    res
}

define_core_op!(Write, 3, bitblast: write_bitblast);
fn write_bitblast(terms: &[TermVec]) -> TermVec {
    let (array, index, value) = (&terms[0], &terms[1], &terms[2]);
    let index_len = index.len();
    let array_len = array.len();
    let index_range = 1_usize.checked_shl(index_len as u32).unwrap();
    let element_len = array_len / index_range;
    let onehot = onehot_encode(index);
    let mut res = array.clone();
    for i in 0..element_len {
        for j in 0..index_range {
            let r = &mut res[element_len * j + i];
            *r = onehot[j].ite(&value[i], r);
        }
    }
    res
}
