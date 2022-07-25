let full_url = params[0];
let query_params = #{page: 0};
let query_string;
let query_param;

if full_url.contains("?") {
    // Get the query string
    query_string = full_url.sub_string(full_url.index_of("?") + 1);
    do {
        // Seperate by parameters
        if query_string.contains("&") {
            query_param = query_string.sub_string(0, query_string.index_of("&"));
            query_string = query_string.sub_string(query_string.index_of("&") + 1);
        } else {
            query_param = query_string;
            query_string.clear();
        }
        // set the page parameter to the value from the url
        if query_param.sub_string(0, query_param.index_of("=")) == "page" {
            try {
            query_params.page = parse_int(query_param.sub_string(query_param.index_of("=") + 1));
            } catch {
                query_params.page = 0;
            }
        }
    } while ( query_string.len() > 0)
}

// Return the query_params
query_params