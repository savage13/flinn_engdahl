const LLINDX: [(usize,usize); 364] = [
 (1, 24),
 (25, 26),
 (51, 26),
 (77, 27),
 (104, 32),
 (136, 29),
 (165, 29),
 (194, 27),
 (221, 28),
 (249, 29),
 (278, 35),
 (313, 37),
 (350, 35),
 (385, 33),
 (418, 28),
 (446, 28),
 (474, 28),
 (502, 28),
 (530, 28),
 (558, 27),
 (585, 26),
 (611, 25),
 (636, 28),
 (664, 28),
 (692, 30),
 (722, 32),
 (754, 31),
 (785, 34),
 (819, 29),
 (848, 27),
 (875, 26),
 (901, 28),
 (929, 30),
 (959, 31),
 (990, 30),
 (1020, 33),
 (1053, 35),
 (1088, 30),
 (1118, 35),
 (1153, 30),
 (1183, 33),
 (1216, 32),
 (1248, 32),
 (1280, 28),
 (1308, 26),
 (1334, 24),
 (1358, 25),
 (1383, 22),
 (1405, 21),
 (1426, 25),
 (1451, 29),
 (1480, 30),
 (1510, 29),
 (1539, 24),
 (1563, 21),
 (1584, 16),
 (1600, 15),
 (1615, 14),
 (1629, 13),
 (1642, 14),
 (1656, 11),
 (1667, 11),
 (1678, 10),
 (1688, 10),
 (1698, 10),
 (1708, 10),
 (1718, 10),
 (1728, 11),
 (1739, 13),
 (1752, 12),
 (1764, 13),
 (1777, 11),
 (1788, 9),
 (1797, 11),
 (1808, 12),
 (1820, 10),
 (1830, 9),
 (1839, 12),
 (1851, 8),
 (1859, 8),
 (1867, 8),
 (1875, 8),
 (1883, 5),
 (1888, 5),
 (1893, 5),
 (1898, 5),
 (1903, 5),
 (1908, 4),
 (1912, 1),
 (1913, 1),
 (1914, 1),
 (1915, 19),
 (1934, 23),
 (1957, 26),
 (1983, 23),
 (2006, 24),
 (2030, 23),
 (2053, 24),
 (2077, 25),
 (2102, 26),
 (2128, 28),
 (2156, 24),
 (2180, 19),
 (2199, 22),
 (2221, 19),
 (2240, 19),
 (2259, 23),
 (2282, 23),
 (2305, 28),
 (2333, 29),
 (2362, 29),
 (2391, 22),
 (2413, 23),
 (2436, 21),
 (2457, 19),
 (2476, 18),
 (2494, 18),
 (2512, 19),
 (2531, 21),
 (2552, 20),
 (2572, 21),
 (2593, 22),
 (2615, 23),
 (2638, 21),
 (2659, 23),
 (2682, 24),
 (2706, 21),
 (2727, 25),
 (2752, 26),
 (2778, 26),
 (2804, 25),
 (2829, 22),
 (2851, 20),
 (2871, 21),
 (2892, 22),
 (2914, 23),
 (2937, 23),
 (2960, 26),
 (2986, 21),
 (3007, 19),
 (3026, 19),
 (3045, 18),
 (3063, 21),
 (3084, 20),
 (3104, 20),
 (3124, 21),
 (3145, 22),
 (3167, 23),
 (3190, 22),
 (3212, 20),
 (3232, 19),
 (3251, 17),
 (3268, 12),
 (3280, 17),
 (3297, 16),
 (3313, 14),
 (3327, 14),
 (3341, 12),
 (3353, 12),
 (3365, 11),
 (3376, 10),
 (3386, 10),
 (3396, 11),
 (3407, 8),
 (3415, 8),
 (3423, 8),
 (3431, 7),
 (3438, 7),
 (3445, 5),
 (3450, 5),
 (3455, 5),
 (3460, 7),
 (3467, 4),
 (3471, 4),
 (3475, 4),
 (3479, 3),
 (3482, 3),
 (3485, 3),
 (3488, 3),
 (3491, 1),
 (3492, 1),
 (3493, 1),
 (3494, 26),
 (3520, 28),
 (3548, 30),
 (3578, 30),
 (3608, 28),
 (3636, 29),
 (3665, 28),
 (3693, 30),
 (3723, 28),
 (3751, 31),
 (3782, 29),
 (3811, 31),
 (3842, 27),
 (3869, 22),
 (3891, 22),
 (3913, 23),
 (3936, 23),
 (3959, 24),
 (3983, 23),
 (4006, 22),
 (4028, 22),
 (4050, 22),
 (4072, 20),
 (4092, 20),
 (4112, 19),
 (4131, 24),
 (4155, 23),
 (4178, 18),
 (4196, 19),
 (4215, 20),
 (4235, 18),
 (4253, 19),
 (4272, 19),
 (4291, 21),
 (4312, 23),
 (4335, 20),
 (4355, 19),
 (4374, 17),
 (4391, 17),
 (4408, 16),
 (4424, 15),
 (4439, 17),
 (4456, 14),
 (4470, 14),
 (4484, 15),
 (4499, 16),
 (4515, 16),
 (4531, 15),
 (4546, 15),
 (4561, 14),
 (4575, 15),
 (4590, 15),
 (4605, 13),
 (4618, 13),
 (4631, 11),
 (4642, 10),
 (4652, 10),
 (4662, 10),
 (4672, 10),
 (4682, 10),
 (4692, 8),
 (4700, 6),
 (4706, 6),
 (4712, 6),
 (4718, 6),
 (4724, 11),
 (4735, 8),
 (4743, 6),
 (4749, 6),
 (4755, 2),
 (4757, 4),
 (4761, 3),
 (4764, 3),
 (4767, 3),
 (4770, 3),
 (4773, 3),
 (4776, 3),
 (4779, 4),
 (4783, 3),
 (4786, 1),
 (4787, 1),
 (4788, 1),
 (4789, 1),
 (4790, 1),
 (4791, 1),
 (4792, 1),
 (4793, 1),
 (4794, 1),
 (4795, 1),
 (4796, 1),
 (4797, 1),
 (4798, 22),
 (4820, 23),
 (4843, 21),
 (4864, 20),
 (4884, 19),
 (4903, 20),
 (4923, 20),
 (4943, 22),
 (4965, 22),
 (4987, 22),
 (5009, 22),
 (5031, 22),
 (5053, 21),
 (5074, 21),
 (5095, 21),
 (5116, 18),
 (5134, 18),
 (5152, 17),
 (5169, 18),
 (5187, 20),
 (5207, 19),
 (5226, 19),
 (5245, 20),
 (5265, 20),
 (5285, 20),
 (5305, 22),
 (5327, 19),
 (5346, 18),
 (5364, 17),
 (5381, 19),
 (5400, 20),
 (5420, 19),
 (5439, 18),
 (5457, 18),
 (5475, 19),
 (5494, 18),
 (5512, 17),
 (5529, 17),
 (5546, 17),
 (5563, 17),
 (5580, 16),
 (5596, 14),
 (5610, 14),
 (5624, 12),
 (5636, 12),
 (5648, 11),
 (5659, 12),
 (5671, 12),
 (5683, 12),
 (5695, 15),
 (5710, 15),
 (5725, 15),
 (5740, 15),
 (5755, 15),
 (5770, 15),
 (5785, 14),
 (5799, 13),
 (5812, 12),
 (5824, 11),
 (5835, 11),
 (5846, 10),
 (5856, 9),
 (5865, 5),
 (5870, 6),
 (5876, 5),
 (5881, 5),
 (5886, 5),
 (5891, 5),
 (5896, 5),
 (5901, 5),
 (5906, 5),
 (5911, 5),
 (5916, 7),
 (5923, 8),
 (5931, 5),
 (5936, 4),
 (5940, 4),
 (5944, 2),
 (5946, 1),
 (5947, 1),
 (5948, 1),
 (5949, 1),
 (5950, 1),
 (5951, 1),
 (5952, 1),
 (5953, 1),
 (5954, 1),
 (5955, 1),
 (5956, 1),
 (5957, 1),
 (5958, 1),
];
