__gexport_config_dir() {
    local source_file='gexport/gexports'
    if [ -n "$XDG_CONFIG_HOME" ]; then
        echo "${XDG_CONFIG_HOME}/${source_file}"
    elif [ -n "$HOME" ]; then
        echo "${HOME}/.config/${source_file}"
    else
        return 1
    fi
}

GEXPORT_SOURCE_FILE="$(__gexport_config_dir)" || return 1

__gexport_ensure_files() {
    if [ ! -f "$GEXPORT_SOURCE_FILE" ]; then
        mkdir -p "$(dirname "$GEXPORT_SOURCE_FILE")"
        touch "$GEXPORT_SOURCE_FILE"
    fi
}

__gexport_ensure_files
GEXPORT_MOD_TIME="$(stat -c %Y "$GEXPORT_SOURCE_FILE")"

__gexport_preexec() {
    __gexport_ensure_files
    local mod_time="$(stat -c %Y "$GEXPORT_SOURCE_FILE")"
    if [ "$mod_time" -ne "$GEXPORT_MOD_TIME" ]; then
        source "$GEXPORT_SOURCE_FILE"
        GEXPORT_MOD_TIME="$mod_time"
    fi
}

source "$GEXPORT_SOURCE_FILE"
