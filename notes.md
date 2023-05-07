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
 - [ ] casino
 - [ ] franprix ?
 - [ ] super u
 - [ ] leader Price
 - [x] intermarché
 - [ ] Monoprix

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

## Casino =
