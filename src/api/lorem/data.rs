// Sourced from https://archive.org/details/definibusbonoru02cicegoog

const LOREM: &str = "
Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore verritatis et quasi architecto baetae vitae dicta sunt explieabo. Nemo enim ipsam voluptatem quia tem sequi nesciunt. Neque porro quisquam est qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, sed quia nonnumquam eiusmodi tempora incidunt ut labore et dolore magnus aliquam quaerat voluptatem.

Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur? Quis autem vel eum iure reprehenderit qui in ea voluptate velit esse quam nihil molestiae consequatur, vel illum qui dolorem eum fugiat quo voluptas nulla pariatur?

At vero eos et accusamus et iusto odio diagnissimos ducimus qui blanditiis praesentium voluptatum deleniti ateque corrupti quos dolorem et quas molestias excepturi sint occaecati cupiditate non provident, similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et solorum fuga.

Et harum quidem rerum facilis est et expedita distinctio. Nam libero tempore, cum soluta nobis est eligendi optio cumque nihil impedit qui minus id quod maxime placeat facere possimus, omnis voluptas assumenda est, omnis dolor repellendus. Temporibus autem quibusdam et aut officiis debitis aut rerum necessitabus saepe eveniet ut et voluptates repudiandae sint et molestiae non recusandae. Itaque earum rerum hic tenetur a sapiente delectus ut aut reiciendis coluptatibus maiores alias consequatur aut preferendis doloribus asperiores repellat.
";

pub fn get_words() -> Vec<String> {
    return LOREM
        .split_whitespace()
        .filter(|w| !w.is_empty())
        .map(|w| String::from(w))
        .collect();
}
