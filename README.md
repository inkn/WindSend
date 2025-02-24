# 风传 WindSend
中文|[English](README-EN.md) 


## 风传是什么

一组应用程序，用于在不同设备之间快速安全的传递剪切板，传输文件或文件夹(支持windows图片与文件剪切板)。



## 为什么选择风传

- 安全 - 所有数据均加密传递(即使是局域网，也有人希望更安全，比如我)
- 简单 - 界面简洁易上手，开源免费无广告，专注于信息传递
- 全面 - 自动与局域网内密钥相同的电脑匹配，切换wifi也不用担心，同时还提供了不在同一局域网内的解决方案
- 省心 - 不用再担心与电脑的连接状态，只要电脑在线手机就能发送
- 快速 - 使用多线程异步传输文件，充分利用带宽。
- 轻量 - 不依赖额外的运行环境，空闲时内存占用不到10M，基本无CPU消耗

![image-20231014225053389](https://raw.githubusercontent.com/doraemonkeys/picture/master/1/202310142251417.png)

## 如何使用

> **注意**：配置阶段需要确保电脑和手机处于同一网络中。



### 下载

github：[Releases · WindSend](https://github.com/doraemonkeys/WindSend/releases)

123云盘：[www.123pan.com](https://www.123pan.com/s/yfFAjv-lymn.html) 提取码:c1at



### PC端

1. 解压 **WindSend-S-XX-amd64-windows.zip** 到任意目录(提供Rust与Go两种实现) 。

2. 双击exe文件运行：

   请点击允许windows网络防火墙，**注意**勾选公用网络(大胆的勾选，所有内容均已加密)。

   ![image-20230621225600846](https://raw.githubusercontent.com/Doraemonkeys/picture/master/1/202306212303629.png)

   可以观察到任务栏系统托盘里出现app的图标，同时生成了默认配置文件到当前目录。

   ![image-20240124202216544](https://raw.githubusercontent.com/doraemonkeys/picture/master/1/202401242022889.png)

   如果你忘记勾选公用网络，请到Windows防火墙手动设置，或者**确保你正在使用专用网络**。

   ![image-20230623220546743](https://raw.githubusercontent.com/Doraemonkeys/picture/master/1/202306232208808.png)

3. 开启快速配对以便让手机能够搜索到(快速配对将在第一次配对成功后自动关闭)。

   ![image-20240124202641303](https://raw.githubusercontent.com/doraemonkeys/picture/master/1/202401242149192.png)

### 移动端

1. 安装APP(一般选 app-armeabi-v8a-release.apk)。
2. 打开APP，点击右下角的加号配置。



3. 电脑开启快速配对后，手机多点几次搜索，如果幸运的话，你将能看到Secretkey被自动填充。

   <img src="https://raw.githubusercontent.com/doraemonkeys/picture/master/1/202401242149664.png" alt="image-20240124203042150" style="zoom:50%;" />

4. 最后，激动人心的时刻到了，手机随便复制一段文字，打开app点击粘贴，电脑瞬间弹出通知，恭喜你已经成功完成了配置，可以愉快的使用了。



### 自动配对失败？手动添加设备密钥

打开默认配置文件`config.yaml`，复制secretKeyHex，手动填入app配置。

<img src="https://raw.githubusercontent.com/Doraemonkeys/picture/master/1/202306212049362.png" alt="image-20230621192929505" style="zoom: 67%;" />

> 大多数情况下，快速配对失败就表示你设备之间的网络无法连通。请使用手机热点，再次尝试。



### 注意事项

1. 一直转圈圈说明你电脑端配置有问题，比如wifi设置为公用网络。
2. 两个设备之间的时间差不能超过5分钟，否则会导致配对失败。



## 小技巧

- **长按上传手机文件夹**

![image-20240124210045160](https://raw.githubusercontent.com/doraemonkeys/picture/master/1/202401242149010.png)



- **快速复制文件夹**

<img src="https://raw.githubusercontent.com/doraemonkeys/picture/master/1/202401242149818.png" alt="image-20240124205814355" style="zoom: 33%;" />







### 不在同一网络的解决方案

#### 1. 使用内网穿透软件

如果是Tailscale，只需要把电脑ip换成Tailscale分配的IP就行了，其他工具自行测试。



#### 2. 使用别人搭好的服务器

本工具内置了一个，只需要新建配置，ip填web，对你没有看错，就是这三个字母。SecretKey 填电脑配置文件中的密钥。使用此功能需要在电脑上手动点击软件，复制到剪切板。

![image-20240124204702833](https://raw.githubusercontent.com/doraemonkeys/picture/master/1/202401242150315.png)



web传递信息的原理是使用了 https://ko0.com/ 网站。





## Rust实现与Go实现有哪些区别？

这两个版本在在功能与外观上几乎没有区别，但某些方面还是有细微差别。


1. Rust版体积相对稍小
2. Rust版支持将更多种类图片写入Windows剪切板
3. Rust版通知不能显示图标
4. Rust版在传输速度上比Go版略低



## 跨平台情况

目前仅支持Android与Windows。



### 移动端

移动端代码使用flutter编写，理论上支持安卓和IOS，但由于作者对IOS不熟悉，也没有相关设备测试，所以暂不支持，动手能力强的可以自己尝试编译。



### PC

Pc端代码拥有Go与Rust两种实现，代码中主要的库都是跨平台的，想要提供其他平台的支持，只需稍微修改一下源代码，作者能力尚浅，欢迎高手来PR。











