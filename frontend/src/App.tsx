import './App.css';

const App = () => {
  return (
    <div className="flex h-screen">
      <div className="w-1/2 p-4 border-r border-gray-300">
        <h2 className="text-xl font-bold mb-4">SQL Query</h2>
        <textarea
          className="w-full h-full border rounded p-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="Write your SQL query here..."
        ></textarea>
      </div>

      <div className="w-1/2 p-4">
        <h2 className="text-xl font-bold mb-4">Results</h2>
        <div className="h-full border rounded p-2 overflow-auto">
          <p className="text-gray-500">Results will be displayed here.</p>
        </div>
      </div>
    </div>
  );
};

export default App;
