#!/usr/bin/env bb
(ns aoc
  (:require [clojure.string :as str]))

(def input (slurp "in/11.txt"))

(defn numbers
  "Extract all numbers from a string and parse them as longs."
  [s]
  (map parse-long (re-seq #"\d+" s)))

(defn parse-monkey
  "Parse a block of text representing a monkey's attributes."
  [s]
  (let [lines (->> s str/split-lines vec)
        items (->> (nth lines 1) numbers)
        [_ a1 op a2] (->> (nth lines 2) (re-matches #".*= ([\d\w]+) ([\+\*]) ([\d\w]+)$"))
        func #((case op "*" * "+" +)
               (if (= a1 "old") % (parse-long a1))
               (if (= a2 "old") % (parse-long a2)))
        divisor (->> (nth lines 3) numbers first)
        divtrue (->> (nth lines 4) numbers first)
        divfalse (->> (nth lines 5) numbers first)
        next-fn #(if (zero? (mod % divisor)) divtrue divfalse)]
    {:items items, :f func, :next next-fn, :divisor divisor, :divtrue divtrue, :divfalse divfalse, :a1 a1, :op op, :a2 a2, :inspected 0}))

(def monkeys (->> (str/split input #"\n\n") (mapv parse-monkey)))

(defn next-round
  "Simulate the next round of item inspections and throws.
   ms_ - vector of monkeys with their current state.
   fworry - function to compute the new worry level."
  [ms_ fworry]
  (loop [ms ms_ ;; Initialize the state of monkeys.
         i 0]   ;; Start with the first monkey.
    (cond
       ;; If all monkeys have been processed, return the updated state.
      (>= i (count ms)) ms
      ;; If the current monkey has no items, move to the next monkey.
      (empty? (get-in ms [i :items])) (recur ms (inc i))
      ;; Otherwise, process the current monkey's first item.
      :else
      (let [m (nth ms i)            ;; Get the current monkey's state.
            it (first (:items m))   ;; Get the first item from the monkey's items.
            newit (fworry m it)     ;; Compute the new worry level using the fworry function.
            next ((:next m) newit)] ;; Determine the next monkey to receive the item. 
        (recur (-> ms
                   ;; Remove the processed item from the current monkey's items. 
                   (update-in [i :items] rest)
                   ;; Increment the inspection count for the current monkey.
                   (update-in [i :inspected] inc)
                   ;; Add the new item to the next monkey's items.
                   (update-in [next :items] concat (list newit)))
               i)))))  ;; Continue processing the current monkey (i) until all items are processed.

(defn monkey-business
  "Calculate the level of monkey business by multiplying the inspections of the two most active monkeys."
  [ms]
  (->> ms
       (map :inspected)
       (sort >)
       (take 2)
       (reduce *)))

(def modulo (->> monkeys (map :divisor) (reduce *)))

(def fworry1 (fn [m old] (int (/ ((:f m) old) 3))))
(def fworry2 (fn [m old] (mod ((:f m) old) modulo)))

 ;; Part 1: Calculate the level of monkey business after 20 rounds with initial worry function.
(println
 (->> monkeys
      (iterate #(next-round % fworry1))
      (#(nth % 20))
      monkey-business))

 ;; Part 2: Calculate the level of monkey business after 10000 rounds with modified worry function.
(println
 (->> monkeys
      (iterate #(next-round % fworry2))
      (#(nth % 10000))
      monkey-business))