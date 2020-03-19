# aliyun-ddns for rust
Due to aliyun-ddns sdk not provide rust version, this is written by rust which follows aliyun docs.


## builg at linux
Make sure you also have the development packages of openssl installed.
For example, `libssl-dev` on Ubuntu or `openssl-devel` on Fedora or 'openssl-devel' on Centos/RedHat


## how to use
抱歉来迟了，补充了篇中文的配置文档:
[aliyun-ddns设置定时监控更新](https://eternaless.com/posts/625554dc/)

### check config
- config.yaml

setting your `change_sub_domains` and `authorization` 

```yaml
# config sub-domains<only sub-domain> here
change_sub_domains:
  - 'hello' # hello.infot.me
  - 'world' # world.infot.me

# here is public ip provider(page and public ip regex extractor), u also can config more provider follow below format
ip_provider:
  -
    provider: 'https://www.ip.cn'
    regex_pattern: '(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})'
  -
    provider: 'http://pv.sohu.com/cityjson'
    regex_pattern: '(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})'

# aliyun authorization
authorization:
  access_key_id: 'IAMasd879679mSAD9'
  access_key_secret: 'IAMasd879679mSAD9IAMasd879679mSAD9'
  current_root_domain: 'infot.me'                   # the root-domain
  region_id: 'cn-shenzhen'                          # city code, China cities default is cn-<city-full-name>
```

- log4rs.yaml 

setting your `root-level` logger level and `appenders-general, appenders-other` log redirect output path 

```yaml
root:
  level: info # <info debug warn error>, change logger level here
  appenders:
    - general
#    - stdout  # if not debugging at IDE, please annotation this line.

refresh_rate: 30 seconds

appenders:
  stdout:
    kind: console
    encoder:
      pattern: "[{d(%Y-%m-%d %H:%M:%S)}]|{l}|{M}@{T}<{f}:{L}>: {m}{n}"

  general:
    kind: rolling_file
    path: "logs/general.log" # if start with args to special this file, use absolute path is better.
    encoder:
      pattern: "[{d(%Y-%m-%d %H:%M:%S)}]|{l}|{M}@{T}<{f}:{L}>: {m}{n}"
    policy:
      trigger:
        kind: size
        limit: 10 mb
      roller:
        kind: fixed_window
        pattern: 'logs/general.log.{}'
        base: 1
        count: 10

  others:
    kind: rolling_file
      path: "logs/others.log" # if start with args to special this file, use absolute path is better.
      encoder:
        pattern: "[{d(%Y-%m-%d %H:%M:%S)}]|{l}|{M}@{T}<{f}:{L}>: {m}{n}"
      policy:
        trigger:
          kind: size
          limit: 10 mb
        roller:
          kind: fixed_window
          pattern: 'logs/others.log.{}'
          base: 1
          count: 10

loggers:
  tokio_reactor:
    level: info
    appenders:
      - others
    additive: false
  hyper:
    level: info
    appenders:
      - others
    additive: false
```

### start working 
```bash
aliyun-ddns -c config.yaml -l log4rs.yaml
```

or 

run in workspace:  
default:
- aliyun-ddns
- common_conf
    - config.yaml
    - log4rs.yaml
```bash
aliyun-ddns
```

### suggest
Q: why not make this program running with self-looping? 
A: cause program most time will fall into the dead loop or other accident, even it not program's self-issue

So, I recommend this program can be running on OS schedule, like crontab on Linux, timing program on windows.
  
