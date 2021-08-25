package com.larryhsiao.diana;

import java.util.List;

/**
 * The Booking records
 */
public interface Bookings {
    /**
     * Fetch booking record by subject id.
     */
    List<Booking> bySubject(long subjectId);

    Booking newBooking(long subjectId, long start, long end);
}
