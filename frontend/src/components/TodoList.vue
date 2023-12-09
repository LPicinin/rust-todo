<template>
    <div>
      <h1>Todo List</h1>
      <ul>
        <li v-for="todo in todos" :key="todo.id">{{ todo.title }}</li>
      </ul>
    </div>
  </template>
  
  <script>
  import { defineComponent } from 'vue';
  
  export default defineComponent({
    name: 'TodoList',
    data() {
      return {
        todos: [], // You'll fetch and populate this data from your Actix backend
      };
    },
    // Add methods to fetch todos from your Actix backend
  });
  </script>
  
  <style scoped>
  /* Add your component styles here */
  </style>
  