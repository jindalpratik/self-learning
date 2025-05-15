package com.example.dan_vega_dependancy_injection.controller;

import com.example.dan_vega_dependancy_injection.model.Article;
import com.example.dan_vega_dependancy_injection.repository.ArticleRepository;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.List;

@RestController
@RequestMapping("/api/articles")
public class ArticleController {

    private final ArticleRepository articleRepository;

    public ArticleController(ArticleRepository articleRepository) {
        this.articleRepository = articleRepository;
    }

    @GetMapping
    public List<Article> findAll() {
        return articleRepository.findAll();
    }

    @GetMapping("/{id}")
    public Article findById(@PathVariable Integer id) {
         return articleRepository.findById(id);
    }

}
