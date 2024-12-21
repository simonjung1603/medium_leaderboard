apt update
apt install -y libpq5
apt install -y libpq-dev
PGSQL_LIB=$(find / -name libpq.so.5)
ln -s "$PGSQL_LIB" /usr/lib/libpq.so.5
LD_LIBRARY_PATH="$PGSQL_LIB"
export LD_LIBRARY_PATH