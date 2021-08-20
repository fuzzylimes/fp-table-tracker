<div class="hero is-fullheight-with-navbar has-text-centered">
    <div class="hero-body">
        <div class="container">
            <div class="columns is-multiline is-vcentered is-centered is-mobile">
                <h1 class="title is-1">Loading...</h1>
                <div id="loading"></div>
            </div>
        </div>
    </div>
</div>

<style>
#loading {
  display: inline-block;
  width: 50px;
  height: 50px;
  border: 3px solid rgba(18, 129, 7, 0.3);
  border-radius: 50%;
  border-top-color: #000000;
  animation: spin 1s ease-in-out infinite;
  -webkit-animation: spin 1s ease-in-out infinite;
}

@keyframes spin {
  to { -webkit-transform: rotate(360deg); }
}
@-webkit-keyframes spin {
  to { -webkit-transform: rotate(360deg); }
}
</style>