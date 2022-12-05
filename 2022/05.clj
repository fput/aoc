#!/usr/bin/env bb
(ns aoc
  (:require [clojure.string :as str]))

(def input (str/split (slurp "in/05.txt")  #"\n\n"))

(defn string->ints
  "Convert a string to a vector of integers."
  [s]
  (vec (map #(Integer/parseInt %) (re-seq #"\d+" s))))

(defn rearrange
  "Rearrange crates between stacks according to the specified move.
  If multiple-crates? is true, crates are moved in bulk; otherwise, they are moved one by one."
  [stacks [n from to] multiple-crates?]
  (let [[sel remaining-source-pile] (split-at n (stacks from))
        dest-pile (concat (if multiple-crates? sel (reverse sel)) (stacks to))]
    (-> stacks
        (assoc from remaining-source-pile)
        (assoc to dest-pile))))

(def stacks
  "Initial crate stacks from input."
  (->> input
       first
       (#(str/split % #"\n"))
       drop-last
       (apply mapv vector)
       (drop 1)
       (take-nth 4)
       (map #(remove #{\space} %))
       (cons ())  ;; prepend empty list to get 1-based indexing
       vec))

(def rearrangements
  "List of rearrangement instructions from input."
  (->> input
       last
       (#(str/split % #"\n"))
       (map string->ints)))

(defn part1
  "Rearrange stacks for part 1 (moving crates one by one)."
  [stacks [n from to]]
  (rearrange stacks [n from to] false))

(defn part2
  "Rearrange stacks for part 2 (moving crates in bulk)."
  [stacks [n from to]]
  (rearrange stacks [n from to] true))

;; Part 1: Output the crates on top of each stack after all rearrangements.
(println (->> rearrangements
              (reduce part1 stacks)
              (map first)
              (apply str)))

;; Part 2: Output the crates on top of each stack after all rearrangements (bulk).
(println (->> rearrangements
              (reduce part2 stacks)
              (map first)
              (apply str)))