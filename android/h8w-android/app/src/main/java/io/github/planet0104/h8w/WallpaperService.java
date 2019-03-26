package io.github.planet0104.h8w;

import android.app.Notification;
import android.app.NotificationChannel;
import android.app.NotificationManager;
import android.app.Service;
import android.content.BroadcastReceiver;
import android.content.Context;
import android.content.Intent;
import android.content.IntentFilter;
import android.os.Build;
import android.os.Handler;
import android.os.IBinder;
import android.os.Message;
import android.support.annotation.NonNull;
import android.support.annotation.Nullable;
import android.support.v4.app.NotificationCompat;
import android.util.Log;

import java.util.Calendar;
import java.util.Date;

import static io.github.planet0104.h8w.MainActivity.SET_HALF;
import static io.github.planet0104.h8w.MainActivity.SET_LAST_UPDATE_TIME;
import static io.github.planet0104.h8w.MainActivity.downloadAndSetWallpaper;
import static io.github.planet0104.h8w.MainActivity.getPeriodTime;

//https://developer.android.google.cn/guide/topics/ui/notifiers/notifications.html
//https://www.jianshu.com/p/b83fc1697232

public class WallpaperService extends Service implements Runnable, Handler.Callback {
    static final String TAG = WallpaperService.class.getSimpleName();

    NotificationManager mNotificationManager;
    Handler mHandler;
    final int NOTIFY_ID = 2019;
    boolean valid = true;

    private NotificationManager getNotificationManager() {
        if (mNotificationManager == null) {
            mNotificationManager = (NotificationManager) getSystemService(Context.NOTIFICATION_SERVICE);
        }
        return mNotificationManager;
    }

    @NonNull
    private NotificationCompat.Builder getNotificationBuilder(String content) {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            NotificationChannel channel = new NotificationChannel(getPackageName(), TAG,
                    NotificationManager.IMPORTANCE_LOW);
            //是否绕过请勿打扰模式
//            channel.canBypassDnd();
            //闪光灯
            channel.enableLights(false);
            //锁屏显示通知
//            channel.setLockscreenVisibility(VISIBILITY_SECRET);
            //闪关灯的灯光颜色
//            channel.setLightColor(Color.RED);
            //桌面launcher的消息角标
//            channel.canShowBadge();
            //是否允许震动
            channel.enableVibration(false);
            //获取系统通知响铃声音的配置
//            channel.getAudioAttributes();
            //获取通知取到组
            channel.getGroup();
            //设置可绕过  请勿打扰模式
//            channel.setBypassDnd(true);
            //设置震动模式
//            channel.setVibrationPattern(new long[]{100, 100, 200});
            //是否会有灯光
//            channel.shouldShowLights();
            getNotificationManager().createNotificationChannel(channel);
        }
        NotificationCompat.Builder notification = new NotificationCompat.Builder(this, "channel_id");
        notification.setContentTitle("卫星壁纸");
        notification.setContentText(content);
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N) {
            notification.setCategory(Notification.CATEGORY_SERVICE);
        }
        notification.setPriority(Notification.PRIORITY_MIN);
        notification.setOngoing(true);
        notification.setSmallIcon(R.mipmap.ic_launcher);
        notification.setChannelId(getPackageName());
        notification.setAutoCancel(true);
        return notification;
    }

    private BroadcastReceiver receiver = new BroadcastReceiver() {
        @Override
        public void onReceive(Context context, Intent intent) {
            int current = intent.getIntExtra("current", 0);
            int total = intent.getIntExtra("total", 0);
            Log.d(TAG, "下载进度监听:"+current+"/"+total);
            getNotificationManager().notify(TAG,NOTIFY_ID, getNotificationBuilder("正在下载壁纸"+current+"/"+total).build());
        }
    };

    @Nullable
    @Override
    public IBinder onBind(Intent intent) {
        return null;
    }

    @Override
    public void onCreate() {
        super.onCreate();
        Log.d(TAG, "WallpaperService onCreate()");

        int hour = Calendar.getInstance().get(Calendar.HOUR_OF_DAY);
        int[] periodTime = getPeriodTime();
        if(hour>periodTime[0] || hour<periodTime[1]){
            valid = false;
        }
        if(valid){
            mHandler = new Handler(this);
            registerReceiver(receiver, new IntentFilter("progress"));
            getNotificationManager().notify(TAG,NOTIFY_ID, getNotificationBuilder("正在更新壁纸").build());
        }
    }

    @Override
    public void onDestroy() {
        super.onDestroy();
        Log.d(TAG, "WallpaperService onDestroy()");
        if(valid) {
            unregisterReceiver(receiver);
            MyApplication.serviceRunning = false;
        }
    }

    @Override
    public int onStartCommand(Intent intent, int flags, int startId) {
        if(valid) {
            Log.d(TAG, "WallpaperService onStartCommand()");
            if(!MyApplication.serviceRunning){
                MyApplication.serviceRunning = true;
                new Thread(this).start();
            }
        }
        return super.onStartCommand(intent, flags, startId);
    }

    @Override
    public void run() {
        if(valid) {
            Log.d(TAG, "WallpaperService run()");
            int type = PrefHelper.getBooleanVal(SET_HALF)?1:0;
            if(downloadAndSetWallpaper(type)){
                Log.i(TAG, "壁纸设置成功.");
                PrefHelper.setVal(SET_LAST_UPDATE_TIME, new Date().getTime());
            }else{
                Log.e(TAG, "壁纸设置失败!");
            }
            mHandler.sendEmptyMessage(0);
        }
    }

    @Override
    public boolean handleMessage(Message message) {
        if(valid) {
            Log.d(TAG, "WallpaperService handleMessage()");
            getNotificationManager().notify(TAG,NOTIFY_ID, getNotificationBuilder("壁纸更新完成").build());
            mHandler.postDelayed(new Runnable() {
                @Override
                public void run() {
                    //删除通知
                    getNotificationManager().cancel(NOTIFY_ID);
                    getNotificationManager().cancelAll();
                    stopSelf();
                }
            }, 1000);
        }
        return true;
    }
}
