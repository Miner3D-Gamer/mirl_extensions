crate::mod_and_pub_import!(one_dimensional);
crate::mod_and_pub_import!(two_dimensional);
crate::mod_and_pub_import!(three_dimensional);
crate::mod_and_pub_import!(four_dimensional);
#[cfg(all(feature = "f128", feature = "f16"))]
crate::mod_and_pub_import!(direction);
