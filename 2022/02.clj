#!/usr/bin/env bb
(ns aoc
  (:require [clojure.string :as str]))

(def input (slurp "in/02.txt"))

(def strategy-guide (->> input str/split-lines))

(defn points-a
  "Calculate points for part 1, where the strategy guide contains both shapes."
  [line]
  (case line
    "A X" 4  ;; Rock vs Rock: Draw (3) + Rock (1)
    "A Y" 8  ;; Rock vs Paper: Win (6) + Paper (2)
    "A Z" 3  ;; Rock vs Scissors: Loss (0) + Scissors (3)
    "B X" 1  ;; Paper vs Rock: Loss (0) + Rock (1)
    "B Y" 5  ;; Paper vs Paper: Draw (3) + Paper (2)
    "B Z" 9  ;; Paper vs Scissors: Win (6) + Scissors (3)
    "C X" 7  ;; Scissors vs Rock: Win (6) + Rock (1)
    "C Y" 2  ;; Scissors vs Paper: Loss (0) + Paper (2)
    "C Z" 6)) ;; Scissors vs Scissors: Draw (3) + Scissors (3)

(defn points-b
  "Calculate points for part 2, where the strategy guide contains the opponent's shape and the desired outcome."
  [line]
  (case line
    "A X" 3  ;; Rock, Loss: Loss (0) + Scissors (3)
    "A Y" 4  ;; Rock, Draw: Draw (3) + Rock (1)
    "A Z" 8  ;; Rock, Win (6) + Paper (2)
    "B X" 1  ;; Paper, Loss: Loss (0) + Rock (1)
    "B Y" 5  ;; Paper, Draw: Draw (3) + Paper (2)
    "B Z" 9  ;; Paper, Win: Win (6) + Scissors (3)
    "C X" 2  ;; Scissors, Loss: Loss (0) + Paper (2)
    "C Y" 6  ;; Scissors, Draw: Draw (3) + Scissors (3)
    "C Z" 7)) ;; Scissors, Win: Win (6) + Rock (1)

;; Part 1: Calculate total score based on the initial strategy guide.
(println (reduce + (map points-a strategy-guide)))

;; Part 2: Calculate total score based on the updated strategy guide.
(println (reduce + (map points-b strategy-guide)))