(ns clojure1.kodnot)




(defn to-mod10 [y]
  "Översätt en lista med bokstäver till en lista av siffor, reducera och ta ut modulus per summa"
  (mod (reduce + (map (fn [x] (Character/digit x 10)) y)) 10))


(def fib
  "Generera upp en fibonacci-sekvens"
  (cons 0N (cons 1N (lazy-seq (map + fib (rest  fib))))))


(def strings
  "Skapa en lista av listor med bokstäver från fibonacci-numret"
  (map seq (map str fib)))

;; hämta ut 10 bokstavslistor från 2 och uppåt genom att droppa de tre första
;; summera "bokstäverna" och ta ut modulus per lista
;; summera listan och ta ut modulus.
(mod (reduce + (map to-mod10 (take 10 (drop 0 strings)))) 10)

;; hämta ut 50 bokstavslistor från 2 och uppåt genom att droppa de tre första
;; summera "bokstäverna" och ta ut modulus per lista
;; summera listan och ta ut modulus.
(mod(reduce + (map to-mod10 (take 50 (drop 0 strings)))) 10 )

;; hämta ut 1000 bokstavslistor från 2 och uppåt genom att droppa de tre första
;; summera "bokstäverna" och ta ut modulus per lista
;; summera listan och ta ut modulus.
(mod(reduce + (map to-mod10 (take 1000 (drop 0 strings)))) 10)