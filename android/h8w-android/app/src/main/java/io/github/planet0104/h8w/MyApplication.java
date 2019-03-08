package io.github.planet0104.h8w;

import android.app.Application;
import android.content.Context;

public class MyApplication extends Application {

    private static Context context;
    public static boolean serviceRunning = false;

    public void onCreate() {
        super.onCreate();
        MyApplication.context = getApplicationContext();
        PrefHelper.initHelper(this, "wallpaper");
    }

    public static Context getAppContext() {
        return MyApplication.context;
    }
}