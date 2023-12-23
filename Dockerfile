FROM rust:bullseye

WORKDIR /myapp

COPY . .
# 
# # 
# CMD ["myapp"]
# # 
# FROM debian:bullseye-slim
# 
# WORKDIR /usr/src/myapp
# #copy --from=builder /myapp/target/release/setup .
# copy --from=builder /myapp/target/release/bridge .
# # CMD ["myapp"]
