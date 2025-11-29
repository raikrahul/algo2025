
Given a linked list, where each node contains one more extra field
called as random pointer (other than the normal "next" pointer) which
could point to any other node or itself i.e., there could be loops. Write
an efficient function to duplicate this linked list.


voidclone_a_node(Node** node)
{
      struct Node* res = new Node{*(node->val), null, null};
      res->next = (*node)->next;
      (*node)->next = res;

}

struct Node* clone(Node** head)
{
   struct Node* res = NULL;
   struct Node** current = head;

   // Phase 1: Interweaving (CORRECTED)
Node* curr = head; // Single pointer, not double
while (curr != NULL) {
    Node* nextOriginal = curr->next; // Save B
    Node* clone = new Node(curr->val); // Create A_Clone
    
    curr->next = clone; // Link A -> A_Clone
    clone->next = nextOriginal; // Link A_Clone -> B
    
    curr = nextOriginal; // JUMP: Move straight to B.
}










   

    current = head;
    res = current->next;
    while(current)
    {
       
        current->next->random = current->random->next;
        
        

    }

    
    return res;
   

}
