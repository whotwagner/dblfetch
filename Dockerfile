FROM rust:latest as builder

WORKDIR /myapp

# COPY . .
# 
# RUN cargo build --release
# 
# #CMD ["myapp"]
# 
# FROM debian:bullseye-slim
# 
# WORKDIR /usr/src/myapp
# copy --from=builder /usr/src/myapp/target/release/setup .
# copy --from=builder /usr/src/myapp/target/release/bridge .
# # CMD ["myapp"]
