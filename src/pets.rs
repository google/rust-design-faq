# // Copyright 2020 Google LLC
# //
# // Licensed under the Apache License, Version 2.0 (the "License");
# // you may not use this file except in compliance with the License.
# // You may obtain a copy of the License at
# //
# //    https://www.apache.org/licenses/LICENSE-2.0
# //
# // Unless required by applicable law or agreed to in writing, software
# // distributed under the License is distributed on an "AS IS" BASIS,
# // WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# // See the License for the specific language governing permissions and
# // limitations under the License.
#
# use std::collections::HashSet;
# struct Animal {
#     kind: &'static str,
#     is_hungry: bool,
#     meal_needed: &'static str,
# }
#
# static PETS: [Animal; 4] = [
#     Animal {
#         kind: "Dog",
#         is_hungry: true,
#         meal_needed: "Kibble",
#     },
#     Animal {
#         kind: "Python",
#         is_hungry: false,
#         meal_needed: "Cat",
#     },
#     Animal {
#         kind: "Cat",
#         is_hungry: true,
#         meal_needed: "Kibble",
#     },
#     Animal {
#         kind: "Lion",
#         is_hungry: false,
#         meal_needed: "Kibble",
#     },
# ];
#
# static NEARBY_DUCK: Animal = Animal {
#     kind: "Duck",
#     is_hungry: true,
#     meal_needed: "pondweed",
# };
