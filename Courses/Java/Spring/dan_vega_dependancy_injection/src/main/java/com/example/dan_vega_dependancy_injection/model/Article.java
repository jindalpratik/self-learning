package com.example.dan_vega_dependancy_injection.model;

import java.time.LocalDateTime;

public record Article(
        Integer id,
        String title,
        String slug,
        String content,
        LocalDateTime publishedOn
) {
}
