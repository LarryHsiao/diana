package com.larryhsiao.diana.bookings;

/**
 * A booking record.
 */
public interface Booking {
    long id();
    long start();
    long end();
    long userId();
}
