import './App.css';
import { useState, useEffect } from 'react';
import { useQuery } from '@tanstack/react-query';
import axios from 'axios';

const App = () => {
  const [query, setQuery] = useState('');
  const [debouncedQuery, setDebouncedQuery] = useState('');

  useEffect(() => {
    const handler = setTimeout(() => {
      setDebouncedQuery(query);
    }, 500);

    return () => {
      clearTimeout(handler);
    };
  }, [query]);

  const { data: results, isLoading, isError } = useQuery({
    queryKey: ['locks', debouncedQuery],
    queryFn: async () => {
      const response = await axios.put('http://localhost:5430/locks', {
        query: debouncedQuery,
        schema: null,
        relation: null,
      });
      return response.data;
    },
    enabled: !!debouncedQuery,
  });

  return (
    <div className="flex h-screen">
      <div className="w-1/2 p-4 border-r border-gray-300">
        <h2 className="text-xl font-bold mb-4">SQL Query</h2>
        <textarea
          className="w-full h-full border rounded p-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="Write your SQL query here..."
          value={query}
          onChange={(e) => setQuery(e.target.value)}
        ></textarea>
      </div>

      <div className="w-1/2 p-4">
        <h2 className="text-xl font-bold mb-4">Results</h2>
        <div className="h-full border rounded p-2 overflow-auto">
          {isLoading && <p className="text-gray-500">Loading...</p>}
          {isError && <p className="text-red-500">Error fetching results.</p>}
          {results && results.length > 0 ? (
            <ul>
              {results.map((result, index) => (
                <li key={index} className="mb-2">
                  Lock of type '{result.locktype}' with mode '{result.mode}' on relation '{result.schema}.{result.relation}'
                </li>
              ))}
            </ul>
          ) : (
            !isLoading && <p className="text-gray-500">No results to display.</p>
          )}
        </div>
      </div>
    </div>
  );
};

export default App;
