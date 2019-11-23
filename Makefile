.PHONY: up bash logs down

up:
	docker-compose up -d

bash1:
	docker-compose exec server1 bash

bash2:
	docker-compose exec server2 bash

logs1:
	docker container logs -f server1

logs2:
	docker container logs -f server2

down:
	docker-compose down
