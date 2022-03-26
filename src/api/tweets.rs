use super::TwitterApi;
use crate::api_result::ApiResult;
use crate::authorization::Authorization;
use crate::data::{Deleted, Tweet};
use crate::id::IntoId;
use crate::meta::TweetsMeta;
use crate::query::{get_req_builder, UrlQueryExt};
use crate::TweetBuilder;
use reqwest::Method;

get_req_builder! {
pub struct GetTweetsRequestBuilder {
    media_fields,
    user_fields,
    poll_fields,
    tweet_fields,
    place_fields,
    tweet_expansions
}
}

get_req_builder! {
pub struct GetTimelineRequestBuilder {
    media_fields,
    user_fields,
    poll_fields,
    tweet_fields,
    place_fields,
    tweet_expansions,
    exclude,
    start_time,
    end_time,
    since_id,
    until_id,
    max_results,
    pagination_token
}
}

get_req_builder! {
pub struct GetTweetsSearchRequestBuilder {
    media_fields,
    user_fields,
    poll_fields,
    tweet_fields,
    place_fields,
    tweet_expansions,
    exclude,
    start_time,
    end_time,
    since_id,
    until_id,
    max_results,
    sort_order,
    pagination_token
}
}

impl<A> TwitterApi<A>
where
    A: Authorization,
{
    pub fn get_tweets(
        &self,
        ids: impl IntoIterator<Item = impl IntoId>,
    ) -> GetTweetsRequestBuilder<A, Vec<Tweet>, Option<()>> {
        let mut url = self.url("tweets").unwrap();
        url.append_query_seq("ids", ids);
        GetTweetsRequestBuilder::new(self, url)
    }
    pub fn get_tweet(&self, id: impl IntoId) -> GetTweetsRequestBuilder<A, Tweet, Option<()>> {
        GetTweetsRequestBuilder::new(self, self.url(format!("tweets/{id}")).unwrap())
    }
    pub fn get_user_tweets(
        &self,
        user_id: impl IntoId,
    ) -> GetTimelineRequestBuilder<A, Vec<Tweet>, TweetsMeta> {
        GetTimelineRequestBuilder::new(self, self.url(format!("users/{user_id}/tweets")).unwrap())
    }
    pub fn get_user_mentions(
        &self,
        user_id: impl IntoId,
    ) -> GetTimelineRequestBuilder<A, Vec<Tweet>, TweetsMeta> {
        GetTimelineRequestBuilder::new(self, self.url(format!("users/{user_id}/mentions")).unwrap())
    }
    pub fn get_tweets_search_recent(
        &self,
        query: impl ToString,
    ) -> GetTweetsSearchRequestBuilder<A, Vec<Tweet>, TweetsMeta> {
        let mut url = self.url("tweets/search/recent").unwrap();
        url.append_query_val("query", query);
        GetTweetsSearchRequestBuilder::new(self, url)
    }
    pub fn get_tweets_search_all(
        &self,
        query: impl ToString,
    ) -> GetTweetsSearchRequestBuilder<A, Vec<Tweet>, TweetsMeta> {
        let mut url = self.url("tweets/search/all").unwrap();
        url.append_query_val("query", query);
        GetTweetsSearchRequestBuilder::new(self, url)
    }
    pub fn post_tweet(&self) -> TweetBuilder<A> {
        TweetBuilder::new(self, self.url("tweets").unwrap())
    }
    pub async fn delete_tweet(&self, id: impl IntoId) -> ApiResult<A, Deleted, Option<()>> {
        self.send(self.request(Method::DELETE, self.url(format!("tweets/{id}"))?))
            .await
    }
}
