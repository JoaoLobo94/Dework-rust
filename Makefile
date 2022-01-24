local_build:
	docker build -t dework_local -f Dockerfile .

local_run:
	docker run --rm -it -p 8000:8000 --mount type=bind,source="$(shell pwd)",target=/app --name dework_local dework_local cargo watch -x 'run --bin dework'

local_stop:
	docker stop dework_local