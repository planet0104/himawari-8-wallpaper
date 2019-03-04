package io.github.planet0104.h8w;

import android.Manifest;
import android.app.AlertDialog;
import android.app.WallpaperManager;
import android.content.DialogInterface;
import android.content.pm.PackageManager;
import android.graphics.Bitmap;
import android.graphics.BitmapFactory;
import android.os.Build;
import android.os.Bundle;
import android.support.annotation.RequiresPermission;
import android.support.v4.app.ActivityCompat;
import android.support.v4.content.ContextCompat;
import android.support.v7.app.AppCompatActivity;
import android.util.Log;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.RandomAccessFile;

import static android.Manifest.permission.SET_WALLPAPER;

public class MainActivity extends AppCompatActivity {
    final static int REQ_SET_WALLPAPER = 110;
    //https://blog.csdn.net/benpao00/article/details/52538052

    //https://www.jb51.net/article/133638.htm

    // 在BroadcastReceiver中启动Service下载壁纸,  onReceive必须在10s中结束

    static {
        System.loadLibrary("wallpaper");
    }
    static final String TAG = MainActivity.class.getSimpleName();
    native void init();

    @SuppressWarnings("unused")
    public static int getScreenWidth(){
        Log.d(TAG, "屏幕宽度:"+MyApplication.getAppContext().getResources().getDisplayMetrics().widthPixels);
        return MyApplication.getAppContext().getResources().getDisplayMetrics().widthPixels;
    }

    @SuppressWarnings("unused")
    public static int getScreenHeight(){
        Log.d(TAG, "屏幕高度:"+MyApplication.getAppContext().getResources().getDisplayMetrics().heightPixels);
        return MyApplication.getAppContext().getResources().getDisplayMetrics().heightPixels;
    }

    @SuppressWarnings("unused")
    public static byte[] openFile(String name){
        Log.d(TAG, "读取文件:"+name);
        try{
            RandomAccessFile f = new RandomAccessFile(new File(MyApplication.getAppContext().getFilesDir(), name), "r");
            byte[] b = new byte[(int)f.length()];
            f.readFully(b);
            return b;
        }catch (Exception e){
            return null;
        }
    }

    @SuppressWarnings("unused")
    public static String saveFile(String name, byte[] png){
        Log.d(TAG, "保存文件:"+name+" len="+png.length);
        try{
            FileOutputStream fos = new FileOutputStream(new File(MyApplication.getAppContext().getFilesDir(), name));
            fos.write(png);
            fos.flush();
            fos.close();
            return "OK";
        }catch (Exception e){
            return e.getMessage();
        }
    }

    @SuppressWarnings("unused")
    @RequiresPermission(SET_WALLPAPER)
    public static String setWallpaper(byte[] png){
        Log.i(TAG, "设置壁纸: png大小:"+png.length);
        try {
            Bitmap bmp = BitmapFactory.decodeByteArray(png, 0, png.length);
            WallpaperManager.getInstance(MyApplication.getAppContext()).setBitmap(bmp);
//            if(Build.VERSION.SDK_INT>=24) {
//                WallpaperManager.getInstance(MyApplication.getAppContext()).setBitmap(bmp, null, true, WallpaperManager.FLAG_LOCK);
//            }
            return "OK";
        } catch (IOException e) {
            e.printStackTrace();
            return e.getMessage();
        }
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        if(Build.VERSION.SDK_INT>=23){
            int hasWriteStoragePermission = ContextCompat.checkSelfPermission(this, Manifest.permission.SET_WALLPAPER);
            if (hasWriteStoragePermission == PackageManager.PERMISSION_GRANTED) {
                init();
            }else{
                ActivityCompat.requestPermissions(this, new String[]{Manifest.permission.SET_WALLPAPER}, REQ_SET_WALLPAPER);
            }
        }else{
            init();
        }
    }

    @Override
    public void onRequestPermissionsResult(int requestCode, String[] permissions, int[] grantResults) {
        //通过requestCode来识别是否同一个请求
        if (requestCode == REQ_SET_WALLPAPER){
            if (grantResults.length > 0 && grantResults[0] == PackageManager.PERMISSION_GRANTED){
                init();
            }else{
                //用户不同意，向用户展示该权限作用
                if (ActivityCompat.shouldShowRequestPermissionRationale(this, Manifest.permission.SET_WALLPAPER)) {
                    new AlertDialog.Builder(this)
                            .setMessage("请允许程序设置手机壁纸")
                            .setPositiveButton("确定", new DialogInterface.OnClickListener() {
                                @Override
                                public void onClick(DialogInterface dialog, int which) {
                                    ActivityCompat.requestPermissions(MainActivity.this,
                                            new String[]{Manifest.permission.SET_WALLPAPER},
                                            REQ_SET_WALLPAPER);
                                }
                            })
                            .setNegativeButton("取消", null)
                            .create()
                            .show();
                }
            }
        }
    }
}
