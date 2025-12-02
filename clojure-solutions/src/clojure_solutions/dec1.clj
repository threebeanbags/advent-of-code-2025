;; The Elves have good news and bad news.

;; The good news is that they've discovered project management! This has given them the tools they need to prevent their usual Christmas emergency. For example, they now know that the North Pole decorations need to be finished soon so that other critical tasks can start on time.

;; The bad news is that they've realized they have a different emergency: according to their resource planning, none of them have any time left to decorate the North Pole!

;; To save Christmas, the Elves need you to finish decorating the North Pole by December 12th.

;; Collect stars by solving puzzles. Two puzzles will be made available on each day; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

;; You arrive at the secret entrance to the North Pole base ready to start decorating. Unfortunately, the password seems to have been changed, so you can't get in. A document taped to the wall helpfully explains:

;; "Due to new security protocols, the password is locked in the safe below. Please see the attached document for the new combination."

;; The safe has a dial with only an arrow on it; around the dial are the numbers 0 through 99 in order. As you turn the dial, it makes a small click noise as it reaches each number.

;; The attached document (your puzzle input) contains a sequence of rotations, one per line, which tell you how to open the safe. A rotation starts with an L or R which indicates whether the rotation should be to the left (toward lower numbers) or to the right (toward higher numbers). Then, the rotation has a distance value which indicates how many clicks the dial should be rotated in that direction.

;; So, if the dial were pointing at 11, a rotation of R8 would cause the dial to point at 19. After that, a rotation of L19 would cause it to point at 0.

;; Because the dial is a circle, turning the dial left from 0 one click makes it point at 99. Similarly, turning the dial right from 99 one click makes it point at 0.

;; So, if the dial were pointing at 5, a rotation of L10 would cause it to point at 95. After that, a rotation of R5 could cause it to point at 0.

;; The dial starts by pointing at 50.

;; You could follow the instructions, but your recent required official North Pole secret entrance security training seminar taught you that the safe is actually a decoy. The actual password is the number of times the dial is left pointing at 0 after any rotation in the sequence.

(ns clojure-solutions.dec1)

;; "it counts if we end up on 0"
(defn problem-a [_ new-position _]
  (if (= new-position 0) 1 0))

;; "it counts if we clicked by it, too"
(defn problem-b [old-position new-position distance]
  (let [spins (quot (abs distance) 100) ;; count a spin no matter what
        adj (+ old-position (rem distance 100))  ;; get the adjusted distance minus the spins
        ;; check if we crossed 0
        crossed (and
                 ;; but not if we STARTED at 0
                 (not= old-position 0)
                 (or (>= adj 100) (<= adj 0)))]
    (+ spins (if crossed 1 0))))
         
(defn solve [moves count-condition]
  (loop [moves moves start 50 password-count 0]
    (cond
      (empty? moves) password-count
      :otherwise
      (let [move (first moves)
            direction (first move)
            distance (Integer/parseInt (subs move 1))
            distance (if (= direction \L) (- distance) distance)
            new-position (mod (+ start distance 100) 100)]
        (recur (rest moves)
               new-position
               (+ password-count (count-condition start new-position distance)))))))

(solve (clojure.string/split-lines (slurp "../data/1.txt")) problem-a)
(solve (clojure.string/split-lines (slurp "../data/1.txt")) problem-b)
