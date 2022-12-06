#!/usr/bin/env bb
(ns aoc (:gen-class))
(def input (slurp "in/06.txt"))

(defn marker-pos
  "Find the first position where the last `len` characters are all different."
  [s len]
  (->> s
       (partition len 1)
       (take-while #(not (apply distinct? %)))
       count
       (+ len)))

;; Part 1: Output the position of the first start-of-packet marker (4 distinct characters).
(println (marker-pos input 4))

;; Part 2: Output the position of the first start-of-message marker (14 distinct characters).
(println (marker-pos input 14))