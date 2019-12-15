import math
import heapq

example_input = """10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL"""

example_2 = """9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL"""

answer_3 = 13312
example_3 = """157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"""

example_4 = """2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF"""
answer_4 = 180697

answer_5 = 2210736
example_5 = """171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX"""

# 64238153290 too high
# 1850 is too low
# 466901 too low
# 475603 wrong
# 469536

real = """1 RNQHX, 1 LFKRJ, 1 JNGM => 8 DSRGV
2 HCQGN, 1 XLNC, 4 WRPWG => 7 ZGVZL
172 ORE => 5 WRPWG
7 MXMQ, 1 SLTF => 3 JTBLB
1 DSRGV => 4 SLZF
2 HDVD, 32 LFKRJ => 4 FCZQD
9 LNRS, 18 WKMWF => 8 RNQRM
12 MWSGQ => 9 DCKC
6 SLTF, 5 XLNC => 1 KFBX
4 QNRZ, 1 QHLF, 15 FWSK => 9 SFJC
9 KFBX, 15 RPKGX, 2 QNRZ => 6 LFKRJ
8 SFJC, 6 ZQGL, 4 PFCGF => 3 THPCT
2 RNQHX, 4 PFCGF, 1 ZQGL => 6 LNRS
195 ORE => 4 PTHDF
3 FJKSL => 7 FWSK
12 KBJW, 9 MWSGQ => 9 WKMWF
3 XLNC => 5 RPKGX
188 ORE => 7 FJKSL
6 ZNPNM, 3 KHXPM, 3 TJXB => 2 HSDS
1 DGKW, 17 XLNC => 1 PFCGF
2 VRPJZ, 3 DSRGV => 5 MWSGQ
12 BJBQP, 5 XLNC => 4 HCQGN
1 GFCGF => 3 HDVD
18 TJXB, 2 THPCT, 1 WPGQN => 4 KHXPM
1 ZGVZL => 1 JNGM
3 ZGVZL => 8 KBJW
12 GFCGF => 8 BJBQP
7 MXMQ, 18 WRPWG => 9 XLNC
13 ZGVZL, 1 QNRZ => 6 RNQHX
5 HRBG, 16 QNRZ => 9 WPGQN
5 SFJC, 1 PFCGF, 1 KHXPM => 5 FXDMQ
1 KBJW, 5 BNFV, 16 XLNC, 1 JNGM, 1 PFCGF, 1 ZNPNM, 4 FXDMQ => 5 VBWCM
5 ZGVZL, 5 LFKRJ => 9 QHLF
14 JTBLB => 5 VRPJZ
4 FWSK => 9 RXHC
2 HRBG, 3 FCZQD => 8 DRLBG
9 KLXC, 23 VBWCM, 44 VPTBL, 5 JRKB, 41 PFCGF, 4 WBCRL, 20 QNRZ, 28 SLZF => 1 FUEL
1 DRLBG => 5 VPTBL
13 LNRS => 7 ZNPNM
3 WPGQN => 9 TJXB
5 GFCGF, 3 HCQGN => 5 ZQGL
1 KHXPM, 4 LMCSR, 1 QHLF, 4 WKMWF, 1 DGKW, 3 KBRM, 2 RNQRM => 4 KLXC
171 ORE => 8 ZJGSJ
3 ZJGSJ => 3 MXMQ
124 ORE => 5 SLTF
22 KHXPM, 10 FXDMQ => 6 KBRM
2 FCZQD => 8 LMCSR
7 DCKC, 8 HSDS, 7 PFCGF, 16 ZNPNM, 3 RNQRM, 3 WKMWF, 2 WBCRL, 14 RXHC => 7 JRKB
7 DCKC, 2 MWSGQ => 3 BNFV
2 ZQGL => 9 DGKW
22 WRPWG => 6 HRBG
22 KBJW, 1 KFBX, 1 THPCT => 6 WBCRL
4 WRPWG, 1 RXHC, 21 FWSK => 8 QNRZ
1 PTHDF => 8 GFCGF"""

def parse_input(input_map: str):
    mapping = {}
    for line in input_map.splitlines():
        inputs, output = line.split("=>")
        input_chems = []
        for x in inputs.split(","):
            input_chems.append(parse_chemical(x))

        output_chem = parse_chemical(output)
        mapping[output_chem] = input_chems
    return mapping

