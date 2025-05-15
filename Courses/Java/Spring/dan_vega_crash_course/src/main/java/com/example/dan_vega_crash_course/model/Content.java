package com.example.dan_vega_crash_course.model;

import java.time.LocalDateTime;

public record Content(
        Integer id,
        String title,
        String desc,
        Status status,
        Type contentType,
        LocalDateTime dateCreated,
        LocalDateTime dateUpdate,
        String url
) {

}
