#!/usr/bin/env bb
(ns aoc
  (:require [clojure.string :as str]))

(def input (slurp "in/01.txt"))

(defn calories
  "Given a string representing one Elf's inventory, calculate the total calories."
  [elf-str]
  (->> elf-str
       str/split-lines
       (map #(Integer/parseInt %))
       (reduce +)))

(def calories-per-elf
  "Calculate the total calories for each Elf."
  (map calories (str/split input #"\n\n")))

;; Part 1: Maximum calories carried by any single Elf.
(println (apply max calories-per-elf))

;; Part 2: Sum of the top three highest calorie counts.
(println (->> calories-per-elf
              (sort >)
              (take 3)
              (apply +)))
