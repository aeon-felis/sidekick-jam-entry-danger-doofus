from omnipytent import *
from omnipytent.ext.idan import *

import json


@task
def check(ctx):
    cargo['check', '-q'] & ERUN.bang


@task
def build(ctx):
    cargo['build'][
        '--features', 'bevy/dynamic',
    ] & TERMINAL_PANEL.size(20)


@task
def run(ctx):
    cargo['run'][
        '--features', 'bevy/dynamic',
    ].with_env(
        RUST_LOG='danger_doofus=info,bevy_yoleck=info',
        RUST_BACKTRACE='1',
    ) & TERMINAL_PANEL.size(20)


@task.options(alias=':0')
def level(ctx):
    ctx.key(lambda level: level['filename'].removesuffix('.yol').replace('_', ' '))
    ctx.value(lambda level: level['filename'].removesuffix('.yol'))
    with local.path('assets/levels/index.yoli').open() as f:
        level_index = json.load(f)
    for level in level_index:
        yield level


@task
def execute(ctx, level=level):
    cargo['run'][
        '--features', 'bevy/dynamic',
        '--', '--level', level,
    ].with_env(
        RUST_LOG='danger_doofus=info,bevy_yoleck=info',
        RUST_BACKTRACE='1',
    ) & TERMINAL_PANEL.size(20)


@task
def go(ctx):
    cargo['run'][
        '--features', 'bevy/dynamic',
        '--', '--editor',
    ].with_env(
        RUST_LOG='danger_doofus=info',
        RUST_BACKTRACE='1',
    ) & TERMINAL_PANEL.size(20)


@task
def clean(ctx):
    cargo['clean'] & BANG


@task
def launch_wasm(ctx):
    cargo['run'][
        '--target', 'wasm32-unknown-unknown'
    ].with_env(
        RUST_BACKTRACE='1',
    ) & TERMINAL_PANEL.size(20)


@task
def browse_wasm(ctx):
    local['firefox']['http://127.0.0.1:1334']()


@task
def clippy(ctx):
    cargo['clippy'] & ERUN.bang
