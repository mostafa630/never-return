use std::collections::HashSet;

use std::cmp::min;
#[derive(Copy, Clone)]
pub struct Landmark{
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub is_inside: bool,
}

impl Landmark {
    pub fn new(id: i32, x: i32, y:i32, is_inside: bool) -> Landmark{
        Landmark{
            id,
            x,
            y,
            is_inside
        }
    }
}

/// <summary>
/// Find the shortest path from "goerge" to any of the landmarks that is outside the Honor Stone 
/// </summary>
/// <param name="landmarks">list of Landmarks, each with Id, x, y, IsInside </param>
/// <param name="trails">list of all trails, each consists of landmark1, landmark2, length</param>
/// <param name="N">number of landmarks</param>
/// <returns>value of the shortest path from goerge to outside </returns>
pub fn required_function(landmarks: Vec<Landmark>, trails: Vec<(i32, i32, i32)>, n: i32) -> i32{
    //data structures used
    let n = landmarks.len() + 2;
    let mut graph: Vec<Vec<(i32 ,i32)>> = vec![Vec::new(); n];
    let mut stack: Vec<usize> = Vec::new();
    let mut visited:HashSet<usize> = HashSet::new();
    //1_construct the Graph
    //2-apply topological sort 
    //3-relax all edges start from the source(node 0) to get teh min cost
    construct_graph(&mut graph, &landmarks, &trails);
    let source:usize = 0 ;
    topological_sort(&graph,&source, &mut stack ,&mut visited);
    let min_cost = relax(n, &graph, &stack,&landmarks);
    return min_cost as i32;
}

fn construct_graph(graph :&mut Vec<Vec<(i32 , i32)>> , landmarks: &Vec<Landmark> , trails: &Vec<(i32, i32, i32)>){

    for (node1 ,node2 ,cost) in trails {
        let x0 =  landmarks[0].x;
        let y0 = landmarks[0].y;
        //node1 info
        let id1 = *node1 as usize;
        let x1 = landmarks[id1].x;
        let y1 = landmarks[id1].y;
        //node2 info
        let id2 = *node2 as usize;
        let x2 = landmarks[id2].x;
        let y2 = landmarks[id2].y;
        //calculate distance from node 0 to node1 
        let term1:f32 = (x0-x1).pow(2) as f32;
        let term2 :f32 = (y0-y1).pow(2) as f32;
        let dist_0_to_node1 =(term1 + term2).sqrt();
        //calculate distance from node 0 to node2
        let term1:f32 = (x0-x2).pow(2) as f32;
        let term2 :f32 = (y0-y2).pow(2) as f32;
        let dist_0_to_node2 =(term1 + term2).sqrt();
        //construct the graph edges
        let cost = *cost;
        if dist_0_to_node1 < dist_0_to_node2 {
            graph[id1].push((*node2 ,cost));
        }
        else {
            graph[id2].push((*node1 ,cost));
        }
    }
}

fn topological_sort(graph :&Vec<Vec<(i32 , i32)>>,source:&usize ,stack : &mut Vec<usize> ,
  visited:&mut HashSet<usize>){
     visited.insert(*source);
    for &node in &graph[*source]{
        let nod = node.0 as usize;
        if visited.contains(&nod)==false{
          topological_sort(graph, &nod, stack, visited)
        }
    }
    stack.push(*source);
}
fn relax(size :usize , graph:&Vec<Vec<(i32 , i32)>> ,stack : &Vec<usize>,landmarks: &Vec<Landmark> )->i64{
    let mut min_cost = i32::MAX as i64 ;
    let infinity :i64 = i32::MAX as i64;
    let mut dis:Vec<i64> = vec![infinity;size];
    dis[0]=0;
    for &top_node in stack.iter().rev() {
        for edge in &graph[top_node]{
            let source = top_node as usize;
            let destination = edge.0 as usize; 
            let path_cost = edge.1;
            //update the cost
            let new_cost:i64 = dis[source] as i64 + path_cost as i64 ;
            dis[destination] = min(dis[destination] , new_cost );
            //minimize the cost of all destenations that are outside
            if landmarks[destination].is_inside ==false{
                min_cost =min(min_cost ,dis[destination] );
            }
        }
    }
    return min_cost;
}