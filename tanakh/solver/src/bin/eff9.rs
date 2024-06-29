use num_bigint::BigInt;
use z3::ast::{self, Ast as _};

const CONSTRAINTS9: &[(usize, usize)] = &[
    (11, 12),
    (11, 13),
    (11, 14),
    (11, 15),
    (11, 16),
    (11, 17),
    (11, 18),
    (11, 19),
    (11, 21),
    (11, 22),
    (11, 23),
    (11, 31),
    (11, 32),
    (11, 33),
    (11, 41),
    (11, 51),
    (11, 61),
    (11, 71),
    (11, 81),
    (11, 91),
    (12, 13),
    (12, 14),
    (12, 15),
    (12, 16),
    (12, 17),
    (12, 18),
    (12, 19),
    (12, 21),
    (12, 22),
    (12, 23),
    (12, 31),
    (12, 32),
    (12, 33),
    (12, 42),
    (12, 52),
    (12, 62),
    (12, 72),
    (12, 82),
    (12, 92),
    (13, 14),
    (13, 15),
    (13, 16),
    (13, 17),
    (13, 18),
    (13, 19),
    (13, 21),
    (13, 22),
    (13, 23),
    (13, 31),
    (13, 32),
    (13, 33),
    (13, 43),
    (13, 53),
    (13, 63),
    (13, 73),
    (13, 83),
    (13, 93),
    (14, 15),
    (14, 16),
    (14, 17),
    (14, 18),
    (14, 19),
    (14, 24),
    (14, 25),
    (14, 26),
    (14, 34),
    (14, 35),
    (14, 36),
    (14, 44),
    (14, 54),
    (14, 64),
    (14, 74),
    (14, 84),
    (14, 94),
    (15, 16),
    (15, 17),
    (15, 18),
    (15, 19),
    (15, 24),
    (15, 25),
    (15, 26),
    (15, 34),
    (15, 35),
    (15, 36),
    (15, 45),
    (15, 55),
    (15, 65),
    (15, 75),
    (15, 85),
    (15, 95),
    (16, 17),
    (16, 18),
    (16, 19),
    (16, 24),
    (16, 25),
    (16, 26),
    (16, 34),
    (16, 35),
    (16, 36),
    (16, 46),
    (16, 56),
    (16, 66),
    (16, 76),
    (16, 86),
    (16, 96),
    (17, 18),
    (17, 19),
    (17, 27),
    (17, 28),
    (17, 29),
    (17, 37),
    (17, 38),
    (17, 39),
    (17, 47),
    (17, 57),
    (17, 67),
    (17, 77),
    (17, 87),
    (17, 97),
    (18, 19),
    (18, 27),
    (18, 28),
    (18, 29),
    (18, 37),
    (18, 38),
    (18, 39),
    (18, 48),
    (18, 58),
    (18, 68),
    (18, 78),
    (18, 88),
    (18, 98),
    (19, 27),
    (19, 28),
    (19, 29),
    (19, 37),
    (19, 38),
    (19, 39),
    (19, 49),
    (19, 59),
    (19, 69),
    (19, 79),
    (19, 89),
    (19, 99),
    (21, 22),
    (21, 23),
    (21, 24),
    (21, 25),
    (21, 26),
    (21, 27),
    (21, 28),
    (21, 29),
    (21, 31),
    (21, 32),
    (21, 33),
    (21, 41),
    (21, 51),
    (21, 61),
    (21, 71),
    (21, 81),
    (21, 91),
    (22, 23),
    (22, 24),
    (22, 25),
    (22, 26),
    (22, 27),
    (22, 28),
    (22, 29),
    (22, 31),
    (22, 32),
    (22, 33),
    (22, 42),
    (22, 52),
    (22, 62),
    (22, 72),
    (22, 82),
    (22, 92),
    (23, 24),
    (23, 25),
    (23, 26),
    (23, 27),
    (23, 28),
    (23, 29),
    (23, 31),
    (23, 32),
    (23, 33),
    (23, 43),
    (23, 53),
    (23, 63),
    (23, 73),
    (23, 83),
    (23, 93),
    (24, 25),
    (24, 26),
    (24, 27),
    (24, 28),
    (24, 29),
    (24, 34),
    (24, 35),
    (24, 36),
    (24, 44),
    (24, 54),
    (24, 64),
    (24, 74),
    (24, 84),
    (24, 94),
    (25, 26),
    (25, 27),
    (25, 28),
    (25, 29),
    (25, 34),
    (25, 35),
    (25, 36),
    (25, 45),
    (25, 55),
    (25, 65),
    (25, 75),
    (25, 85),
    (25, 95),
    (26, 27),
    (26, 28),
    (26, 29),
    (26, 34),
    (26, 35),
    (26, 36),
    (26, 46),
    (26, 56),
    (26, 66),
    (26, 76),
    (26, 86),
    (26, 96),
    (27, 28),
    (27, 29),
    (27, 37),
    (27, 38),
    (27, 39),
    (27, 47),
    (27, 57),
    (27, 67),
    (27, 77),
    (27, 87),
    (27, 97),
    (28, 29),
    (28, 37),
    (28, 38),
    (28, 39),
    (28, 48),
    (28, 58),
    (28, 68),
    (28, 78),
    (28, 88),
    (28, 98),
    (29, 37),
    (29, 38),
    (29, 39),
    (29, 49),
    (29, 59),
    (29, 69),
    (29, 79),
    (29, 89),
    (29, 99),
    (31, 32),
    (31, 33),
    (31, 34),
    (31, 35),
    (31, 36),
    (31, 37),
    (31, 38),
    (31, 39),
    (31, 41),
    (31, 51),
    (31, 61),
    (31, 71),
    (31, 81),
    (31, 91),
    (32, 33),
    (32, 34),
    (32, 35),
    (32, 36),
    (32, 37),
    (32, 38),
    (32, 39),
    (32, 42),
    (32, 52),
    (32, 62),
    (32, 72),
    (32, 82),
    (32, 92),
    (33, 34),
    (33, 35),
    (33, 36),
    (33, 37),
    (33, 38),
    (33, 39),
    (33, 43),
    (33, 53),
    (33, 63),
    (33, 73),
    (33, 83),
    (33, 93),
    (34, 35),
    (34, 36),
    (34, 37),
    (34, 38),
    (34, 39),
    (34, 44),
    (34, 54),
    (34, 64),
    (34, 74),
    (34, 84),
    (34, 94),
    (35, 36),
    (35, 37),
    (35, 38),
    (35, 39),
    (35, 45),
    (35, 55),
    (35, 65),
    (35, 75),
    (35, 85),
    (35, 95),
    (36, 37),
    (36, 38),
    (36, 39),
    (36, 46),
    (36, 56),
    (36, 66),
    (36, 76),
    (36, 86),
    (36, 96),
    (37, 38),
    (37, 39),
    (37, 47),
    (37, 57),
    (37, 67),
    (37, 77),
    (37, 87),
    (37, 97),
    (38, 39),
    (38, 48),
    (38, 58),
    (38, 68),
    (38, 78),
    (38, 88),
    (38, 98),
    (39, 49),
    (39, 59),
    (39, 69),
    (39, 79),
    (39, 89),
    (39, 99),
    (41, 42),
    (41, 43),
    (41, 44),
    (41, 45),
    (41, 46),
    (41, 47),
    (41, 48),
    (41, 49),
    (41, 51),
    (41, 52),
    (41, 53),
    (41, 61),
    (41, 62),
    (41, 63),
    (41, 71),
    (41, 81),
    (41, 91),
    (42, 43),
    (42, 44),
    (42, 45),
    (42, 46),
    (42, 47),
    (42, 48),
    (42, 49),
    (42, 51),
    (42, 52),
    (42, 53),
    (42, 61),
    (42, 62),
    (42, 63),
    (42, 72),
    (42, 82),
    (42, 92),
    (43, 44),
    (43, 45),
    (43, 46),
    (43, 47),
    (43, 48),
    (43, 49),
    (43, 51),
    (43, 52),
    (43, 53),
    (43, 61),
    (43, 62),
    (43, 63),
    (43, 73),
    (43, 83),
    (43, 93),
    (44, 45),
    (44, 46),
    (44, 47),
    (44, 48),
    (44, 49),
    (44, 54),
    (44, 55),
    (44, 56),
    (44, 64),
    (44, 65),
    (44, 66),
    (44, 74),
    (44, 84),
    (44, 94),
    (45, 46),
    (45, 47),
    (45, 48),
    (45, 49),
    (45, 54),
    (45, 55),
    (45, 56),
    (45, 64),
    (45, 65),
    (45, 66),
    (45, 75),
    (45, 85),
    (45, 95),
    (46, 47),
    (46, 48),
    (46, 49),
    (46, 54),
    (46, 55),
    (46, 56),
    (46, 64),
    (46, 65),
    (46, 66),
    (46, 76),
    (46, 86),
    (46, 96),
    (47, 48),
    (47, 49),
    (47, 57),
    (47, 58),
    (47, 59),
    (47, 67),
    (47, 68),
    (47, 69),
    (47, 77),
    (47, 87),
    (47, 97),
    (48, 49),
    (48, 57),
    (48, 58),
    (48, 59),
    (48, 67),
    (48, 68),
    (48, 69),
    (48, 78),
    (48, 88),
    (48, 98),
    (49, 57),
    (49, 58),
    (49, 59),
    (49, 67),
    (49, 68),
    (49, 69),
    (49, 79),
    (49, 89),
    (49, 99),
    (51, 52),
    (51, 53),
    (51, 54),
    (51, 55),
    (51, 56),
    (51, 57),
    (51, 58),
    (51, 59),
    (51, 61),
    (51, 62),
    (51, 63),
    (51, 71),
    (51, 81),
    (51, 91),
    (52, 53),
    (52, 54),
    (52, 55),
    (52, 56),
    (52, 57),
    (52, 58),
    (52, 59),
    (52, 61),
    (52, 62),
    (52, 63),
    (52, 72),
    (52, 82),
    (52, 92),
    (53, 54),
    (53, 55),
    (53, 56),
    (53, 57),
    (53, 58),
    (53, 59),
    (53, 61),
    (53, 62),
    (53, 63),
    (53, 73),
    (53, 83),
    (53, 93),
    (54, 55),
    (54, 56),
    (54, 57),
    (54, 58),
    (54, 59),
    (54, 64),
    (54, 65),
    (54, 66),
    (54, 74),
    (54, 84),
    (54, 94),
    (55, 56),
    (55, 57),
    (55, 58),
    (55, 59),
    (55, 64),
    (55, 65),
    (55, 66),
    (55, 75),
    (55, 85),
    (55, 95),
    (56, 57),
    (56, 58),
    (56, 59),
    (56, 64),
    (56, 65),
    (56, 66),
    (56, 76),
    (56, 86),
    (56, 96),
    (57, 58),
    (57, 59),
    (57, 67),
    (57, 68),
    (57, 69),
    (57, 77),
    (57, 87),
    (57, 97),
    (58, 59),
    (58, 67),
    (58, 68),
    (58, 69),
    (58, 78),
    (58, 88),
    (58, 98),
    (59, 67),
    (59, 68),
    (59, 69),
    (59, 79),
    (59, 89),
    (59, 99),
    (61, 62),
    (61, 63),
    (61, 64),
    (61, 65),
    (61, 66),
    (61, 67),
    (61, 68),
    (61, 69),
    (61, 71),
    (61, 81),
    (61, 91),
    (62, 63),
    (62, 64),
    (62, 65),
    (62, 66),
    (62, 67),
    (62, 68),
    (62, 69),
    (62, 72),
    (62, 82),
    (62, 92),
    (63, 64),
    (63, 65),
    (63, 66),
    (63, 67),
    (63, 68),
    (63, 69),
    (63, 73),
    (63, 83),
    (63, 93),
    (64, 65),
    (64, 66),
    (64, 67),
    (64, 68),
    (64, 69),
    (64, 74),
    (64, 84),
    (64, 94),
    (65, 66),
    (65, 67),
    (65, 68),
    (65, 69),
    (65, 75),
    (65, 85),
    (65, 95),
    (66, 67),
    (66, 68),
    (66, 69),
    (66, 76),
    (66, 86),
    (66, 96),
    (67, 68),
    (67, 69),
    (67, 77),
    (67, 87),
    (67, 97),
    (68, 69),
    (68, 78),
    (68, 88),
    (68, 98),
    (69, 79),
    (69, 89),
    (69, 99),
    (71, 72),
    (71, 73),
    (71, 74),
    (71, 75),
    (71, 76),
    (71, 77),
    (71, 78),
    (71, 79),
    (71, 81),
    (71, 82),
    (71, 83),
    (71, 91),
    (71, 92),
    (71, 93),
    (72, 73),
    (72, 74),
    (72, 75),
    (72, 76),
    (72, 77),
    (72, 78),
    (72, 79),
    (72, 81),
    (72, 82),
    (72, 83),
    (72, 91),
    (72, 92),
    (72, 93),
    (73, 74),
    (73, 75),
    (73, 76),
    (73, 77),
    (73, 78),
    (73, 79),
    (73, 81),
    (73, 82),
    (73, 83),
    (73, 91),
    (73, 92),
    (73, 93),
    (74, 75),
    (74, 76),
    (74, 77),
    (74, 78),
    (74, 79),
    (74, 84),
    (74, 85),
    (74, 86),
    (74, 94),
    (74, 95),
    (74, 96),
    (75, 76),
    (75, 77),
    (75, 78),
    (75, 79),
    (75, 84),
    (75, 85),
    (75, 86),
    (75, 94),
    (75, 95),
    (75, 96),
    (76, 77),
    (76, 78),
    (76, 79),
    (76, 84),
    (76, 85),
    (76, 86),
    (76, 94),
    (76, 95),
    (76, 96),
    (77, 78),
    (77, 79),
    (77, 87),
    (77, 88),
    (77, 89),
    (77, 97),
    (77, 98),
    (77, 99),
    (78, 79),
    (78, 87),
    (78, 88),
    (78, 89),
    (78, 97),
    (78, 98),
    (78, 99),
    (79, 87),
    (79, 88),
    (79, 89),
    (79, 97),
    (79, 98),
    (79, 99),
    (81, 82),
    (81, 83),
    (81, 84),
    (81, 85),
    (81, 86),
    (81, 87),
    (81, 88),
    (81, 89),
    (81, 91),
    (81, 92),
    (81, 93),
    (82, 83),
    (82, 84),
    (82, 85),
    (82, 86),
    (82, 87),
    (82, 88),
    (82, 89),
    (82, 91),
    (82, 92),
    (82, 93),
    (83, 84),
    (83, 85),
    (83, 86),
    (83, 87),
    (83, 88),
    (83, 89),
    (83, 91),
    (83, 92),
    (83, 93),
    (84, 85),
    (84, 86),
    (84, 87),
    (84, 88),
    (84, 89),
    (84, 94),
    (84, 95),
    (84, 96),
    (85, 86),
    (85, 87),
    (85, 88),
    (85, 89),
    (85, 94),
    (85, 95),
    (85, 96),
    (86, 87),
    (86, 88),
    (86, 89),
    (86, 94),
    (86, 95),
    (86, 96),
    (87, 88),
    (87, 89),
    (87, 97),
    (87, 98),
    (87, 99),
    (88, 89),
    (88, 97),
    (88, 98),
    (88, 99),
    (89, 97),
    (89, 98),
    (89, 99),
    (91, 92),
    (91, 93),
    (91, 94),
    (91, 95),
    (91, 96),
    (91, 97),
    (91, 98),
    (91, 99),
    (92, 93),
    (92, 94),
    (92, 95),
    (92, 96),
    (92, 97),
    (92, 98),
    (92, 99),
    (93, 94),
    (93, 95),
    (93, 96),
    (93, 97),
    (93, 98),
    (93, 99),
    (94, 95),
    (94, 96),
    (94, 97),
    (94, 98),
    (94, 99),
    (95, 96),
    (95, 97),
    (95, 98),
    (95, 99),
    (96, 97),
    (96, 98),
    (96, 99),
    (97, 98),
    (97, 99),
    (98, 99),
];

