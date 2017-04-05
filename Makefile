build:
	cargo build --release
	cp ./target/release/httpd ./

clean:
	${RM} -r target