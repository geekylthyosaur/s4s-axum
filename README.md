# Blog
Blog-like API written using Actix-Web
## Run
* before build ```./scripts/init_db.sh``` (```docker``` needed)
* build ```cargo build```
* run ```cargo run```
* test ```cargo test```
## Note
Currently working on complete user auth cycle.
Business logic of this API is a simple part of project, so the only routes implemented is ```/user/signup``` and ```/user/login```.
