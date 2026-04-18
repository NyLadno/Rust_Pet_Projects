# Stage 1: Сборка (Build)
# Используем официальный образ Rust (версия slim для экономии места и ускорения скачивания)
FROM rust:1.80-slim-bookworm AS builder

# Устанавливаем зависимости системы, которые могут понадобиться для сборки (например, pkg-config и ssl для reqwest)
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Устанавливаем рабочую директорию внутри контейнера
WORKDIR /usr/src/app

# Копируем файлы проекта
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Запускаем сборку оптимизированной релизной версии
RUN cargo build --release


# Stage 2: Выполнение (Runtime)
# Используем минимальный образ Debian, соответствующий тому, на котором собирали
FROM debian:bookworm-slim

# Для работы webhook (reqwest) по HTTPS нужны корневые сертификаты SSL
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Создаем рабочую директорию
WORKDIR /app

# Копируем собранный исполняемый файл из первого стейджа
COPY --from=builder /usr/src/app/target/release/fs-watch-rs /usr/local/bin/fs-watch-rs

# Точка входа в контейнер (программа, которая будет запущена)
ENTRYPOINT ["fs-watch-rs"]

# Задаем команду по умолчанию (вывести help, если пользователь ничего не передал)
CMD ["--help"]
