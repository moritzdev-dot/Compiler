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
  const nodes = data.map((item) => ({ id: item.id, label: item.value || `Node ${item.id}` }));
  const links = [];

  data.forEach((item) => {
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
    f = [{
      id: exp,
      value: expression,
    }]
    return f;
  }
  return [];
}

function parse_stmt(stmt, program, list) {
  if (stmt.IfElseStatement) {

  } else if (stmt.FuncStatement) {
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
  const program = data.program;
  console.log(program);
  const stmt = data.stmt;
  let list = [];
  parse_stmt(stmt, program, list);
  console.log(list)
  data = list;

  visualization.innerHTML = '';
  const graphData = convertToGraph(data);

  const width = visualization.clientWidth;
  const height = visualization.clientHeight;

  const svg = d3.select(visualization).append("svg")
    .attr("width", width)
    .attr("height", height);

  // Create hierarchical layout
  const stratify = d3.stratify()
    .id(d => d.id)
    .parentId(d => {
      // Find parent by looking through links
      const link = graphData.links.find(link => link.target === d.id);
      return link ? link.source : null;
    });

  // Convert flat data to hierarchical structure
  const hierarchicalData = stratify(graphData.nodes);

  // Create tree layout
  const treeLayout = d3.tree()
    .size([width - 100, height - 100]);

  const root = treeLayout(hierarchicalData);

  // Create links
  const link = svg.append("g")
    .attr("class", "links")
    .selectAll("line")
    .data(root.links())
    .enter().append("line")
    .attr("class", "link")
    .attr("transform", `translate(50, 50)`); // Add some padding

  // Create nodes
  const node = svg.append("g")
    .attr("class", "nodes")
    .selectAll("g")
    .data(root.descendants())
    .enter().append("g")
    .attr("transform", d => `translate(${d.x + 50}, ${d.y + 50})`); // Add same padding

  node.append("circle")
    .attr("r", 10);

  node.append("text")
    .text(d => d.data.label)
    .attr("x", 15)
    .attr("y", 5);

  // Update link positions
  link
    .attr("x1", d => d.source.x)
    .attr("y1", d => d.source.y)
    .attr("x2", d => d.target.x)
    .attr("y2", d => d.target.y);
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
    const graphData = await response.json();
    renderGraph(graphData);
  } catch (error) {
    console.error('Error processing code:', error);
    alert('Error processing code. Please check your syntax.');
  }
}

document.getElementById('processButton').addEventListener('click', fetchGraphData);
