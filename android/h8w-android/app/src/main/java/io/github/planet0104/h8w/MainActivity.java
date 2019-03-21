package io.github.planet0104.h8w;

import android.Manifest;
import android.app.AlertDialog;
import android.app.WallpaperManager;
import android.content.Context;
import android.content.DialogInterface;
import android.content.Intent;
import android.content.pm.PackageManager;
import android.graphics.Bitmap;
import android.graphics.BitmapFactory;
import android.net.Uri;
import android.os.Build;
import android.os.Bundle;
import android.os.Environment;
import android.support.annotation.RequiresPermission;
import android.support.v4.app.ActivityCompat;
import android.support.v4.content.ContextCompat;
import android.support.v7.app.AppCompatActivity;
import android.util.Log;
import android.view.View;
import android.widget.CheckBox;
import android.widget.CompoundButton;
import android.widget.RadioGroup;
import android.widget.Toast;

import java.io.ByteArrayInputStream;
import java.io.File;
import java.io.FileNotFoundException;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.RandomAccessFile;

import static android.Manifest.permission.SET_WALLPAPER;
public class MainActivity extends AppCompatActivity implements CompoundButton.OnCheckedChangeListener, RadioGroup.OnCheckedChangeListener {
    final static int REQ_SET_WALLPAPER = 110;
    final static int REQ_SAVE_WALLPAPER = 111;

    //https://www.jb51.net/article/133638.htm

    // 在BroadcastReceiver中启动Service下载壁纸,  onReceive必须在10s中结束


    //https://blog.csdn.net/ting_ting_liu/article/details/73859649 Android 壁纸设置 总结

    //https://blog.csdn.net/heng615975867/article/details/18983317

    // service设置壁纸
    //http://www.androidchina.net/2972.html

    static {
        System.loadLibrary("wallpaper");
    }
    static final String TAG = MainActivity.class.getSimpleName();

