
hello world

Write an efficient function that prints a 2-D (n x n) array in spiral
order. Your function should cover all the elements in the given array.
Function Prototype:
void printSpiralWay(char [][]a, int n)
Input:
abc
123
pq r
Output: a b c 3 r q p 1 2


fn printSprialWay(a : &[Vec<char>], n : usize)
{

   let mut left:usize = 0; 
   let mut right:usize = n -1;
   let mut top:usize = 0;
   let mut bottom:usize  = n-1;

  while   top <= bottom && left <= right          { 

   for i in left..=right {
       println("{}", a[top][i]
       }
   top = top + 1;
   for i in top..=bottom
   {
       println!("{}", a[i][right]);
       }
   right = right -1;
   for i in (left..=right).rev()
   {
       println("{}", a[bottom][i]);
   }
   bottom = bottom - 1;
   for i in (top..=bottom).rev()
   {
      println!("{}", a[i][left]);
      }
   left = left  + 1;

}







