#!/usr/bin/env bb
(ns aoc
  (:require [clojure.string :as str]))

(def input (slurp "in/10.txt"))

(defn evall
  "Evaluate the given line and update the value of x accordingly."
  [line x]
  (cond (< (count line) 5) [x] ;; For 'noop' instruction.
        :else [x (+ x (parse-long (subs line 5)))])) ;; For 'addx' instruction.

(def cycles
  "Generate the list of cycle values by evaluating each line of input."
  (->> input
       str/split-lines
       (reductions #(evall %2 (last %1)) [1])
       (apply concat)
       vec))

(defn signal-strength
  "Calculate the signal strength at the given cycle."
  [cycle]
  (* cycle (nth cycles (dec cycle))))

;; Part 1: Calculate the sum of signal strengths at the specified cycles.
(println (reduce + (map signal-strength [20 60 100 140 180 220])))

(defn render-pixel
  "Render the pixel at the given index based on the value of x."
  [index x]
  (if (<= -1 (- x (mod index 40)) 1) \# \.))

;; Part 2: Render the image given by the program.
(println
 (->> cycles
      drop-last
      (map-indexed render-pixel)
      (partition 40)
      (map (partial apply str))
      (str/join "\n")))
