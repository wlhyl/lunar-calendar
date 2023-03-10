
# 单元测试
**注意：**以下测试在linux中执行。
* 下载瑞士星历表，并编译
```
mkdir /tmp/swe
cd /tmp/swe
wget https://www.astro.com/ftp/swisseph/swe_unix_src_2.10.03.tar.gz
tar xvzf swe_unix_src_2.10.03.tar.gz
cd src
make libswe.so
```

* 下载瑞士星历表文件
```
cd /tmp/swe

wget https://www.astro.com/ftp/swisseph/ephe/semo_18.se1
wget https://www.astro.com/ftp/swisseph/ephe/semom48.se1
wget https://www.astro.com/ftp/swisseph/ephe/sepl_18.se1
wget https://www.astro.com/ftp/swisseph/ephe/seplm48.se1
```
* 运行测试
**注意：**瑞士星历表不支持多线程
```
EPHE_PATH=/tmp/swe RUSTFLAGS=-L/tmp/swe/src LD_LIBRARY_PATH=/tmp/swe/src cargo test   -- --test-threads=1
```

# 使用
在rust项目中添加以下依赖
```
[dependencies]
...
lunar_calendar = { git = "https://github.com/wlhyl/lunar-calendar.git", branch = "rust" }
```