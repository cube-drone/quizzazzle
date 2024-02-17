default:
	cat Makefile

s:
	docker compose up -d
	docker container restart ministry && docker logs --follow ministry
start:
	docker compose up -d
	docker container restart ministry && docker logs --follow ministry

c:
	docker compose down
clean:
	docker compose down

j:
	docker logs --follow ministry
join:
	docker logs --follow ministry


scyllastatus:
	docker exec -it some-scylla nodetool status
cqlsh:
	docker exec -it some-scylla cqlsh