/*
https://github.com/graphql-rust/juniper/blob/master/juniper/src/tests/schema.rs

BSD 2-Clause License

Copyright (c) 2016, Magnus Hallin
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

* Redistributions of source code must retain the above copyright notice, this
  list of conditions and the following disclaimer.

* Redistributions in binary form must reproduce the above copyright notice,
  this list of conditions and the following disclaimer in the documentation
  and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

use model::{Character, Database, Droid, Episode, Human};

use juniper::Context;

impl Context for Database {}

graphql_interface!(<'a> &'a Character: Database as "Character" |&self| {
    description: "A character in the Star Wars Trilogy"

    field id() -> &str as "The id of the character" {
        self.id()
    }

    field name() -> Option<&str> as "The name of the character" {
        Some(self.name())
    }

    field friends(&executor) -> Vec<&Character>
    as "The friends of the character" {
        executor.context().get_friends(self.as_character())
    }

    field appears_in() -> &[Episode] as "Which movies they appear in" {
        self.appears_in()
    }

    instance_resolvers: |&context| {
        &Human => context.get_human(&self.id()),
        &Droid => context.get_droid(&self.id()),
    }
});

graphql_object!(<'a> &'a Human: Database as "Human" |&self| {
    description: "A humanoid creature in the Star Wars universe."

    interfaces: [&Character]

    field id() -> &str as "The id of the human"{
        self.id()
    }

    field name() -> Option<&str> as "The name of the human" {
        Some(self.name())
    }

    field friends(&executor) -> Vec<&Character>
    as "The friends of the human" {
        executor.context().get_friends(self.as_character())
    }

    field appears_in() -> &[Episode] as "Which movies they appear in" {
        self.appears_in()
    }

    field home_planet() -> &Option<String> as "The home planet of the human" {
        self.home_planet()
    }
});

graphql_object!(<'a> &'a Droid: Database as "Droid" |&self| {
    description: "A mechanical creature in the Star Wars universe."

    interfaces: [&Character]

    field id() -> &str as "The id of the droid" {
        self.id()
    }

    field name() -> Option<&str> as "The name of the droid" {
        Some(self.name())
    }

    field friends(&executor) -> Vec<&Character>
    as "The friends of the droid" {
        executor.context().get_friends(self.as_character())
    }

    field appears_in() -> &[Episode] as "Which movies they appear in" {
        self.appears_in()
    }

    field primary_function() -> &Option<String> as "The primary function of the droid" {
        self.primary_function()
    }
});

graphql_object!(Database: Database as "Query" |&self| {
    description: "The root query object of the schema"

    field human(
        id: String as "id of the human"
    ) -> Option<&Human> {
        self.get_human(&id)
    }

    field droid(
        id: String as "id of the droid"
    ) -> Option<&Droid> {
        self.get_droid(&id)
    }

    field hero(
        episode: Option<Episode> as
        "If omitted, returns the hero of the whole saga. If provided, returns \
        the hero of that particular episode"
    ) -> Option<&Character> {
        Some(self.get_hero(episode).as_character())
    }
});
