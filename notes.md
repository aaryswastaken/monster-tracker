Price class: ids-title ds-title--m product-card-price__price--final

Monster classique: https://www.carrefour.fr/p/boisson-energisante-monster-energy-5060335632302

<span.+class=\"[a-zA-Z0-9\-\_ ]*product-card-price__price--final\"[a-zA-Z0-9\-\_ =\"]*> ([0-9\,]*)€ <\/span>

grep -Poi

--cookie "FRONTAL_STORE=800333"

carrouf:

```
echo "$(curl https://www.carrefour.fr/p/boisson-energisante-monster-energy-5060335632302 --cookie "FRONTAL_STORE=800333" -A "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/112.0")" | pcregrep -M -o1 "<span.+class=\"[a-zA-Z0-9\-\_ ]*product-card-price__price--final\"[a-zA-Z0-9\-\_ =\"]*>[\n ]*([0-9\,]*)€[\n ]*<\/span>"
```


 - [x] carrefour
 - [x] casino
 - [x] franprix ?
 - [ ] super u
 - [ ] leader Price
 - [x] intermarché
 - [x] Monoprix

## Super u

c la merde mdr
server side bullshit -> que par puppeteet & cie


## Intermarché

cookie: itm_pdv: {%22ref%22:%2207042%22%2C%22isEcommerce%22:true%2C%22name%22:%22Express%2520Villeurbanne%22}

curl
sed "s/<!--[ ]*-->//g"
grep "<span class=\"productDetail__productPrice\"[<>a-zA-Z\= \"_0-9\/,!-]*€<\/span>"
sed "s/<\/span>//g"

echo "$(curl https://www.intermarche.com/produit/boisson-energisante-energy/5060335632302 --cookie "itm_pdv={%22ref%22:%2207042%22%2C%22isEcommerce%22:true%2C%22name%22:%22Express%2520Villeurbanne%22}" -A "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/112.0")" | sed "s/<\!--[ ]*-->//g" | pcregrep --buffer-size 1000000 -M -o1 "productDetail__productPrice\"[<>a-zA-Z-_\" \n=]*productDetail__integer[a-zA-Z-_\"0-9]*>([0-9]+<\/span>,[0-9]+) €" | sed "s/<\/span>//g"

## Casino

curl "https://www.casino.fr/casinoexpress_web/affichageDetailProduit/WE69944/F-77409-201-_-limonades-sodas-extrait/P-297494-monster-pacific-punch-energy-_-boisson-energisante-_-x1-canette?_=1683027746779" -A "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/112.0" --cookie 'typeShop=Z3&2841; shopId=WE69944&1930445000' | pcregrep -M -o1 "itemprop=\"price\">[ \t\n]+([0-9]+,[0-9*]+).*euro"


## Franprix

curl https://www.franprix.fr/courses/p/99044938 \
    -A "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/112.0" \
    --cookie 'vuex.user={"shipping":{"slot":{"id":1,"start":"2023-05-02T17:00:00.000Z","end":"2023-05-02T18:00:00.000Z","day":{"id":2,"date":"2023-05-01T22:00:00.000Z"},"label":"Mardi 2 mai - 19:00 - 20:00","selectedLabel":"Aujourd\'hui 19:00 - 20:00","dateOrigin":"2023-05-02T19:00:00/20:00:00"},"address":{"type":"store","lat":45.768361,"lon":4.866609,"street_number":"30","street":"Petite Rue De La Viabert","city":"Villeurbanne","postcode":"69100"},"store":{"id":"4768"},"shop":{"id":"4768","name":"Franprix Avenue du Marechal Foch","addressLabel":"30 Avenue du Marechal Foch, 69006 LYON","lat":45.77187,"lon":4.84302,"cart_min":5,"free_delivery_amount":45,"nb_products":3749,"nb_users_mark":24,"mark":4.7,"time_delivery":"40-100","distance":1.8690309604839066,"shipping_cost":1.99,"img":"https://cdn.mcommerce.franprix.fr/offer-images/0f370c25-5054-4812-9738-52b90caab117"}}}'



ALED

curl https://www.franprix.fr/courses/p/99044938 -A "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/112.0"

### Non optimized but working shit:

curl https://www.franprix.fr/courses/p/99044938 -A "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/112.0" --cookie "vuex.user=%7B%22shipping%22:%7B%22slot%22:%7B%22id%22:1,%22start%22:%222023-05-02T17:00:00.000Z%22,%22end%22:%222023-05-02T18:00:00.000Z%22,%22day%22:%7B%22id%22:2,%22date%22:%222023-05-01T22:00:00.000Z%22%7D,%22label%22:%22Mardi%202%20mai%20-%2019:00%20-%2020:00%22,%22selectedLabel%22:%22Aujourd'hui%2019:00%20-%2020:00%22,%22dateOrigin%22:%222023-05-02T19:00:00/20:00:00%22%7D,%22address%22:%7B%22type%22:%22store%22,%22lat%22:45.768361,%22lon%22:4.866609,%22street_number%22:%2230%22,%22street%22:%22Petite%20Rue%20De%20La%20Viabert%22,%22city%22:%22Villeurbanne%22,%22postcode%22:%2269100%22%7D,%22store%22:%7B%22id%22:%224768%22%7D,%22shop%22:%7B%22id%22:%224768%22,%22name%22:%22Franprix%20Avenue%20du%20Marechal%20Foch%22,%22addressLabel%22:%2230%20Avenue%20du%20Marechal%20Foch,%2069006%20LYON%22,%22lat%22:45.77187,%22lon%22:4.84302,%22cart_min%22:5,%22free_delivery_amount%22:45,%22nb_products%22:3749,%22nb_users_mark%22:24,%22mark%22:4.7,%22time_delivery%22:%2240-100%22,%22distance%22:1.8690309604839066,%22shipping_cost%22:1.99,%22img%22:%22https://cdn.mcommerce.franprix.fr/offer-images/0f370c25-5054-4812-9738-52b90caab117%22%7D%7D%7D" > out_franprix | pcregrep -M --buffer-size 10000000 -o1 "product-item-price[\ \"][a-zA-Z0-9\ \-\_\:\"\<\>]*([0-9]+,[0-9]*) €"


### Optimized

curl https://www.franprix.fr/courses/p/99044938 -A "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/112.0" --cookie "vuex.user=%7B%22shipping%22:%20%7B%22slot%22:%20%7B%7D,%22address%22:%20%7B%7D,%22store%22:%20%7B%22id%22:%20%225413%22%7D,%22shop%22:%20%7B%22id%22:%20%225413%22%7D%7D%7D" | pcregrep -M --buffer-size 10000000 -o1 "product-item-price[\ \"][a-zA-Z0-9\ \-\_\:\"\<\>]*([0-9]+,[0-9]*) €"

Note: has to keep both id and closed values or else server error

=> Url encoded because otherwise shell is too much trash


## Monoprix

### UNIFIED PRICES????

https://courses.monoprix.fr/api/v4/products/bop?retailerProductId=MPX_2443401


curl 'https://courses.monoprix.fr/api/v4/products/bop?retailerProductId=MPX_2443401' -A "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/112.0" | jq ".entities.product | to_entries[] | .value.price.current.amount "

No counter examples, I'll call it a working theory. I'll check irl soon(tm)


## Super U

dwac* ?


tcpid: 12352915437935935117
tfpsi: 0059ef84-057b-4add-84cb-ff9a8f548daa

session/dtSa


== trop relou
