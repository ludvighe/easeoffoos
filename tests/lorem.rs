use rand::Rng;

const BASE_URL:&str = "http://localhost:8080/";

const LOREM_TEST_N: [i32;6] = [0, 1, 1000, 1_000_000_000, -1, -100_000_000];
const LOREM_TEST_S: [&str;6] = [
    // String
    "a",

    // Number String
    "1000_000_000",

    // Words
    "jab-splashy-goofball-shore-catfish-kite-unheard-remarry-dinner-blubber-flask-chalice-buccaneer",
    
    // Hash
    "2A7LuFays66P5ootc9uAnhNtmmbcxRF7zde2Taj3HMhF7q8GdgzZYdB2YP5CVExa3DRYFMFxbZE2SSjhufd3YMUWBRxfYZKexaeYRgZ6qGp7cyfpAZnFe4f5VKE8XckS",

    // Hash w/ special chars
    "dUD!D5mtDnyJmLoxP^S$nB*4b*u2fTvs*Xb6KnPpwwz^^XGNU5&yU3d!yF2YPf!o5KXJMg3QMYQ!99M77w$6^!7Rg9H75LEAz#ARDGc@m2NV4$hT28eVm3YA!BzyB77v",

    // Hash w/ ambiguous/escaped chars
    "$GZ3T2YPY6LhsyM€$xWXziuC!fc&sRSyb!VlU\"!b1mSxeRcPrwqkaYuF29l^ao3q700#f1X#c7uT7Yw9Tc69\\.&&!MSdd9N7wmB5b3aREcrPCyXbFf%KUq#/sS%69GRNm6tHG",
];



#[tokio::test]
async fn test_lorem_w() {
    let client = match httpc_test::new_client(BASE_URL) {
        Ok(client) => client,
        Err(_) => panic!(),
    };

    for n in LOREM_TEST_N {
        let result = match client.do_get(&format!("lorem/{n}")).await {
            Ok(res) => res,
            Err(_) => {
                println!("Failed to fetch lorem with n = {n}");
                panic!();
            }
        };

        let body = match result.text_body() {
            Ok(body)=>body,
            Err(_) => {
                println!("Failed to parse body with n = {n}");
                panic!();
            }
        };

        let words: Vec<&str> = body.split(" ").filter(|w|!w.is_empty()).collect();
        let words_len = words.len();

        let expected = n.clamp(0, 1_000_000);
        assert_eq!(words_len, expected as usize)
    }



    for s in LOREM_TEST_S {
        match client.do_get(&format!("lorem/{s}")).await {
            Ok(res) => res,
            Err(_) => {
                println!("Failed to fetch lorem with n = {s}");
                panic!();
            }
        };
    }
}

#[tokio::test]
async fn test_lorem_p() {
    let client = match httpc_test::new_client(BASE_URL) {
        Ok(client) => client,
        Err(_) => panic!(),
    };
    let mut rng = rand::thread_rng();
    

    for n in LOREM_TEST_N {
        let n_words = rng.gen_range(0..1_000_000);
        let result = match client.do_get(&format!("lorem/{n_words}?p={n}")).await {
            Ok(res) => res,
            Err(_) => {
                println!("Failed to fetch lorem with n = {n}");
                panic!();
            }
        };

        let body = match result.text_body() {
            Ok(body)=>body,
            Err(_) => {
                println!("Failed to parse body with n = {n}");
                panic!();
            }
        };

        let paragraphs: Vec<&str> = body.split("\n").filter(|w|!w.is_empty()).collect();
        let paragraphs_len = paragraphs.len();
        
        let expected = n.clamp(1, n_words) as usize;
        assert_eq!(paragraphs_len, expected)
    }

    for s in LOREM_TEST_S {
        let n_words = rng.gen_range(0..1_000_000);
        match client.do_get(&format!("lorem/{n_words}?p={s}")).await {
            Ok(res) => res,
            Err(_) => {
                println!("Failed to fetch lorem with n = {s}");
                panic!();
            }
        };
    }
}