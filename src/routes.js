// Components
import Home from './routes/Home.svelte'
import About from './routes/About.svelte'
import Weekly from './routes/Weekly.svelte'

// Export the route definition object
export default {
    '/': Home,
    '/about': About,
    '/recent': Weekly
}