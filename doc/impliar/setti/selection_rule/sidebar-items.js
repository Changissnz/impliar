window.SIDEBAR_ITEMS = {"fn":[["build_requirement","builds the Requirement structure with 2-d matrix of dimension rs x k.  required := Vec<index in 0..k,indices not allowed>"],["build_requirement_matrix","2-dimensional matrix denoting (row,column) elements that are required; requirement == -1"],["build_restriction","builds the Restriction structure with 2-d matrix of dimension rs x k.  restricted := Vec<index in 0..k,indices not allowed>"],["build_restriction_matrix","builds the 2-d matrix of dimension rs x k used by a Restriction structure.  restricted := Vec<index in 0..k,indices not allowed>"],["build_rmatrix","description"],["check_rule_contents","description"],["collision_score","description"],["default_rmatrix","description"],["fix_rule_contents_1","description"],["next_available_forward","calculates the next choice given choice (of len k) by greedy forward selection (first available). Forward selection selects the first available index i for the subvector “[i:i + distance]”. both `choice` and `output` are ordered. "],["std_collision_score",""],["test_rule_contents",""],["test_rule_contents_2",""]],"struct":[["Requirement","structure containing 2-dimensional matrix denoting (row,column) elements that are required; requirement == -1"],["Restriction","structure containing 2-dimensional matrix denoting (row,column) elements that are restricted; restriction == 1"],["SelectionRule","a structure comprised of a Restriction and Requirement structure each containing an n x k matrix;  used for calculating k-sequences drawn from a vector of n choices. "]]};