const CONSTRAINTS10: &[(usize, usize)] = &[
    (11, 12),
    (11, 13),
    (11, 14),
    (11, 15),
    (11, 16),
    (11, 17),
    (11, 18),
    (11, 19),
    (11, 21),
    (11, 22),
    (11, 23),
    (11, 31),
    (11, 32),
    (11, 33),
    (11, 41),
    (11, 51),
    (11, 61),
    (11, 71),
    (11, 81),
    (11, 91),
    (12, 13),
    (12, 14),
    (12, 15),
    (12, 16),
    (12, 17),
    (12, 18),
    (12, 19),
    (12, 21),
    (12, 22),
    (12, 23),
    (12, 31),
    (12, 32),
    (12, 33),
    (12, 42),
    (12, 52),
    (12, 62),
    (12, 72),
    (12, 82),
    (12, 92),
    (13, 14),
    (13, 15),
    (13, 16),
    (13, 17),
    (13, 18),
    (13, 19),
    (13, 21),
    (13, 22),
    (13, 23),
    (13, 31),
    (13, 32),
    (13, 33),
    (13, 43),
    (13, 53),
    (13, 63),
    (13, 73),
    (13, 83),
    (13, 93),
    (14, 15),
    (14, 16),
    (14, 17),
    (14, 18),
    (14, 19),
    (14, 24),
    (14, 25),
    (14, 26),
    (14, 34),
    (14, 35),
    (14, 36),
    (14, 44),
    (14, 54),
    (14, 64),
    (14, 74),
    (14, 84),
    (14, 94),
    (15, 16),
    (15, 17),
    (15, 18),
    (15, 19),
    (15, 24),
    (15, 25),
    (15, 26),
    (15, 34),
    (15, 35),
    (15, 36),
    (15, 45),
    (15, 55),
    (15, 65),
    (15, 75),
    (15, 85),
    (15, 95),
    (16, 17),
    (16, 18),
    (16, 19),
    (16, 24),
    (16, 25),
    (16, 26),
    (16, 34),
    (16, 35),
    (16, 36),
    (16, 46),
    (16, 56),
    (16, 66),
    (16, 76),
    (16, 86),
    (16, 96),
    (17, 18),
    (17, 19),
    (17, 27),
    (17, 28),
    (17, 29),
    (17, 37),
    (17, 38),
    (17, 39),
    (17, 47),
    (17, 57),
    (17, 67),
    (17, 77),
    (17, 87),
    (17, 97),
    (18, 19),
    (18, 27),
    (18, 28),
    (18, 29),
    (18, 37),
    (18, 38),
    (18, 39),
    (18, 48),
    (18, 58),
    (18, 68),
    (18, 78),
    (18, 88),
    (18, 98),
    (19, 27),
    (19, 28),
    (19, 29),
    (19, 37),
    (19, 38),
    (19, 39),
    (19, 49),
    (19, 59),
    (19, 69),
    (19, 79),
    (19, 89),
    (19, 99),
    (21, 22),
    (21, 23),
    (21, 24),
    (21, 25),
    (21, 26),
    (21, 27),
    (21, 28),
    (21, 29),
    (21, 31),
    (21, 32),
    (21, 33),
    (21, 41),
    (21, 51),
    (21, 61),
    (21, 71),
    (21, 81),
    (21, 91),
    (22, 23),
    (22, 24),
    (22, 25),
    (22, 26),
    (22, 27),
    (22, 28),
    (22, 29),
    (22, 31),
    (22, 32),
    (22, 33),
    (22, 42),
    (22, 52),
    (22, 62),
    (22, 72),
    (22, 82),
    (22, 92),
    (23, 24),
    (23, 25),
    (23, 26),
    (23, 27),
    (23, 28),
    (23, 29),
    (23, 31),
    (23, 32),
    (23, 33),
    (23, 43),
    (23, 53),
    (23, 63),
    (23, 73),
    (23, 83),
    (23, 93),
    (24, 25),
    (24, 26),
    (24, 27),
    (24, 28),
    (24, 29),
    (24, 34),
    (24, 35),
    (24, 36),
    (24, 44),
    (24, 54),
    (24, 64),
    (24, 74),
    (24, 84),
    (24, 94),
    (25, 26),
    (25, 27),
    (25, 28),
    (25, 29),
    (25, 34),
    (25, 35),
    (25, 36),
    (25, 45),
    (25, 55),
    (25, 65),
    (25, 75),
    (25, 85),
    (25, 95),
    (26, 27),
    (26, 28),
    (26, 29),
    (26, 34),
    (26, 35),
    (26, 36),
    (26, 46),
    (26, 56),
    (26, 66),
    (26, 76),
    (26, 86),
    (26, 96),
    (27, 28),
    (27, 29),
    (27, 37),
    (27, 38),
    (27, 39),
    (27, 47),
    (27, 57),
    (27, 67),
    (27, 77),
    (27, 87),
    (27, 97),
    (28, 29),
    (28, 37),
    (28, 38),
    (28, 39),
    (28, 48),
    (28, 58),
    (28, 68),
    (28, 78),
    (28, 88),
    (28, 98),
    (29, 37),
    (29, 38),
    (29, 39),
    (29, 49),
    (29, 59),
    (29, 69),
    (29, 79),
    (29, 89),
    (29, 99),
    (31, 32),
    (31, 33),
    (31, 34),
    (31, 35),
    (31, 36),
    (31, 37),
    (31, 38),
    (31, 39),
    (31, 41),
    (31, 51),
    (31, 61),
    (31, 71),
    (31, 81),
    (31, 91),
    (32, 33),
    (32, 34),
    (32, 35),
    (32, 36),
    (32, 37),
    (32, 38),
    (32, 39),
    (32, 42),
    (32, 52),
    (32, 62),
    (32, 72),
    (32, 82),
    (32, 92),
    (33, 34),
    (33, 35),
    (33, 36),
    (33, 37),
    (33, 38),
    (33, 39),
    (33, 43),
    (33, 53),
    (33, 63),
    (33, 73),
    (33, 83),
    (33, 93),
    (34, 35),
    (34, 36),
    (34, 37),
    (34, 38),
    (34, 39),
    (34, 44),
    (34, 54),
    (34, 64),
    (34, 74),
    (34, 84),
    (34, 94),
    (35, 36),
    (35, 37),
    (35, 38),
    (35, 39),
    (35, 45),
    (35, 55),
    (35, 65),
    (35, 75),
    (35, 85),
    (35, 95),
    (36, 37),
    (36, 38),
    (36, 39),
    (36, 46),
    (36, 56),
    (36, 66),
    (36, 76),
    (36, 86),
    (36, 96),
    (37, 38),
    (37, 39),
    (37, 47),
    (37, 57),
    (37, 67),
    (37, 77),
    (37, 87),
    (37, 97),
    (38, 39),
    (38, 48),
    (38, 58),
    (38, 68),
    (38, 78),
    (38, 88),
    (38, 98),
    (39, 49),
    (39, 59),
    (39, 69),
    (39, 79),
    (39, 89),
    (39, 99),
    (41, 42),
    (41, 43),
    (41, 44),
    (41, 45),
    (41, 46),
    (41, 47),
    (41, 48),
    (41, 49),
    (41, 51),
    (41, 52),
    (41, 53),
    (41, 61),
    (41, 62),
    (41, 63),
    (41, 71),
    (41, 81),
    (41, 91),
    (42, 43),
    (42, 44),
    (42, 45),
    (42, 46),
    (42, 47),
    (42, 48),
    (42, 49),
    (42, 51),
    (42, 52),
    (42, 53),
    (42, 61),
    (42, 62),
    (42, 63),
    (42, 72),
    (42, 82),
    (42, 92),
    (43, 44),
    (43, 45),
    (43, 46),
    (43, 47),
    (43, 48),
    (43, 49),
    (43, 51),
    (43, 52),
    (43, 53),
    (43, 61),
    (43, 62),
    (43, 63),
    (43, 73),
    (43, 83),
    (43, 93),
    (44, 45),
    (44, 46),
    (44, 47),
    (44, 48),
    (44, 49),
    (44, 54),
    (44, 55),
    (44, 56),
    (44, 64),
    (44, 65),
    (44, 66),
    (44, 74),
    (44, 84),
    (44, 94),
    (45, 46),
    (45, 47),
    (45, 48),
    (45, 49),
    (45, 54),
    (45, 55),
    (45, 56),
    (45, 64),
    (45, 65),
    (45, 66),
    (45, 75),
    (45, 85),
    (45, 95),
    (46, 47),
    (46, 48),
    (46, 49),
    (46, 54),
    (46, 55),
    (46, 56),
    (46, 64),
    (46, 65),
    (46, 66),
    (46, 76),
    (46, 86),
    (46, 96),
    (47, 48),
    (47, 49),
    (47, 57),
    (47, 58),
    (47, 59),
    (47, 67),
    (47, 68),
    (47, 69),
    (47, 77),
    (47, 87),
    (47, 97),
    (48, 49),
    (48, 57),
    (48, 58),
    (48, 59),
    (48, 67),
    (48, 68),
    (48, 69),
    (48, 78),
    (48, 88),
    (48, 98),
    (49, 57),
    (49, 58),
    (49, 59),
    (49, 67),
    (49, 68),
    (49, 69),
    (49, 79),
    (49, 89),
    (49, 99),
    (51, 52),
    (51, 53),
    (51, 54),
    (51, 55),
    (51, 56),
    (51, 57),
    (51, 58),
    (51, 59),
    (51, 61),
    (51, 62),
    (51, 63),
    (51, 71),
    (51, 81),
    (51, 91),
    (52, 53),
    (52, 54),
    (52, 55),
    (52, 56),
    (52, 57),
    (52, 58),
    (52, 59),
    (52, 61),
    (52, 62),
    (52, 63),
    (52, 72),
    (52, 82),
    (52, 92),
    (53, 54),
    (53, 55),
    (53, 56),
    (53, 57),
    (53, 58),
    (53, 59),
    (53, 61),
    (53, 62),
    (53, 63),
    (53, 73),
    (53, 83),
    (53, 93),
    (54, 55),
    (54, 56),
    (54, 57),
    (54, 58),
    (54, 59),
    (54, 64),
    (54, 65),
    (54, 66),
    (54, 74),
    (54, 84),
    (54, 94),
    (55, 56),
    (55, 57),
    (55, 58),
    (55, 59),
    (55, 64),
    (55, 65),
    (55, 66),
    (55, 75),
    (55, 85),
    (55, 95),
    (56, 57),
    (56, 58),
    (56, 59),
    (56, 64),
    (56, 65),
    (56, 66),
    (56, 76),
    (56, 86),
    (56, 96),
    (57, 58),
    (57, 59),
    (57, 67),
    (57, 68),
    (57, 69),
    (57, 77),
    (57, 87),
    (57, 97),
    (58, 59),
    (58, 67),
    (58, 68),
    (58, 69),
    (58, 78),
    (58, 88),
    (58, 98),
    (59, 67),
    (59, 68),
    (59, 69),
    (59, 79),
    (59, 89),
    (59, 99),
    (61, 62),
    (61, 63),
    (61, 64),
    (61, 65),
    (61, 66),
    (61, 67),
    (61, 68),
    (61, 69),
    (61, 71),
    (61, 81),
    (61, 91),
    (62, 63),
    (62, 64),
    (62, 65),
    (62, 66),
    (62, 67),
    (62, 68),
    (62, 69),
    (62, 72),
    (62, 82),
    (62, 92),
    (63, 64),
    (63, 65),
    (63, 66),
    (63, 67),
    (63, 68),
    (63, 69),
    (63, 73),
    (63, 83),
    (63, 93),
    (64, 65),
    (64, 66),
    (64, 67),
    (64, 68),
    (64, 69),
    (64, 74),
    (64, 84),
    (64, 94),
    (65, 66),
    (65, 67),
    (65, 68),
    (65, 69),
    (65, 75),
    (65, 85),
    (65, 95),
    (66, 67),
    (66, 68),
    (66, 69),
    (66, 76),
    (66, 86),
    (66, 96),
    (67, 68),
    (67, 69),
    (67, 77),
    (67, 87),
    (67, 97),
    (68, 69),
    (68, 78),
    (68, 88),
    (68, 98),
    (69, 79),
    (69, 89),
    (69, 99),
    (71, 72),
    (71, 73),
    (71, 74),
    (71, 75),
    (71, 76),
    (71, 77),
    (71, 78),
    (71, 79),
    (71, 81),
    (71, 82),
    (71, 83),
    (71, 91),
    (71, 92),
    (71, 93),
    (72, 73),
    (72, 74),
    (72, 75),
    (72, 76),
    (72, 77),
    (72, 78),
    (72, 79),
    (72, 81),
    (72, 82),
    (72, 83),
    (72, 91),
    (72, 92),
    (72, 93),
    (73, 74),
    (73, 75),
    (73, 76),
    (73, 77),
    (73, 78),
    (73, 79),
    (73, 81),
    (73, 82),
    (73, 83),
    (73, 91),
    (73, 92),
    (73, 93),
    (74, 75),
    (74, 76),
    (74, 77),
    (74, 78),
    (74, 79),
    (74, 84),
    (74, 85),
    (74, 86),
    (74, 94),
    (74, 95),
    (74, 96),
    (75, 76),
    (75, 77),
    (75, 78),
    (75, 79),
    (75, 84),
    (75, 85),
    (75, 86),
    (75, 94),
    (75, 95),
    (75, 96),
    (76, 77),
    (76, 78),
    (76, 79),
    (76, 84),
    (76, 85),
    (76, 86),
    (76, 94),
    (76, 95),
    (76, 96),
    (77, 78),
    (77, 79),
    (77, 87),
    (77, 88),
    (77, 89),
    (77, 97),
    (77, 98),
    (77, 99),
    (78, 79),
    (78, 87),
    (78, 88),
    (78, 89),
    (78, 97),
    (78, 98),
    (78, 99),
    (79, 87),
    (79, 88),
    (79, 89),
    (79, 97),
    (79, 98),
    (79, 99),
    (81, 82),
    (81, 83),
    (81, 84),
    (81, 85),
    (81, 86),
    (81, 87),
    (81, 88),
    (81, 89),
    (81, 91),
    (81, 92),
    (81, 93),
    (82, 83),
    (82, 84),
    (82, 85),
    (82, 86),
    (82, 87),
    (82, 88),
    (82, 89),
    (82, 91),
    (82, 92),
    (82, 93),
    (83, 84),
    (83, 85),
    (83, 86),
    (83, 87),
    (83, 88),
    (83, 89),
    (83, 91),
    (83, 92),
    (83, 93),
    (84, 85),
    (84, 86),
    (84, 87),
    (84, 88),
    (84, 89),
    (84, 94),
    (84, 95),
    (84, 96),
    (85, 86),
    (85, 87),
    (85, 88),
    (85, 89),
    (85, 94),
    (85, 95),
    (85, 96),
    (86, 87),
    (86, 88),
    (86, 89),
    (86, 94),
    (86, 95),
    (86, 96),
    (87, 88),
    (87, 89),
    (87, 97),
    (87, 98),
    (87, 99),
    (88, 89),
    (88, 97),
    (88, 98),
    (88, 99),
    (89, 97),
    (89, 98),
    (89, 99),
    (91, 92),
    (91, 93),
    (91, 94),
    (91, 95),
    (91, 96),
    (91, 97),
    (91, 98),
    (91, 99),
    (92, 93),
    (92, 94),
    (92, 95),
    (92, 96),
    (92, 97),
    (92, 98),
    (92, 99),
    (93, 94),
    (93, 95),
    (93, 96),
    (93, 97),
    (93, 98),
    (93, 99),
    (94, 95),
    (94, 96),
    (94, 97),
    (94, 98),
    (94, 99),
    (95, 96),
    (95, 97),
    (95, 98),
    (95, 99),
    (96, 97),
    (96, 98),
    (96, 99),
    (97, 98),
    (97, 99),
    (98, 99),
];

