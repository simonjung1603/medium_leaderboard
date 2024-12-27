mv /app/target/release/medium_leaderboard /app/target/release/shuttle_medium_leaderboard
dx build --release
rm /app/target/release/medium_leaderboard
mv /app/target/release/shuttle_medium_leaderboard /app/target/release/medium_leaderboard
mv /app/target/dx/medium_leaderboard/release/web/public public