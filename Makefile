NAME = scop

all: $(NAME)

$(NAME): 
	cargo build --release
	@mv target/release/scop ./scop

clean:
	rm -rf target

fclean: clean
	rm -rf $(NAME)

re: fclean all

run: 
	cargo run

.PHONY: clean fclean re run