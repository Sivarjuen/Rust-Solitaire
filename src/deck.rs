#![allow(dead_code)]

use bevy::prelude::*;
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

use crate::card::{Card, Rank, Suit};

#[derive(Resource, Debug)]
pub struct Deck {
    cards: Vec<Card>,
    drawn: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut deck = Deck {
            cards: Deck::populate(),
            drawn: vec![],
        };
        deck.shuffle();
        deck
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::rng());
    }

    pub fn get_cards(&self) -> &Vec<Card> {
        &self.cards
    }

    fn populate() -> Vec<Card> {
        let mut cards = Vec::new();
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                cards.push(Card {
                    rank: rank.clone(),
                    suit: suit.clone(),
                    flipped: true,
                })
            }
        }
        cards
    }

    pub fn play(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn play_drawn(&mut self) -> Option<Card> {
        self.drawn.pop()
    }

    pub fn draw(&mut self) -> Option<Card> {
        let card = self.cards.pop();
        if let Some(card) = card.as_ref() {
            self.drawn.push(card.clone());
        }
        card
    }

    pub fn reset(&mut self) {
        self.cards = self.drawn.clone();
        self.cards.reverse();
        self.drawn.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}

impl Default for Deck {
    fn default() -> Self {
        Deck::new()
    }
}
