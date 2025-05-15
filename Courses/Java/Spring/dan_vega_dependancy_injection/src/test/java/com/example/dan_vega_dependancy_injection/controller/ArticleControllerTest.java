package com.example.dan_vega_dependancy_injection.controller;

import com.example.dan_vega_dependancy_injection.model.Article;
import com.example.dan_vega_dependancy_injection.repository.ArticleRepository;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.Mockito;
import org.springframework.boot.test.autoconfigure.web.servlet.WebMvcTest;
import org.springframework.test.context.bean.override.mockito.MockitoBean;

import java.time.LocalDateTime;
import java.util.ArrayList;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

@WebMvcTest(ArticleController.class)
class ArticleControllerTest {

    ArticleController controller;

    @MockitoBean
    ArticleRepository articleRepository;

    List<Article> articles = new ArrayList<>();

    @BeforeEach
    void setUp() {
        controller = new ArticleController(articleRepository);
        articles = List.of(
                new Article(1, "Getting Started with Spring Boot",
                        "Getting Started with Spring Boot",
                        "Spring Boot makes it easy to create stand-alone, production-grade Spring based Applications. " +
                                "This article covers the basics of setting up your first Spring Boot application.",
                        LocalDateTime.of(2023, 1, 15, 10, 30)),

                new Article(2, "Understanding Java Records",
                        "Understanding Java Records",
                        "Java Records provide a compact syntax for declaring classes that are transparent holders for shallowly immutable data. " +
                                "Learn how to use this powerful feature introduced in Java 14.",
                        LocalDateTime.of(2023, 2, 22, 14, 15)),

                new Article(3, "REST API Best Practices",
                        "REST API Best Practices",
                        "Building RESTful APIs requires following certain conventions and best practices. " +
                                "This guide walks through the key principles for designing clean and effective REST APIs.",
                        LocalDateTime.of(2023, 3, 10, 9, 45)),

                new Article(4, "Introduction to Spring Data JPA",
                        "Introduction to Spring Data JPA",
                        "Spring Data JPA simplifies data access layer implementation by reducing boilerplate code. " +
                                "This tutorial demonstrates how to leverage repositories and entity mappings effectively.",
                        LocalDateTime.of(2023, 4, 5, 16, 20)),

                new Article(5, "Securing Spring Boot Applications",
                        "Securing Spring Boot Applications",
                        "Security is crucial for modern web applications. Learn how to implement authentication and " +
                                "authorization in your Spring Boot application using Spring Security.",
                        LocalDateTime.of(2023, 5, 18, 11, 0))
        );
    }

    @Test
    void shouldReturnAllArticles() {
        Mockito.when(articleRepository.findAll()).thenReturn(articles);
        assertEquals(5, controller.findAll().spliterator().getExactSizeIfKnown());
    }

    @Test
    void shouldReturnArticleWithValidId() {
        Mockito.when(articleRepository.findById(1)).thenReturn(articles.getFirst());
        Article article = controller.findById(1);
        assertNotNull(article);
    }

}