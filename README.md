# golang编写的公历转农历库
## 单元测试
* 测试环境准备
os：Rocky Linux 8
```bash
yum --enablerepo=powertools install glibc-static
```
* 下载瑞士星历表，并编译
```bash
mkdir /tmp/swe
cd /tmp/swe
wget https://www.astro.com/ftp/swisseph/swe_unix_src_2.10.02.tar.gz
tar xvzf swe_unix_src_2.10.02.tar.gz 
cd swe
make libswe.a
```
* 下载瑞士星历表文件
```bash
wget https://www.astro.com/ftp/swisseph/ephe/semo_18.se1
wget https://www.astro.com/ftp/swisseph/ephe/semom48.se1
wget https://www.astro.com/ftp/swisseph/ephe/sepl_18.se1
wget https://www.astro.com/ftp/swisseph/ephe/seplm48.se1
```

* 单元测试
使用静态编译，以免测试时提示找不到libsw.so
```bash
EPHE_PATH=`pwd`  CGO_LDFLAGS="-L/tmp/swe/src -lswe -lm -ldl -static" go test
```