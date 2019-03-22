package io.github.planet0104.h8w;

import android.Manifest;
import android.app.AlarmManager;
import android.app.AlertDialog;
import android.app.PendingIntent;
import android.app.Service;
import android.app.WallpaperManager;
import android.content.Context;
import android.content.DialogInterface;
import android.content.Intent;
import android.content.SharedPreferences;
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
import android.widget.AdapterView;
import android.widget.ArrayAdapter;
import android.widget.CheckBox;
import android.widget.CompoundButton;
import android.widget.RadioButton;
import android.widget.RadioGroup;
import android.widget.Spinner;
import android.widget.TimePicker;
import android.widget.Toast;

import java.io.ByteArrayInputStream;
import java.io.File;
import java.io.FileNotFoundException;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.RandomAccessFile;
import java.util.Date;

import static android.Manifest.permission.SET_WALLPAPER;
public class MainActivity extends AppCompatActivity implements CompoundButton.OnCheckedChangeListener, RadioGroup.OnCheckedChangeListener, AdapterView.OnItemSelectedListener {
    final static int REQ_SET_WALLPAPER = 110;
    final static int REQ_SAVE_WALLPAPER = 111;

    static final String SET_SAVE = "save";                  //是否保存到相册 boolean
    static final String SET_HALF = "half";                  //下载整张图还是半张图 boolean
    static final String SET_AUTO_UPDATE = "auto";           //是否自动更新 boolean
    static final String SET_UPDATE_INTERVAL = "interval";   //更新间隔 number
    static final String SET_DISABLE_PERIOD = "disable";     //禁止在某时间段不更新 boolean
    static final String SET_DISABLE_PERIOD_TIME = "disable-time"; //不更新时间段 [20,6]
    static final String SET_LAST_UPDATE_TIME = "last_update_time"; //上次更新时间 long

    private CheckBox chk_save_to_album;
    private RadioGroup rg_interval;
    private RadioButton cb_delay_10;
    private RadioButton cb_delay_20;
    private RadioButton cb_delay_30;
    private RadioButton cb_delay_60;
    private CheckBox cb_disable;
    private Spinner sp_start;
    private Spinner sp_end;

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
            if(PrefHelper.getBooleanVal(SET_SAVE)){
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

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        CheckBox cb_auto_update = findViewById(R.id.cb_auto_update);
        chk_save_to_album = findViewById(R.id.chk_save_to_album);
        RadioGroup rg_type = findViewById(R.id.rg_type);

        rg_interval = findViewById(R.id.rg_interval);
        rg_interval.setOnCheckedChangeListener(new RadioGroup.OnCheckedChangeListener() {
            @Override
            public void onCheckedChanged(RadioGroup group, int checkedId) {
                switch(rg_interval.getCheckedRadioButtonId()){
                    case R.id.cb_delay_10: updateInterval(10); break;
                    case R.id.cb_delay_20: updateInterval(20); break;
                    case R.id.cb_delay_30: updateInterval(30); break;
                    case R.id.cb_delay_60: updateInterval(60); break;
                }
            }
        });


        cb_disable = findViewById(R.id.cb_disable);
        cb_disable.setOnCheckedChangeListener(this);
        cb_delay_10 = findViewById(R.id.cb_delay_10);
        cb_delay_20 = findViewById(R.id.cb_delay_20);
        cb_delay_30 = findViewById(R.id.cb_delay_30);
        cb_delay_60 = findViewById(R.id.cb_delay_60);
        sp_start = findViewById(R.id.sp_start);
        sp_end = findViewById(R.id.sp_end);
        sp_start.setOnItemSelectedListener(this);
        sp_end.setOnItemSelectedListener(this);

        cb_auto_update.setChecked(PrefHelper.getBooleanVal(SET_AUTO_UPDATE));
        setAutoUpdate(cb_auto_update.isChecked());

        chk_save_to_album.setOnCheckedChangeListener(this);
        cb_auto_update.setOnCheckedChangeListener(this);
        chk_save_to_album.setChecked(PrefHelper.getBooleanVal(SET_SAVE));

        rg_type.setOnCheckedChangeListener(this);

        if(PrefHelper.getBooleanVal(SET_HALF)){
            rg_type.check(R.id.rb_half);
        }else{
            rg_type.check(R.id.rb_full);
        }


        //--------- 不更新时间段设置 ----------------
        String[] hours = new String[24];
        for(int i=0;i<24; i++){
            hours[i] = i+"时";
        }
        ArrayAdapter adapter = new ArrayAdapter<>(this, android.R.layout.simple_list_item_1, hours);
        sp_start.setAdapter(adapter);
        sp_end.setAdapter(adapter);
        int[] t = getPeriodTime();
        sp_start.setSelection(t[0]);
        sp_end.setSelection(t[1]);

        //------------ 更新时间间隔 -------------
        int minutes = PrefHelper.getIntVal(SET_UPDATE_INTERVAL);
        if(minutes==0){
            minutes = 60;
            PrefHelper.setVal(SET_UPDATE_INTERVAL, 60);
        }
        switch (minutes){
            case 10: rg_interval.check(R.id.cb_delay_10); break;
            case 20: rg_interval.check(R.id.cb_delay_20); break;
            case 30: rg_interval.check(R.id.cb_delay_30); break;
            case 60: rg_interval.check(R.id.cb_delay_60); break;
        }
    }

