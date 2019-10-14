# aliyun-ddns for rust

Due to aliyun-ddns sdk not provide rust version, this is written by rust which follows aliyun docs.

## how to use
抱歉来迟了，补充了篇中文的配置文档:
[aliyun-ddns设置定时监控更新](https://pace.eternaless.com/2019/10/14/aliyun-ddns%E8%AE%BE%E7%BD%AE%E5%AE%9A%E6%97%B6%E7%9B%91%E6%8E%A7%E6%9B%B4%E6%96%B0/)

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
  level: debug # <info debug warn error>, change logger level here
  appenders:
    - general
    - stdout

refresh_rate: 30 seconds

appenders:
  stdout:
    kind: console

  general:
    kind: file
    path: "logs/general.log" # if start with args to special this file, use absolute path is better.
    encoder:
      pattern: "{d} [{t}] {l} {M}:{m}{n}"

  others:
    kind: file
    path: "logs/others.log" # if start with args to special this file, use absolute path is better.
    encoder:
      pattern: "{d} [{t}] {l} {M}:{m}{n}"

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
  
