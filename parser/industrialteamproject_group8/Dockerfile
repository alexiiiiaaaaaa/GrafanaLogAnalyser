FROM python:3.11-slim

WORKDIR /app

COPY ./app.py /app
COPY ./config.txt /app/config.txt
COPY ./analysis /app/analysis

RUN pip install pydevd-pycharm~=243.23654.177  # Match your PyCharm version

EXPOSE 8080

CMD ["python", "-u", "-W", "all", "-X", "dev",  "app.py"]
#CMD ["tail", "-f", "/dev/null"]