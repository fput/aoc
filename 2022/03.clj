#!/usr/bin/env bb
(ns aoc
  (:require [clojure.string :as str]
            [clojure.set :as set]))

(def input (str/split-lines (slurp "in/03.txt")))

(def priorities
  (mapv #(cond
           (<= (long \A) % (long \Z)) (- % (long \A) -27)  ;; Uppercase ascii->points: A-Z = 27-52
           (<= (long \a) % (long \z)) (- % (long \a) -1)   ;; Lowercase ascii->points: a-z = 1-26
           :else 0)
        (range 128)))

(defn letter->priority 
  "Convert a character to its priority value."
  [c]
  (priorities (long c)))

(defn duplicate-priority
  "Find the common item type in the compartments and calculate its priority."
  [lst]
  (->> lst
       (map set)
       (reduce set/intersection)
       first
       letter->priority))

;; Part 1: Calculate sum of priorities for items appearing in both compartments of each rucksack.
(println (transduce
          (comp (map #(split-at (/ (count %) 2) %))
                (map duplicate-priority))
          +
          input))

;; Part 2: Calculate sum of priorities for items common to all three-elf groups.
(println (transduce
          (comp (partition-all 3)
                (map duplicate-priority))
          +
          input))