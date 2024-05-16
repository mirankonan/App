import './header.css'
import { useState, useEffect } from 'react';
import './CreatedTask.css';
import axios from "axios";


function Header() {
 const [tasks, setTasks] = useState([]);
 const [newTask, setNewTask] = useState("");
useEffect(() => {
  const createTasks = async () => {
    try {
      const res = await axios.post("http://localhost:8080/create");
      console.log(res.data);
      setTasks(res.data);
    } catch (error) {
      console.log(error.message);
    }
  }
  createTasks();
}, [])
const handleSubmit = async (event) => {
  event.preventDefault();
  try {
    const taskData = { task: newTask }; // Prepare the data for the new task
    const res = await axios.post("http://localhost:8080/create", taskData); // Send the POST request
    console.log(res.data);
    setTasks(prevTasks => [...prevTasks, res.data]); // Add the new task to the state
    setNewTask(""); // Clear the input field
  } catch (error) {
    console.log(error.message);
  }
}
return (
    <> 
      <header className='head'>
        <div className='todo'>Todolist</div>
        <form onSubmit={handleSubmit}>
            <input placeholder='Add New Task' type = 'text' className='task'
              value = {newTask} onChange={(e) => {setNewTask(e.target.value);}}
            />  
            <button type='button'  className='bt'
            onClick ={() => {
              setTasks(t => [...t, newTask]);
              setNewTask('');
            }}
            >Add</button>
        </form>
      </header>
      { <ol>
           {
             tasks.map((task, index) => 
                <li key = {index}>
                  <span className='div'>
                    <input type="checkbox" className="check"/>
                    {index + 1} : {task}   
                  </span>
                  <button className='deletebut'
                    onClick={() => {
                      setTasks(tasks.filter((_, taskIndex) => taskIndex !== index));
                    }}
                  >Delete</button>
                </li>
              
             )
           }
      </ol> 
      }
    </>
  );
}
export default Header;
