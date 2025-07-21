
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use std::sync::{Arc, Mutex};
use crate::llm::faiss::FaissIndex;

struct PersonalityMatrix {
    profile: HashMap<i32, Cylinder>,
    entities: HashMap<i32, Cylinder>
    entity_groups: HashMap<i32, Vec<i32>>,
    traits: HashMap<&'static str, Trait>
}

struct Cylinder {
    start: i32,
    end: i32,
    word_index: i32,
    entity_type: EntityType,
    faiss: Arc<Mutex<FaissIndex>>,
    dimensions: Vec<Feature>,
    segments: HashMap<i32, Cylinder>
}

enum EntityType {
    Personal,
    Individual,   // family member, friend, co-worker, business colleague, employee
    Group,   // societies, clubs, sports teams, etc.
    Energy,    // work, learning, project, home repair, anything involving energy
    Other
}

struct Feature {
    start: i32,
    end: i32,
    word_index: i32
}

struct DataPoint {
    cylinder_index: i32,
    coord: Coordinate
    density: f32,
    size: i32,
    sentiment: i32,
    folding_xaxis: Option<i32>,
    related_cylinders: HashMap<i32, Vec<i32>>,
    creation_time: DateTime<Utc>,
    last_active: DateTime<Utc>,
    influencers: Vec<i32>,
    event_data: <i32>
}

struct Coordinate {
    x: i32,
    y: HashMap<i32, i32>,
    z: i32
}

No, it should be fine I think.  Ok, we have:
    PersonalityMatrix = user's overall personality profile
    - cylinders = segments of personality (eg. individual personality traits, family life, work life, hobbies and interests, likes / dislikes, routines and schedules, rules and avoidances, etc.)
    - Each cylinder is segmented on its x-axis by category (eg. food, audio / visual / physical entertainment, music, outdoors, etc.)
    - for all intents, all cylinders are segmented on their x-axis the same way, allowing the system to easily retrieve related and relevant data points from other aspects of the user's personality to take into account
    - Every category consist sof multiple y-axises which have normalized features / labels.  See below for example
    - z-axis is the person's mood at the time of data point creation.

Every data point consists of the following information:
    3d coordinates:  x, vector of y, z
    density score - based on time relevance (eg. 3 days ago vs. 16 months ago) and source of information (eg. user directly told AI assistnt, indirectly inferred, previous interation, etc.)
    size - Frequency for cases where the exact same coordinates are plotted more than once
    sentiment - score defining positive or negative for the data point
    creation time and last active for decay function of density score
    influencer tags (see below)
    event factoids _eg. exact dish eaten, song / band listened to, video watched, time of day, location, etc.)



Ok, to step through an example of adding a data point.  User says they went for a business dinner at a Thai restaurant, had spicy green Thai chicken curry with eggplant which they thought was delicious.

First, find the x-axis:
    Get correct cylinder (eg. personal likes / dislikes)
     Get "food" segment within cylinder, x = 4000 - 8999
    - get segment for "lunch / dinner meal", x = 5000 - 6499
    - get segment for "thai lunch / dinner meal", x = 5800 - 5899 
    - get segment for "chicken as main ingredient", x = 5832

Then of course, on this x-axis Korean and Vietnamese cuisines will be closer on the x-asix to Thai food than British / Swedish cuisines, for example, helping with similarity searches.

Now that we have the x-axis we need to get the vector of features for the dimension of the y-axis.  This will be manually defined during development, and fir this example, will utilize the set of features labelled for "food - lunch / dinner meal", such as:
    - flavor profile (sweet, spicy, tangy, etc.) = 88
    - cooking style (steamed, fried, sauteed, bbq, grilled, etc.) = 38
    - dish type (noodle, paste, rice, soup, salad, sandwich, etc.) = 54

Then z-axis will be there mood, which unless user states will be informal work dinner, so "informal, somewhat serious, slightly nervous", or whatever, I don't know.

So we have coordinates, assign strong density score since it's new and input came from user, sentiment will be high since user loved the food, event details will be name of restaurant, city / location, exact dish ordered, etc.  Additional labels / tags will be things such as Thailand, the people they went with, "dinner time", etc.

And that's it for adding a data point.  Does that make sense so far?  How does that sound?  Decent?


#### Querying

Right, and then querying is super efficient as well.  For example, say food question for dinner is wanted, and they're undecided on what they want but need to order for the family.

- System grabs personal likes / intersts cylinder, and already knows exactly wher eon the x-axis "lunch / dinner meal" starts and ends.
- Taking into account user's input / desires, their mood, and personality traits, using a simple yet to be written algorithm the system will decide on both, the center and radius of the sphere.
- The system grabs all data points within the sphere, plus all data points within relevant cylinders at the same spherical location (eg. rules and avoidances to use a filter, family life and their likes / dislikes since ordering for a family, etc.)
- System now has loads of quality, relevant data points without actually querying any data -- just a few simple math calculations.

Since user is undecided and in a mood to try new things, ray casting is done utilizing the influencer tags, which is used to grab relevant data points on totally unrelated parts of the x-axis.  This will be defined during development, but for example, it will be defined that geographical interests can sometimes sporadically and slightly help influence food interests.  

So it will ray cast, and the user's like of Portugal as a location and culture will end up getting collected as a relevant point, and since the user is undecided and in a good mood to try new things, maybe Protugese cuisine will be one of the recommendations.

So the majority of the time no actual query will even be done, then during the limited number of times ray casting is done is the only actual querying fo the data.  The rest is just basic math calculations.

Then still have to write the algorithm, but shouldn't be overly hard to score, rank, vectorize or whatever all the data points and pull out relevant information / recommendations / whatever.  Should work fine, I think.





