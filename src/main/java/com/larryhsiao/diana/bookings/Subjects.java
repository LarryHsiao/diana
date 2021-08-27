package com.larryhsiao.diana.bookings;

/**
 * The subjects.
 */
public interface Subjects {
    Subject create(String name);

    void delete(long subjectId);

    Subject byId(long subjectId);

    // @todo #2 update
}