    /**
     * 下载并设置壁纸
     * @param type 0整张,1半张
     * @return
     */
    public static native boolean downloadAndSetWallpaper(int type);

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
    public static void notifyDownloadProgress(int current, int total){
        Log.d(TAG, "下载进度监听:"+current+"/"+total);
        Intent intent = new Intent("progress");
        intent.putExtra("current", current);
        intent.putExtra("total", total);
        MyApplication.getAppContext().sendBroadcast(intent);
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
            if(Build.VERSION.SDK_INT >= Build.VERSION_CODES.N){
                WallpaperManager.getInstance(MyApplication.getAppContext()).setStream(new ByteArrayInputStream(png), null, true, WallpaperManager.FLAG_SYSTEM | WallpaperManager.FLAG_LOCK);
            }else{
                Bitmap bmp = BitmapFactory.decodeByteArray(png, 0, png.length);
                WallpaperManager.getInstance(MyApplication.getAppContext()).setBitmap(bmp);
            }
            if(PrefHelper.getBooleanVal("save")){
                //保存图片文件并更新到图库
                saveWallpaperToAlbum(png, "himawari-8-wallpaper.jpg");
            }
            return "OK";
        } catch (IOException e) {
            e.printStackTrace();
            return e.getMessage();
        }
    }

    /*
     * 保存文件
     */
    public static void saveWallpaperToAlbum(byte[] png, String bitName){
        Context context = MyApplication.getAppContext();
        String fileName ;
        File file ;
        if(Build.BRAND .equals("Xiaomi") ){ // 小米手机
            fileName = Environment.getExternalStorageDirectory().getPath()+"/DCIM/Camera/"+bitName ;
        }else{  // Meizu 、Oppo
            fileName = Environment.getExternalStorageDirectory().getPath()+"/DCIM/"+bitName ;
        }
        file = new File(fileName);
        if(file.exists()){
            if(!file.delete()){
                Log.w(TAG, "旧的壁纸照片删除失败:"+fileName);
            }
        }
        try{
            FileOutputStream out = new FileOutputStream(file);
            Bitmap bitmap = BitmapFactory.decodeByteArray(png, 0, png.length);
            // 格式为 JPEG，照相机拍出的图片为JPEG格式的，PNG格式的不能显示在相册中
            if(bitmap.compress(Bitmap.CompressFormat.JPEG, 100, out)){
                out.flush();
                out.close();
            }
        }catch (FileNotFoundException e) {
            e.printStackTrace();
        } catch (IOException e) {
            e.printStackTrace();

        }
        // 发送广播，通知刷新图库的显示
        context.sendBroadcast(new Intent(Intent.ACTION_MEDIA_SCANNER_SCAN_FILE, Uri.parse("file://" + fileName)));
    }

    CheckBox chk_save_to_album;
    RadioGroup rg_type;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        chk_save_to_album = findViewById(R.id.chk_save_to_album);
        rg_type = findViewById(R.id.rg_type);

        chk_save_to_album.setOnCheckedChangeListener(this);
        chk_save_to_album.setChecked(PrefHelper.getBooleanVal("save"));

        rg_type.setOnCheckedChangeListener(this);

        if(PrefHelper.getBooleanVal("half")){
            rg_type.check(R.id.rb_half);
        }else{
            rg_type.check(R.id.rb_full);
        }
    }

    private void startService(){
        if(!MyApplication.serviceRunning){
            Intent serviceIntent = new Intent(this, WallpaperService.class);
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
                startForegroundService(serviceIntent);
            } else {
                startService(serviceIntent);
            }
            Toast.makeText(this, "开始下载壁纸", Toast.LENGTH_SHORT).show();
        }else{
            Toast.makeText(this, "正在下载壁纸", Toast.LENGTH_SHORT).show();
        }
    }

    public void doAction(View v){
        if(Build.VERSION.SDK_INT>=23){
            int hasSetWallpaperPermission = ContextCompat.checkSelfPermission(this, Manifest.permission.SET_WALLPAPER);
            if (hasSetWallpaperPermission ==  PackageManager.PERMISSION_GRANTED) {
                startService();
            }else{
                ActivityCompat.requestPermissions(this, new String[]{Manifest.permission.SET_WALLPAPER}, REQ_SET_WALLPAPER);
            }
        }else{
            startService();
        }
    }

    @Override
    public void onRequestPermissionsResult(int requestCode, String[] permissions, int[] grantResults) {
        if(requestCode == REQ_SAVE_WALLPAPER){
            if (grantResults.length > 0 && grantResults[0] == PackageManager.PERMISSION_GRANTED){
                PrefHelper.setVal("save", true);
            }else{
                chk_save_to_album.setChecked(false);
            }
        }else if (requestCode == REQ_SET_WALLPAPER){
            if (grantResults.length > 0 && grantResults[0] == PackageManager.PERMISSION_GRANTED){
                startService();
            }else{
                AlertDialog dialog = new AlertDialog.Builder(this)
                        .setMessage("请允许程序设置手机壁纸")
                        .setPositiveButton("确定", new DialogInterface.OnClickListener() {
                            @Override
                            public void onClick(DialogInterface dialog, int which) {
                                ActivityCompat.requestPermissions(MainActivity.this,
                                        new String[]{Manifest.permission.SET_WALLPAPER}, REQ_SET_WALLPAPER);
                            }
                        })
                        .setNegativeButton("取消", null)
                        .create();
                dialog.show();
            }
        }
    }

    @Override
    public void onCheckedChanged(CompoundButton buttonView, boolean isChecked) {
        if(isChecked){
            //检查权限
            if(Build.VERSION.SDK_INT>=23) {
                int hasWriteStoragePermission = ContextCompat.checkSelfPermission(this, Manifest.permission.WRITE_EXTERNAL_STORAGE);
                if (hasWriteStoragePermission != PackageManager.PERMISSION_GRANTED){
                    ActivityCompat.requestPermissions(this, new String[]{Manifest.permission.WRITE_EXTERNAL_STORAGE}, REQ_SAVE_WALLPAPER);
                }else{
                    PrefHelper.setVal("save", true);
                }
            }else{
                PrefHelper.setVal("save", true);
            }
        }else{
            PrefHelper.setVal("save", false);
        }
    }

    @Override
    public void onCheckedChanged(RadioGroup group, int checkedId) {
        if(checkedId == R.id.rb_full){
            PrefHelper.setVal("half", false);
        }else{
            PrefHelper.setVal("half", true);
        }
    }
}
