from itertools import product
from typing import Iterable

from pysat.formula import And, Atom, Formula, Implies, Neg, Or, XOr
from pysat.solvers import Glucose4

"""
Define graph G(V, E) as
    vertices V = { v0, v1, ... }
    edges E = { (v0, v1), (v1, v2), (v1, v3), ... }
        where all of
            (v_a, v_b) in E implies (v_b, v_a) in E
                ie edges are undirected
            (v_a, v_b) in E implies (v_a, v_a) in E and (v_b, v_b) in E
                ie each vertex shares an edge with itself
---

Given graph G(V_g, E_g)
find connected subgraph S(V_s, E_s) of size k

variables
    v1, v2, v3, ...
        where v0 = true means vertex v0 is in the subgraph
        [n]
    e_1_0, e_2_1, ...
        where e_0_1 = true means vertex v0 shares an edge with v1
        [n**2]
    d_1_0, d_1_1, ... d_1_n, d_2_0, d_3_1, ... d_k_n
        where d_t_a indicates that vertex a was chosen at step t
        [n * k]
    
constraint clauses
    edge variables are given
        e_a_b = ...
        [n]
    vertices have a self-edge
        for every v_a, e_a_a = true
        [n]
    from graph definition
        undirected edges
            e_a_b implies e_b_a
            [n**2]
    decisions
        vertex is included if decision
            d_t_a implies d_a
            [n * k]
        one decision per time step
            XOR ( d_t_0, ..., d_t_k } )
            [n**2 * k**2]
        connectivity
            d_t1_a implies d_t0_b and e_a_b
                for some t0 < t1 and b
            [n**2 * k**2]
        no cycles
            for t0 != t1, not (d_t0_a and d_t1_a)
            [n * k**2]
"""


def fac(n: int) -> int:
    if n == 1:
        return 1

    return n * fac(n - 1)


def build_constraints(
    num_vertices: int, num_pick: int, edges: Iterable[tuple[int, int]]
):
    n = num_vertices
    iter_n = list(range(n))
    iter_nn = list(product(iter_n, iter_n))

    k = num_pick
    iter_k = list(range(k))
    iter_kn = list(product(iter_k, iter_n))
    iter_kk = list(product(iter_k, iter_k))

    vs = []
    for i in iter_n:
        vs.append(Atom(f"v{i}"))

    es = []
    for i in iter_n:
        es.append([Atom(f"e_{i}_{j}") for j in iter_n])

    ds = []
    for t in iter_k:
        ds.append([Atom(f"d_{t}_{i}") for i in iter_n])

    constraints = []

    # graph is non-empty
    constraints.append(Or(*[vs[i] for i in iter_n]))

    # self edge
    for i in iter_n:
        constraints.append(es[i][i])

    # undirected graph edges
    for i, j in iter_nn:
        constraints.append(Implies(es[i][j], es[j][i]))

    # include vertexes from decisions
    for t, i in iter_kn:
        constraints.append(Implies(ds[t][i], vs[i]))
    for i in iter_n:
        constraints.append(
            Implies(
                vs[i],
                Or(*[ds[t][i] for t in iter_k]),
            )
        )

    # one decision per time step
    for t in iter_k:
        constraints.append(XOr(*ds[t]))

    # connectivity
    for t1, i in iter_kn:
        if t1 == 0:
            continue

        is_connected = []
        for t0 in range(t1):
            for j in iter_n:
                is_connected.append(
                    And(
                        ds[t0][j],
                        es[i][j],
                    )
                )

        constraints.append(
            Implies(
                ds[t1][i],
                Or(*is_connected),
            )
        )

    # no cycles
    for i in iter_n:
        for t0, t1 in iter_kk:
            if t0 == t1:
                continue

            constraints.append(
                Neg(
                    And(
                        ds[t0][i],
                        ds[t1][i],
                    )
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

    constraints = build_constraints(2, 2, [(0, 1)])

    with Glucose4(bootstrap_with=And(*constraints)) as solver:
        models = list(solver.enum_models())
        for model in models:
            print_model(model)

        assert len(models) == 1 * fac(2)


def test_abc():
    """
    a - b - c
    """

    constraints = build_constraints(3, 2, [(0, 1), (1, 2)])

    with Glucose4(bootstrap_with=And(*constraints)) as solver:
        models = list(solver.enum_models())
        for model in models:
            print_model(model)


def test_diamond():
    """
      a
     / \
    b   c
     \ /
      d
    """

    constraints = build_constraints(4, 3, [(0, 1), (0, 2), (1, 3), (2, 3)])

    with Glucose4(bootstrap_with=And(*constraints)) as solver:
        models = list(solver.enum_models())
        for model in models:
            print_model(model)


if __name__ == "__main__":
    # test_ab()
    # test_abc()
    test_diamond()
