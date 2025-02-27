<script setup>
import { defineClientComponent } from "vitepress";

const Download = defineClientComponent(() => import("./home/Download.vue"));
</script>

<template>
    <div class="container hero">
        <h1 class="tagline">The static analysis of your Cairo code</h1>
    </div>

    <div class="features-bg">
        <div class="container features-container">
            <Download />
        </div>
    </div>

    <div class="container vp-doc">
        <h2 id="community" tabindex="-1">Community</h2>
        <p>
            Cairo lint is created by the same team at
            <a href="https://swmansion.com" rel="noreferrer" target="_blank">Software Mansion</a>
            that is behind
            <a href="https://docs.swmansion.com/scarb" rel="noreferrer" target="_blank"
                >Scarb</a
            >, the Cairo build toolchain as well as package manager.
        </p>
        Feel free chat with us on our channel on
        <a href="https://t.me/cairolint" rel="noreferrer" target="_blank">Telegram</a>.
    </div>
</template>

<style scoped>
.container {
  margin: 0 auto;
  padding: 32px 24px;
  width: 100%;
}

@media (min-width: 640px) {
  .container {
    max-width: 640px;
  }
}

@media (min-width: 768px) {
  .container {
    max-width: 768px;
  }
}

@media (min-width: 1024px) {
  .container {
    max-width: 1024px;
  }
}

.hero {
  display: flex;
  flex-direction: column;
  gap: 40px;
}

.features-bg {
  background-color: var(--vp-c-bg-alt);
}

.features-container {
  display: flex;
  flex-direction: column;
  gap: 40px;
}

.tagline {
  color: var(--vp-c-text-2);
  font-family: var(--swm-font-family-geometric-humanist);
  font-size: 2.5em;
  font-weight: 400;
  line-height: 1.2;
  margin: 0 auto;
  max-width: 48rem;
  text-align: center;
}

.vp-doc {
  padding: 32px 24px 96px;
}

.vp-doc > h2 {
  border: none;
}

.vp-doc > :first-child {
  margin-top: 0;
}
</style>
