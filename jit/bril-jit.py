import sys, json, random, subprocess


def main():

    ### The orginal program itself in json format
    prog = json.load(sys.stdin)

    command = 'node ../bril-ts/build/brili.js'
    
    try:
        for arg in prog["functions"][0]["args"]:
            command = command + " " + rand_arg(arg["type"])
    except:
        ()
    
    ### The trace
    trace_str = subprocess.run([command],
                               text=True,
                               shell=True,
                               stderr=subprocess.PIPE,
                               stdout=subprocess.DEVNULL,
                               input=json.dumps(prog)).stderr

    print(command)
    print(trace_str)
    arr = [x for x in trace_str.split("\n") if x]
    pos = int(arr.pop())
    trace = [json.loads(instr) for instr in arr]
    stitch(prog, trace, pos)

    
# Stitches prog with the trace hehe
def stitch(prog, trace, pos):
    
    for func in prog['functions']:
        if func['name'] == 'main':
            main = func

    label = json.loads('{"label": "speculate-success"}')
    main['instrs'].insert(pos, label)

    spec = json.loads('{"op": "speculate"}')
    commit = json.loads('{"op": "commit"}')
    jump = json.loads('{"op": "jmp", "labels":["speculate-success"]}')
    handle = json.loads('{"label": "speculate-fail"}')
    prelude = [spec] + trace + [commit, jump, handle]

    main['instrs'] = prelude + main['instrs']

    print(json.dumps(prog, indent=1))
    
    
def rand_arg(typ):
    
    if typ == 'int':
        return str(random.randint(1, 3))
    elif typ == 'bool':
        return 'true'
    else:
        return '3.1415926'


main()
