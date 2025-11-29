
Write an efficient function which divides the list into two equal
sublists and put the second sublist at front of first sublist in single
pass only. If the number of elements is odd, extra element go into the
first sublist. What is the space complexity of your solution?
Function Prototype:
void SplitList(Node head)
Input: 2 4 5 3 8 7 6 1 9
Output: 7 6 1 9 2 4 5 3 8
Input: 1 3 5 7 2 4 6 8
Output: 2 4 6 8 1 3 5 7


#[derive(Debug)]
pub struct Node
{
   val :usize,
   next : Option<Box<Node>>,
}
fn split_node(head :&Node) ->()
{


}
