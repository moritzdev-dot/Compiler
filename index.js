const editorElement = document.getElementById('editor');
const visualization = document.getElementById('visualization');

// Initialize CodeMirror
const editor = CodeMirror.fromTextArea(editorElement, {
  mode: "javascript",
  lineNumbers: true,
  theme: "default",
  autoCloseBrackets: true,
  matchBrackets: true,
  indentUnit: 4,
  tabSize: 4,
  lineWrapping: true,
  extraKeys: {"Ctrl-Space": "autocomplete"}
});

function convertToGraph(data) {
  // Check if data is an array, if not, try to extract the list
  let nodeList = Array.isArray(data) ? data : [];
  
  // If data has program and stmt properties, process it
  if (data.program && data.stmt) {
    let list = [];
    parse_stmt(data.stmt, data.program, list);
    nodeList = list;
  }

  const nodes = nodeList.map((item) => ({ 
    id: item.id, 
    label: item.value || `Node ${item.id}` 
  }));
  
  const links = [];

  nodeList.forEach((item) => {
    if (item.children) {
      item.children.forEach((childIndex) => {
        links.push({ source: item.id, target: childIndex });
      });
    }
  });

  return { nodes, links };
}

function expression_to_node(exp, program) {
  console.log(exp);
  let expression = program[exp];
  if(expression.InfixExpression) {
    expression = expression.InfixExpression;

    const left = expression_to_node(expression.left, program);
    const right = expression_to_node(expression.right, program);
    op = expression.op;

    let obj = [{
      id: exp,
      value: op,
      children: [expression.left, expression.right]
    }];

    left.forEach((x) => {obj.push(x)});
    right.forEach((x) => {obj.push(x)});
    return obj;

  } else if (expression.Integer) {

    expression = expression.Integer;
    return [{
      id: exp,
      value: expression,
    }]
  } else if (expression.String) {
    expression = expression.String;
    return [{
      id: exp,
      value: expression,
    }]

  }
  return [];
}

function parse_stmt(stmt, program, list) {
  if (stmt.IfElseStatement) {
    throw Error("not implemented");
  } else if (stmt.FuncStatement) {
    throw Error("not implemented");
  } else {
    const exp = stmt.ExpressionStatement;
    let nodes = expression_to_node(exp, program);
    console.log(nodes);
    nodes.forEach((x) => {
      list.push(x);
    })
  }

}

function renderGraph(data) {
  visualization.innerHTML = '';
  const graphData = convertToGraph(data);

  const width = visualization.clientWidth;
  const height = visualization.clientHeight;

  const svg = d3.select(visualization).append("svg")
    .attr("width", width)
    .attr("height", height);

  // Group nodes by their root (nodes without parents)
  const getRootId = (nodeId) => {
    let currentId = nodeId;
    let link;
    while ((link = graphData.links.find(l => l.target === currentId))) {
      currentId = link.source;
    }
    return currentId;
  };

  const nodesByRoot = {};
  graphData.nodes.forEach(node => {
    const rootId = getRootId(node.id);
    if (!nodesByRoot[rootId]) {
      nodesByRoot[rootId] = [];
    }
    nodesByRoot[rootId].push(node);
  });

  // Calculate width for each tree
  const numTrees = Object.keys(nodesByRoot).length;
  const treeWidth = (width - 100) / numTrees;

  // Process each tree separately
  Object.entries(nodesByRoot).forEach(([rootId, nodes], index) => {
    // Filter links for current tree
    const treeLinks = graphData.links.filter(link => 
      nodes.some(n => n.id === link.source || n.id === link.target)
    );

    // Create hierarchical layout for this tree
    const stratify = d3.stratify()
      .id(d => d.id)
      .parentId(d => {
        const link = treeLinks.find(link => link.target === d.id);
        return link ? link.source : null;
      });

    // Convert flat data to hierarchical structure
    const hierarchicalData = stratify(nodes);

    // Create tree layout with reduced size
    const treeLayout = d3.tree()
      .size([treeWidth - 40, height - 100])  // Reduced width for each tree
      .nodeSize([40, 40]);  // Set fixed size between nodes [horizontal, vertical]

    const root = treeLayout(hierarchicalData);

    // Calculate x offset for this tree
    const xOffset = (index * treeWidth) + 50;

    // Create links
    const link = svg.append("g")
      .attr("class", "links")
      .selectAll("line")
      .data(root.links())
      .enter().append("line")
      .attr("class", "link")
      .attr("transform", `translate(${xOffset}, 50)`)
      .attr("x1", d => d.source.x)
      .attr("y1", d => d.source.y)
      .attr("x2", d => d.target.x)
      .attr("y2", d => d.target.y);

    // Create nodes
    const node = svg.append("g")
      .attr("class", "nodes")
      .selectAll("g")
      .data(root.descendants())
      .enter().append("g")
      .attr("transform", d => `translate(${d.x + xOffset}, ${d.y + 50})`);

    node.append("circle")
      .attr("r", 10);

    node.append("text")
      .text(d => d.data.label)
      .attr("x", -10)  // Center text
      .attr("y", -15)  // Move text above node
      .attr("text-anchor", "middle");  // Center text horizontally
  });
}

async function fetchGraphData() {
  try {
    const sourceCode = editor.getValue();
    const response = await fetch('/parse', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ code: sourceCode }),
    });
    const data = await response.json();
    renderGraph(data);
  } catch (error) {
    console.error('Error processing code:', error);
    alert('Error processing code. Please check your syntax.');
  }
}

document.getElementById('processButton').addEventListener('click', fetchGraphData);
