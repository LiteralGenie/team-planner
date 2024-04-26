from itertools import product
from typing import Iterable

from pysat.formula import And, Atom, Formula, Implies, Neg, Or
from pysat.solvers import Glucose4

"""
Define graph G(V, E) as
    vertices V = { v0, v1, ... }
    edges E = { (v0, v1), (v1, v2), (v1, v3), ... }
        where all of
            (v_a, v_b) in E implies (v_b, v_a) in E
                ie edges are undirected
            (v_a, v_b) in E implies (v_a, v_a) in E and (v_b, v_b) in E
                ie each node shares an edge with itself
                    is this necessary?

Given a graph G(V,E) define the reachability matrix R(G) as
    a set of 2-tuples of elements from V
    where (v_a, v_b) in R implies there exists a sequence (v_a, v_i1, v_i2, v_i3, ..., v_in v_b)
        such that each consecutive pair { (v_a, v_i0), (v_i1, v_i2), ... } is in E
        for n >= 0
            if 0, no intermediate points, only v_a and v_b
        ie theres a "path" from v_a to v_b
    (v_a, v_b) in E implies (v_a, v_b) in R
        ie if v_a and v_b share an edge, v_a is reachable from v_b
    since E is reflexive / symmetric, so is R
    (v_a, v_b) in R and (v_b, v_c) in R implies (v_a, v_c) in R
        ie if two nodes a and c are connected to an intermediate node b, a and b are connected

A graph G(V, E) is connected if (v_a, v_b) in R(G) for every (v_a, v_b) in V

---

Given graph G(V_g, E_g)
find connected subgraph S(V_s, E_s) of size k

variables
    v0, v1, v2, ...
        where v0 = true means vertex v0 is in the subgraph
    e_0_0, e_0_1, ...
        where e_0_1 = true means vertex v0 shares an edge with v1
    r_0_0, r_0_1, ... r_a_b, r_a_(b+1), ...
        where r_0_1 = true means vertex v1 is reachable from v0
    

variable count is (n + n**2 + n**2)
    
constraint clauses
    edge variables are given
        e_a_b = ...
    vertices have a self-edge
        for every v_a, e_a_a = true
    from graph definition
        undirected edges
            e_a_b implies e_b_a
    from reachability definition
        edges imply reachability
            e_a_b implies r_a_b
        transitive
            r_a_b and r_b_c implies r_a_c
    from connectivity definition
        v_a and v_b implies r_a_b


constraint count is
    n**2 + 
    n**2 + n**2 +
    n**2 + n**3 +
    n**2
"""

# def build


def build_constraints(num_vertices: int, edges: Iterable[tuple[int, int]]):
    n = num_vertices
    iter_n = list(range(n))
    iter_nn = list(product(iter_n, iter_n))
    iter_nnn = list(product(iter_n, iter_n, iter_n))

    vs = []
    for i in iter_n:
        vs.append(Atom(f"v{i}"))

    es, rs = [], []
    for i in iter_n:
        es.append([Atom(f"e_{i}_{j}") for j in iter_n])
        rs.append([Atom(f"r_{i}_{j}") for j in iter_n])

    constraints = []

    # graph is non-empty
    constraints.append(Or(*[vs[i] for i in iter_n]))

    for i in iter_n:
        # self edge
        constraints.append(es[i][i])

    for i, j in iter_nn:
        # undirected graph edges
        constraints.append(Implies(es[i][j], es[j][i]))

        # edges imply reachability
        constraints.append(Implies(es[i][j], rs[i][j]))

        # connectivity definition
        constraints.append(Implies(And(vs[i], vs[j]), rs[i][j]))

        # reachability implies inclusion in result
        constraints.append(
            Implies(
                And(Or(vs[i], vs[j]), rs[i][j]),
                And(vs[i], vs[j]),
            )
        )

    for i, j, k in iter_nnn:
        # reachability is transitive
        constraints.append(
            Implies(
                And(
                    rs[i][j],
                    rs[j][k],
                ),
                rs[i][k],
            )
        )

    # edges
    conns = dict()
    edges = set(edges)
    for i, j in iter_nn:
        if i == j:
            conns[(i, j)] = es[i][j]
            continue

        if (i, j) in edges or (j, i) in edges:
            conns[(i, j)] = es[i][j]
            continue

        conns[(i, j)] = Neg(es[i][j])

    constraints.extend(conns.values())

    return constraints


def print_model(model):
    formulas = Formula.formulas(
        model,
        atoms_only=True,
    )
    print([f for f in formulas if getattr(f, "object", "").startswith("v")])
    # print(formulas)


def test_ab():
    """
    a - b
    """

    constraints = build_constraints(2, [(0, 1)])

    with Glucose4(bootstrap_with=And(*constraints)) as solver:
        for model in solver.enum_models():
            print_model(model)


def test_abc():
    """
    a - b - c
    """

    constraints = build_constraints(3, [(0, 1), (1, 2)])

    with Glucose4(bootstrap_with=And(*constraints)) as solver:
        for model in solver.enum_models():
            print_model(model)


if __name__ == "__main__":
    test_abc()