def get_required_ore(mapping):
    # contains list of reactions
    output_requirements = {}
    # contains the amount generated
    output_amounts = {}
    for amount, name in mapping.keys():
        output_amounts[name] = amount
        output_requirements[name] = mapping[(amount, name)]

    # the total amount that is required, no surplus
    required = {"FUEL": 1,}
    # the total amount that has already been created
    created = {}
    # the total surplus amount
    surplus = {}
    # priority = depth
    ingredient_set = set()
    ingredient_set.add("FUEL")
    ingredient_queue = [(1, "FUEL")]

    while len(ingredient_queue) > 0:
        print("\n------------------------------------------------------------------------")
        print(f"queue: {ingredient_queue}")
        depth, ingredient = heapq.heappop(ingredient_queue)
        print(f"ingredient {ingredient} depth: {depth}")

        if ingredient == "ORE":
            print(f"REQUIRED ORE: {required['ORE']}")
            continue
            # return required["ORE"]

        already_created = 0 if ingredient not in created else created[ingredient]
        total_required_amount = required[ingredient]
        current_surplus = 0 if ingredient not in surplus else surplus[ingredient]
        # how much created for each item
        each_created_amount = output_amounts[ingredient]

        if already_created >= total_required_amount:
            # already enough, don't need to dip into the cache
            print("---- ALREADY ENOUGH, NO CACHE NEEDED")
        elif total_required_amount <= (already_created + current_surplus):
            # if there's already enough within the surplus
            # no new ones need to be made
            surplus[ingredient] = (already_created + current_surplus) - total_required_amount
            assert surplus[ingredient] >= 0
            print("---- WITHIN SURPLUS")
            print(f"{ingredient} surplus {current_surplus} + {already_created} > {total_required_amount} new surplus {surplus[ingredient]}")
        else:
            required_amount = total_required_amount - (already_created + current_surplus)
            amount_to_create = math.ceil(required_amount / each_created_amount)
            created_amount = amount_to_create * each_created_amount

            print(f"ingredient {ingredient}")
            print(f"surplus {current_surplus} already {already_created} total required {total_required_amount}")
            print(f"required {total_required_amount} - {already_created} - {current_surplus} = {required_amount} ")

            new_surplus = (already_created + created_amount + current_surplus) - total_required_amount
            print(f"already created {already_created + created_amount + current_surplus} - required {total_required_amount} = surplus {new_surplus}")

            print(f"{ingredient} total = {already_created} + {created_amount} + {current_surplus} = {already_created + created_amount + current_surplus}")
            created[ingredient] = already_created + created_amount + current_surplus - new_surplus
            surplus[ingredient] = new_surplus

            assert created[ingredient] >= 0
            assert surplus[ingredient] >= 0
            assert created[ingredient] >= required[ingredient]

            print(f"created {created_amount} {ingredient} with a surplus of {new_surplus}")

            # get the required ingredients for this
            for required_amount, required_name in output_requirements[ingredient]:
                print(required)
                if required_name in required:
                    print(f"required for {required_name} is now {required[required_name]} + {required_amount} * {amount_to_create}")
                    required[required_name] += amount_to_create * required_amount
                else:
                    print(f"required for {required_name} is now {required_amount} * {amount_to_create}")
                    required[required_name] = amount_to_create * required_amount
                if required_name in ingredient_set:
                    # avoid duplicate
                    for i, x in enumerate(ingredient_queue):
                        if x[1] == required_name:
                            del ingredient_queue[i]
                    heapq.heappush(ingredient_queue, (depth + 1, required_name))
                else:
                    ingredient_set.add(required_name)
                    heapq.heappush(ingredient_queue, (depth + 1, required_name))
    return required["ORE"]

def get_part2(mapping, expected_min = 0):
    print("part2")
    for fuel in range(expected_min, 99999999999999999999, 1):
        result = get_required_ore_part2(mapping, fuel)
        print(f"{fuel} using {result} ore")
        if result is None or result > 1000000000000:
            # 12063757192
            # 1000000000000
            print(f"too much or none {fuel - 1}")
            return fuel - 1

