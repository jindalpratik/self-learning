package com.example.dan_vega_dependancy_injection.service;

import org.springframework.stereotype.Service;

@Service
public class SimpleSlugService implements SlugService{

    @Override
    public String slugify(String input) {
        return input
                .toLowerCase()
                .replaceAll("[^a-zA-Z0-9]", " ")
                .replaceAll(" ", "-");
    }

}
