(ns clojure-solutions.dec3
  (:require [clojure.string :refer [split-lines]]))

(defn load-set [filename]
  (->>
   (split-lines (slurp (str "../data/" filename)))
   (map (fn [x] (map #(Character/digit % 10) x)))))

(def test-set (load-set "3.test"))
(def problem-set (load-set "3.txt"))

(defn drop-first [f sq]
  (let [[a b] (split-with (complement f) sq)]
    (concat a (rest b))))

(defn max-jolt' [s n]
  (defn take-max-ignoring [p to-ignore]
    (let [largest (apply max (drop-last to-ignore p))
          remaining (rest (drop-while #(not= % largest) p))]
      [ largest remaining ]))
  (if (= n 1)
    (list (apply max s))
    (let [[d0 s'] (take-max-ignoring s (- n 1))]
      (cons d0 (max-jolt' s' (- n 1))))))
        
(defn max-jolt [s n]
  (reduce
   (fn [accum x] (+ (* accum 10) x))
   0
   (max-jolt' s n)))

(apply + (map #(max-jolt % 2) test-set))
(apply + (map #(max-jolt % 2) problem-set))
(apply + (map #(max-jolt % 12) test-set))
(apply + (map #(max-jolt % 12) problem-set))
