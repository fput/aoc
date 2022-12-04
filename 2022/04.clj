#!/usr/bin/env bb
(ns aoc
  (:require [clojure.string :as str]))

(def input (slurp "in/04.txt"))

(defn line->list
  "Convert a line of the form 'a-b,c-d' to a list of integers [a b c d]."
   [line]
  (let [i1 (str/index-of line \-)
        i2 (str/index-of line \,)
        i3 (str/index-of line \- i2)
        a (Integer/parseInt (subs line 0 i1)) ;; first set a-b
        b (Integer/parseInt (subs line (inc i1) i2))
        c (Integer/parseInt (subs line (inc i2) i3)) ;; second set c-d
        d (Integer/parseInt (subs line (inc i3)))]
    [a b c d]))

(def lists (->> input
                str/split-lines
                (map line->list)))

(defn part1
  "Check if one range fully contains the other."
  [[a b c d]]
  (or
   (and (<= a c) (>= b d)) ;; second subset of first
   (and (>= a c) (<= b d)))) ;; first subset of second

(defn part2
  "Check if two ranges overlap."
  [[a b c d]]
  (and (<= c b) (>= d a))) ;; overlap

;; Part 1: Count how many assignment pairs have one range fully containing the other.
(println (count (filter part1 lists)))

;; Part 2: Count how many assignment pairs overlap at all.
(println (count (filter part2 lists)))