    private void startService(){
        if(!MyApplication.serviceRunning){
            Intent serviceIntent = new Intent(this, WallpaperService.class);
            startService(serviceIntent);
//            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
//                startForegroundService(serviceIntent);
//            } else {
//                startService(serviceIntent);
//            }
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
                PrefHelper.setVal(SET_SAVE, true);
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

    private void setAutoUpdate(boolean isChecked){
        cb_delay_10.setEnabled(isChecked);
        cb_delay_20.setEnabled(isChecked);
        cb_delay_30.setEnabled(isChecked);
        cb_delay_60.setEnabled(isChecked);
        cb_disable.setEnabled(isChecked);
        setDisablePeriod(PrefHelper.getBooleanVal(SET_DISABLE_PERIOD));

        PrefHelper.setVal(SET_AUTO_UPDATE, isChecked);

        AlarmManager alarmManager = (AlarmManager) getSystemService(Service.ALARM_SERVICE);
        Intent intent = new Intent(this, WallpaperService.class);
        final PendingIntent pi = PendingIntent.getService(this, 0, intent, 0);
        if(isChecked){
            int interval = PrefHelper.getIntVal(SET_UPDATE_INTERVAL);
            //获取上次更新时间
            Date lastUpdateTime = null;
            long val = PrefHelper.getLongVal(SET_LAST_UPDATE_TIME);
            if(val>0){
                lastUpdateTime = new Date(val);
            }

            //如果没有更新过也没有正在更新，立即更新壁纸
            if(!MyApplication.serviceRunning && lastUpdateTime==null){
                alarmManager.setRepeating(AlarmManager.ELAPSED_REALTIME, 0,interval*60*1000, pi);
            }

        }else{
            sp_end.setEnabled(false);
            sp_start.setEnabled(false);
        }
    }

    /**
     * 获取不更新时间区间
     * @return
     */
    private int[] getPeriodTime(){
        int[] arr = new int[]{20, 6};
        String t = PrefHelper.getStringVal(SET_DISABLE_PERIOD_TIME);
        if(t != null && t.length()>0){
            String[] a = t.split("-");
            arr[0] = Integer.valueOf(a[0]);
            arr[1] = Integer.valueOf(a[1]);
        }
        return arr;
    }

    /**
     * 设置不更新时间段
     * @param start
     * @param end
     */
    private void setPeriodTime(int start, int end){
        PrefHelper.setVal(SET_DISABLE_PERIOD_TIME, start+"-"+end);
    }

    /**
     *
     * @param disable true: 禁止在某区间不更新 false:允许在指定时间段不更新
     */
    private void setDisablePeriod(boolean disable){
        cb_disable.setChecked(!disable);
        sp_start.setEnabled(!disable);
        sp_end.setEnabled(!disable);
        PrefHelper.setVal(SET_DISABLE_PERIOD, disable);
    }

    //更新定时器时间间隔
    private void updateInterval(int minutes){
        PrefHelper.setVal(SET_UPDATE_INTERVAL, minutes);
    }

    @Override
    public void onCheckedChanged(CompoundButton buttonView, boolean isChecked) {
        if(buttonView.getId() == R.id.cb_disable){
            setDisablePeriod(!isChecked);
        }else if(buttonView.getId() == R.id.cb_auto_update){
            setAutoUpdate(isChecked);
        }else if(buttonView.getId() == R.id.chk_save_to_album){
            if(isChecked){
                //检查权限
                if(Build.VERSION.SDK_INT>=23) {
                    int hasWriteStoragePermission = ContextCompat.checkSelfPermission(this, Manifest.permission.WRITE_EXTERNAL_STORAGE);
                    if (hasWriteStoragePermission != PackageManager.PERMISSION_GRANTED){
                        ActivityCompat.requestPermissions(this, new String[]{Manifest.permission.WRITE_EXTERNAL_STORAGE}, REQ_SAVE_WALLPAPER);
                    }else{
                        PrefHelper.setVal(SET_SAVE, true);
                    }
                }else{
                    PrefHelper.setVal(SET_SAVE, true);
                }
            }else{
                PrefHelper.setVal(SET_SAVE, false);
            }
        }
    }

    @Override
    public void onCheckedChanged(RadioGroup group, int checkedId) {
        if(checkedId == R.id.rb_full){
            PrefHelper.setVal(SET_HALF, false);
        }else{
            PrefHelper.setVal(SET_HALF, true);
        }
    }

    @Override
    public void onItemSelected(AdapterView<?> parent, View view, int position, long id) {
        switch (parent.getId()){
            case R.id.sp_start:
                setPeriodTime(position, sp_end.getSelectedItemPosition());
                break;
            case R.id.sp_end:
                setPeriodTime(sp_start.getSelectedItemPosition(), position);
                break;
        }
    }

    @Override
    public void onNothingSelected(AdapterView<?> parent) {

    }
}
