# himawari-8-wallpaper

苏轼在诗句“人有悲欢离合，月有阴晴圆缺”中描述了月相变化，如果他当时能在人造卫星上旅游一遭，看到地球的阴晴圆缺会发出怎样的感慨呢？

下载地址:

[1.0.0 Android](https://raw.githubusercontent.com/planet0104/himawari-8-wallpaper/master/himawari-8-wallpaper.apk)  
[1.0.0 Windows 64位](https://raw.githubusercontent.com/planet0104/himawari-8-wallpaper/master/himawari-8-wallpaper.exe)  
[1.0.0 Linux 64位](https://raw.githubusercontent.com/planet0104/himawari-8-wallpaper/master/himawari-8-wallpaper)  

Android运行截图如下：

![上午4点](https://github.com/planet0104/himawari-8-wallpaper/blob/master/screenshot/am4.png)  
![上午6点](https://github.com/planet0104/himawari-8-wallpaper/blob/master/screenshot/am6.png)  
![上午8点](https://github.com/planet0104/himawari-8-wallpaper/blob/master/screenshot/am8.png)  
![中午12点](https://github.com/planet0104/himawari-8-wallpaper/blob/master/screenshot/pm12.png)  
![下午3点](https://github.com/planet0104/himawari-8-wallpaper/blob/master/screenshot/pm15.png)  
![傍晚6点](https://github.com/planet0104/himawari-8-wallpaper/blob/master/screenshot/pm18.png)  
![晚上8点](https://github.com/planet0104/himawari-8-wallpaper/blob/master/screenshot/pm20.png)  


Himawari 8是日本的气象卫星，[卫星官网](http://himawari8.nict.go.jp/himawari8-image.htm)的图片每隔10分钟更新一次。himawari-8-wallpaper可自动下载卫星照片并设置为桌面壁纸。


从网站获取图片时，大概只能取到20分钟之前的图片。下载图片的url格式如下：  

```Rust
let url = format!("http://himawari8-dl.nict.go.jp/himawari8/img/D531106/{}d/550/{}/{:02}/{:02}/{:02}{}000_{}_{}.png", d, year, month, day, hour, ten_minute/10, x, y);
```

每次请求返回的图片大小为550x550像素，url示例：  
(以下示例都是取的世界标准时间UTC2019-02-27 14:00分的图片，140000代表14点00分, 141000代表14点10分)  
http://himawari8-dl.nict.go.jp/himawari8/img/D531106/1d/550/2019/02/27/140000_0_0.png 取1x1整幅图  
http://himawari8-dl.nict.go.jp/himawari8/img/D531106/2d/550/2019/02/27/140000_0_0.png 取2x2的左上角  
http://himawari8-dl.nict.go.jp/himawari8/img/D531106/2d/550/2019/02/27/140000_1_0.png 取2x2的右上角  
  
代码中根据不同屏幕分辨率，分别下载2x2和4x4的图片到临时文件夹，然后拼接为黑色背景的地球照片，并将其设置为桌面背景。  
由于服务器在日本，国内访问速度比较慢，桌面分辨率小于1920x1200时，全幅图都是下载2x2图(分辨率1100x1100)，速度较快，半幅图时需要下载4x4的图(2200x2200)，速度较慢，请耐心等待。  

Windows版本运行以后，在系统托盘区域点击图标进行设置![图片地址](https://github.com/planet0104/himawari-8-wallpaper/blob/master/desktop/icon.png)  
Linux版本运行：  

```Bash
./himawari-8-wallpaper [freq10|freq20|freq30|freq60] [mode0|mode1]
```