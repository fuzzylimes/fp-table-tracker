<nav class="navbar is-fixed-top is-info" role="navigation" aria-label="main navigation">
    <div class="container">
        <div class="navbar-brand">
            <a class="navbar-item is-size-4" href="/">
                <span class="icon-text">
                    <span class="is-hidden-desktop"><strong>Poker Tracker</strong></span>
                    <span class="is-hidden-touch"><strong>Fox in the Woods Poker Tracker</strong></span>
                </span>
            </a>
      
          <!-- svelte-ignore a11y-missing-attribute -->
          <a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false" data-target="navbarBasicExample">
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
          </a>
        </div>
      
        <div id="navbarBasicExample" class="navbar-menu">
          <div class="navbar-end">
            {#each links as link}
            <a class="navbar-item" href={link.url}>
              {link.text}
            </a>
            {/each}
          </div>
        </div>
    </div>
</nav>

<script>
  let links = [
      {
        url: "#/",
        text: "Home"
      },
      {
        url: "#/recent",
        text: "Recent"
      },
      {
        url: "#/about",
        text: "About"
      }
  ]
  

  // Hanlde the menu icon
  document.addEventListener('DOMContentLoaded', () => {

		// Get all "navbar-burger" elements
    const burger = document.querySelector('.navbar-burger');
    const links = document.querySelector('.navbar-menu');
    const items = document.querySelectorAll('.navbar-item');

    burger.addEventListener('click', () => {
      links.classList.toggle('is-active');
    })

    items.forEach(i => {
      i.addEventListener('click', () => {
        if(links.classList.contains('is-active')) {
          links.classList.toggle('is-active');
        }
      })
    })
	});
</script>