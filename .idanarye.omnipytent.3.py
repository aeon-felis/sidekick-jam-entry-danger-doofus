from omnipytent import *
from omnipytent.ext.idan import *


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
    local['chrome']['http://127.0.0.1:1334']()


@task
def clippy(ctx):
    cargo['clippy'] & ERUN.bang
