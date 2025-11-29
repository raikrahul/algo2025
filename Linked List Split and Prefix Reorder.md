
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
fn split_node(head :&mut Option<Box<Node>>) ->()
{
    let mut fast = head.as_ref();
    let mut slow = head.as_ref();
    let mut temp  = None;
// Assuming fast and slow are initialized as: 
    // let mut fast = head.as_ref();
    // let mut slow = head.as_ref();

    while fast.is_some() {
        // 1. Fix: .next.as_ref()
        // We look two steps ahead safely
        let next_step = fast.and_then(|node| node.next.as_ref())
                            .and_then(|node| node.next.as_ref());

        // 2. Fix: Method call .is_some()
        if next_step.is_some() {
            fast = next_step;
            slow = slow.and_then(|n| n.next.as_ref());
        } else {
            // 3. Fix: CRITICAL infinite loop prevention
            break; 
        }
    }
    
    // At this point:
    // 'slow' points to the split point (End of first half).
    // 'fast' points to the last node (End of second half).
    head = slow;
    
    
    
}
