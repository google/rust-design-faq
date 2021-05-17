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
