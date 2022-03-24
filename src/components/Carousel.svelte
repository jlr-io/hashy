<script lang="ts">
  import Card from './Card.svelte';
  import type { FileHash } from "../stores/hash.store";
  export let files: FileHash[];

  const nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
</script>

<!-- {#if fileHashes} -->

<div class="carousel carousel-center rounded-box w-full">
  {#each files as file, i}
    <div id="slide{i}" class="carousel-item relative w-full">
      
        {i}
        <Card file={file} />
        <div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2">
          <a href="#slide{i-1}" class="btn btn-circle">❮</a> 
          <a href="#slide{i+1}" class="btn btn-circle">❯</a>
        </div>
      </div>
  {/each}
  <!-- {#each fileHashes as file, i}
    <div class="carousel-item">
      <div id="slide{i}">
        <Card fileHash={file} />
      </div>
      

    <div
    class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2"
  >
    <a href="slide{i + 1}" class="btn btn-circle">❮</a>
    <a href="slide{fileHashes.length - i}" class="btn btn-circle">❯</a>
  </div>
    </div>
  {/each} -->
</div>
  
  <!-- {fileHashes} -->
<!-- {/if} -->
<!-- 
<div class="carousel w-full">
  <div id="slide1" class="carousel-item relative w-full justify-center">
    <div class="card w-96 bg-base-100 shadow-xl">
      <div class="card-body">
        <h2 class="card-title">Card title!</h2>
        <p>If a dog chews shoes whose shoes does he choose?</p>
        <div class="card-actions justify-end">
          <button class="btn btn-primary">Buy Now</button>
        </div>
      </div>
    </div>

    <div
      class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2"
    >
      <a href="#slide4" class="btn btn-circle">❮</a>
      <a href="#slide2" class="btn btn-circle">❯</a>
    </div>
  </div>
  <div id="slide2" class="carousel-item relative w-full">
    <div
      class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2"
    >
      <a href="#slide1" class="btn btn-circle">❮</a>
      <a href="#slide3" class="btn btn-circle">❯</a>
    </div>
  </div>
  <div id="slide3" class="carousel-item relative w-full">
    <div
      class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2"
    >
      <a href="#slide2" class="btn btn-circle">❮</a>
      <a href="#slide4" class="btn btn-circle">❯</a>
    </div>
  </div>
  <div id="slide4" class="carousel-item relative w-full">
    <div
      class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2"
    >
      <a href="#slide3" class="btn btn-circle">❮</a>
      <a href="#slide1" class="btn btn-circle">❯</a>
    </div>
  </div>
</div> -->
