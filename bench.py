#!/usr/bin/python2.7

import subprocess
import sys

HEADER = '''\
#![feature(phase)]

extern crate seq;
#[phase(plugin)]
extern crate seq_macros;
extern crate test;

use test::Bencher;
'''

ITER = '''
#[bench]
fn iter_{n}(b: &mut Bencher) {{
    b.iter(|| {{
        let v: Vec<Box<int>> = iter![
            {elems}
        ].collect();
        v
    }})
}}
'''

SEQ = '''
#[bench]
fn seq_{n}(b: &mut Bencher) {{
    b.iter(|| {{
        let v: Vec<Box<int>> = seq![
            {elems}
        ];
        v
    }})
}}
'''

BENCHES = [ITER, SEQ]

ELEM = 'box {}'

BENCH_SIZE = int(sys.argv[1])

with open('benches/compare.rs', 'w') as f:
    f.write(HEADER)

    for bench in BENCHES:
        elems = ''
        for i in range(BENCH_SIZE):
            elems += ELEM.format(i) + ',\n'
        f.write(bench.format(n=BENCH_SIZE, elems=elems))

subprocess.call(['cargo', 'bench'])
