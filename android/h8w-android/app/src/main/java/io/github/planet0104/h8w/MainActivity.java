package io.github.planet0104.h8w;

import android.support.v7.app.AppCompatActivity;
import android.os.Bundle;
import android.util.Log;

public class MainActivity extends AppCompatActivity {
    static {
        System.loadLibrary("wallpaper");
    }
    static final String TAG = MainActivity.class.getSimpleName();
    native void init();

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        Log.d(TAG, "调用native init()");
        init();
        Log.d(TAG, "调用native init()结束");
    }
}
