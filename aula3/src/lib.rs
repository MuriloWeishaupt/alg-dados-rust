// src/lib.rs

pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    //Cria a pilha
    pub fn new() -> Stack<T> {
        Stack { elements: Vec::new() }
    }

    //Cria elemento no topo da pilha
    pub fn push(&mut self, value: T) {
        self.elements.push(value);
    }

    //Remove elemento no topo da pilha
    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    //Retorna referência do elemento no topo da lista
    pub fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_operations() {
        let mut stack = Stack::new();

        //Verifica se a pilha tá vazia
        assert!(stack.is_empty());

        //Adiciona elementos na pilha
        stack.push(10);
        stack.push(20);

        //Verifica o elemento no topo da pilha
         assert_eq!(stack.peek(), Some(&20));

         //Remove os elementos da pilha e verifica
         assert_eq!(stack.pop(), Some(20));
         assert_eq!(stack.pop(), Some(10));

        //A pilha deve estar vazia novamente
        assert!(stack.is_empty());  
    }
}