if [ -z "$GEXPORT_PREEXEC_REGISTERED" ]; then
    preexec_functions+=(__gexport_preexec)
    GEXPORT_PREEXEC_REGISTERED=1
fi
