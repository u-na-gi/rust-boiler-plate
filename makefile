build: 
	docker-compose up -d --build


exec:
	docker exec -it runacy-note-backend-rs bash

watch:
	cargo watch -x run