def get_required_ore_part2(mapping, fuel):
    # contains list of reactions
    output_requirements = {}
    # contains the amount generated
    output_amounts = {}
    for amount, name in mapping.keys():
        output_amounts[name] = amount
        output_requirements[name] = mapping[(amount, name)]

    # the total amount that is required, no surplus
    required = {"FUEL": fuel,}
    # the total amount that has already been created
    created = {}
    # the total surplus amount
    surplus = {}
    # priority = depth
    ingredient_set = set()
    ingredient_set.add("FUEL")
    ingredient_queue = [(1, "FUEL")]

    while len(ingredient_queue) > 0:
        # print("\n------------------------------------------------------------------------")
        # print(f"queue: {ingredient_queue}")
        depth, ingredient = heapq.heappop(ingredient_queue)
        # print(f"ingredient {ingredient} depth: {depth}")

        if ingredient == "ORE":
            # print(f"REQUIRED ORE: {required['ORE']}")
            continue
            # return required["ORE"]

        already_created = 0 if ingredient not in created else created[ingredient]
        total_required_amount = required[ingredient]
        current_surplus = 0 if ingredient not in surplus else surplus[ingredient]
        # how much created for each item
        each_created_amount = output_amounts[ingredient]

        if already_created >= total_required_amount:
            # already enough, don't need to dip into the cache
            # print("---- ALREADY ENOUGH, NO CACHE NEEDED")
            pass
        elif total_required_amount <= (already_created + current_surplus):
            # if there's already enough within the surplus
            # no new ones need to be made
            surplus[ingredient] = (already_created + current_surplus) - total_required_amount
            assert surplus[ingredient] >= 0
            # print("---- WITHIN SURPLUS")
            # print(f"{ingredient} surplus {current_surplus} + {already_created} > {total_required_amount} new surplus {surplus[ingredient]}")
        else:
            required_amount = total_required_amount - (already_created + current_surplus)
            amount_to_create = math.ceil(required_amount / each_created_amount)
            created_amount = amount_to_create * each_created_amount

            # print(f"ingredient {ingredient}")
            # print(f"surplus {current_surplus} already {already_created} total required {total_required_amount}")
            # print(f"required {total_required_amount} - {already_created} - {current_surplus} = {required_amount} ")

            new_surplus = (already_created + created_amount + current_surplus) - total_required_amount
            # print(f"already created {already_created + created_amount + current_surplus} - required {total_required_amount} = surplus {new_surplus}")

            # print(f"{ingredient} total = {already_created} + {created_amount} + {current_surplus} = {already_created + created_amount + current_surplus}")
            created[ingredient] = already_created + created_amount + current_surplus - new_surplus
            surplus[ingredient] = new_surplus

            assert created[ingredient] >= 0
            assert surplus[ingredient] >= 0
            assert created[ingredient] >= required[ingredient]

            # print(f"created {created_amount} {ingredient} with a surplus of {new_surplus}")

            # get the required ingredients for this
            for required_amount, required_name in output_requirements[ingredient]:
                # print(required)
                if required_name in required:
                    # print(f"required for {required_name} is now {required[required_name]} + {required_amount} * {amount_to_create}")
                    required[required_name] += amount_to_create * required_amount
                else:
                    # print(f"required for {required_name} is now {required_amount} * {amount_to_create}")
                    required[required_name] = amount_to_create * required_amount
                if required_name in ingredient_set:
                    # avoid duplicate
                    for i, x in enumerate(ingredient_queue):
                        if x[1] == required_name:
                            del ingredient_queue[i]
                    heapq.heappush(ingredient_queue, (depth + 1, required_name))
                else:
                    ingredient_set.add(required_name)
                    heapq.heappush(ingredient_queue, (depth + 1, required_name))
    # return required["ORE"]
    if "ORE" in required:
        return required["ORE"]
    return None


def calc_required(mapping):
    required = {"FUEL": 1,}
    surplus = {}
    # active_set = set()
    active_set = []
    active_set.append("FUEL")
    # key_name = "FUEL"
    while len(active_set) > 0:
        print(required)
        # key_name = list(active_set)[0]
        key_name = active_set.pop()
        # get the key for the key name
        # key = list(filter(lambda x: x[1] == key_name, list(mapping.keys())))
        key = [x for x in mapping.keys() if x[1] == key_name]
        print(active_set)
        print(key_name)
        print(key)
        key = key[0]

        existing_surplus = 0 if key_name not in surplus else surplus[key_name]
        required_amount = required[key_name] - existing_surplus
        produced = key[0]
        multiplier = math.ceil(required_amount / produced)
        created_surplus = multiplier * (required_amount % produced)
        surplus[key_name] = created_surplus

        print(f"{key_name} produces {produced} requires {required_amount} surplus {created_surplus} mul {multiplier}")

        # remove this key from the dict
        # del active_set[key_name]
        if key_name in active_set:
            active_set.remove(key_name)

        inputs = mapping[key]
        for input_amount, input_name in inputs:
            if input_name in required:
                current = required[input_name]
                print(f"current {input_name} already has {current}, adding {input_amount * multiplier}")
                required[input_name] = input_amount * multiplier
            else:
                required[input_name] = input_amount * multiplier
            if input_name != "ORE":
                # active_set.add(input_name)
                if input_name in active_set:
                    print(f"{input_name} in {active_set}")
                    active_set.remove(input_name)
                active_set.append(input_name)
        
        # ensure that the required doesn't only contain ORE
        # if "ORE" in required:
        #     print(f"ore {ore} + {required['ORE']}")
        #     ore += required["ORE"]
        #     del required["ORE"]

    return required["ORE"]

def parse_chemical(input_chem: str):
    chem = input_chem.strip().split(" ")
    return int(chem[0]), chem[1]

if __name__ == "__main__":
    print(example_input)
    mapping = parse_input(example_input)
    print(mapping)
    result = get_required_ore(mapping)
    print(result)
    assert result == 31

    mapping = parse_input(example_2)
    assert get_required_ore(mapping) == 165

    mapping = parse_input(example_3)
    assert get_required_ore(mapping) == answer_3
    assert get_part2(mapping, 82892700) == 82892753

    mapping = parse_input(example_4)
    assert get_required_ore(mapping) == answer_4
    assert get_part2(mapping, 5586000) == 5586022

    mapping = parse_input(example_5)
    assert get_required_ore(mapping) == answer_5
    assert get_part2(mapping, 460600) == 460664

    mapping = parse_input(real)
    result = get_required_ore(mapping)
    print(f"real result: {result}")
    assert result == 469536

    # part 2
    result = get_part2(mapping, 3299999)
    # 3343477
    print(f"part2: {result}")