const CONSTRAINTS10_PLACED: &[(usize, usize)] = &[
    (17, 0x6),
    (18, 0x8),
    (25, 0x7),
    (26, 0x3),
    (29, 0x9),
    (31, 0x3),
    (33, 0x9),
    (38, 0x4),
    (39, 0x5),
    (41, 0x4),
    (42, 0x9),
    (51, 0x8),
    (53, 0x3),
    (55, 0x5),
    (57, 0x9),
    (59, 0x2),
    (68, 0x3),
    (69, 0x6),
    (71, 0x9),
    (72, 0x6),
    (77, 0x3),
    (79, 0x8),
    (81, 0x7),
    (84, 0x6),
    (85, 0x8),
    (92, 0x2),
    (93, 0x8),
];

const CONSTRAINTS11_PLACED: &[(usize, usize)] = &[
    (12, 0x6),
    (13, 0x4),
    (17, 0x7),
    (25, 0x2),
    (28, 0x3),
    (29, 0x6),
    (33, 0x1),
    (41, 0x2),
    (42, 0x3),
    (45, 0x8),
    (54, 0x7),
    (57, 0x1),
    (59, 0x4),
    (71, 0x9),
    (81, 0x8),
    (88, 0x2),
    (94, 0x4),
];

fn main() {
    let z3 = z3::Config::new();

    const VALS: usize = 81;

    let mut mins = vec![8; VALS];

    for i in 0..VALS {
        for d in 0..8 {
            println!("testing: {i} {d}");

            let ctx = z3::Context::new(&z3);

            let v = (0..VALS)
                .map(|_| ast::Int::fresh_const(&ctx, "v"))
                .collect::<Vec<_>>();

            let solver = z3::Solver::new(&ctx);

            for v in &v {
                solver.assert(&v.ge(&ast::Int::from_u64(&ctx, 0)));
                solver.assert(&v.le(&ast::Int::from_u64(&ctx, 8)));
            }

            for j in 0..i {
                solver.assert(&v[j]._eq(&ast::Int::from_u64(&ctx, mins[j])));
            }

            solver.assert(&v[i].le(&ast::Int::from_u64(&ctx, d)));

            for &(x, y) in CONSTRAINTS9 {
                let x = ((x / 10) - 1) * 9 + (x % 10 - 1);
                let y = ((y / 10) - 1) * 9 + (y % 10 - 1);
                solver.assert(&v[x]._eq(&v[y]).not());
            }

            for &(x, val) in CONSTRAINTS11_PLACED {
                let x = ((x / 10) - 1) * 9 + (x % 10 - 1);
                solver.assert(&v[x]._eq(&ast::Int::from_u64(&ctx, (val - 1) as _)));
            }

            let mut num = ast::Int::from_u64(&ctx, 0);
            for i in 0..v.len() {
                num = &num * ast::Int::from_u64(&ctx, 9) + &v[i];
            }

            // println!("solver: {solver}");
            let res = solver.check();
            if res != z3::SatResult::Sat {
                continue;
            }

            mins[i] = d;

            let model = solver.get_model().unwrap();

            let vals = v
                .iter()
                .map(|v| model.eval(v, false).unwrap().as_i64().unwrap())
                .collect::<Vec<_>>();

            println!("vals: {vals:?}");

            let mut ans = BigInt::from(0);

            for v in &vals {
                ans = &ans * 9 + v;
            }

            println!("cur: {ans}");

            break;
        }
    }
}
