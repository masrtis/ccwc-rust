FROM rust:latest AS build

WORKDIR /tmp/build/

COPY Cargo.* ./
COPY ./src/ ./src/

RUN cargo build --verbose --all-targets --release

ENTRYPOINT ["/bin/bash"]

FROM python:latest AS test

RUN pip install pytest

WORKDIR /tmp/test

COPY integration_tests/* .
COPY --from=build /tmp/build/target/release/ccwc-rust .

ENV CCWC_PATH /tmp/test/ccwc-rust
ENV TEST_DATA_PATH test.txt

CMD ["pytest"]
