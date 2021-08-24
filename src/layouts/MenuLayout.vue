<script lang="ts" setup>
import { useRoute } from 'vue-router';
import { useMenu } from '../store/menu';

const route = useRoute();
const menu = useMenu();
</script>

<template>
  <div class="layout">
    <nav class="menu" :class="{ collapsed: menu.isCollapsed, expanded: menu.isExpanded }">
      <div class="top">
        <router-link :to="{ name: 'home' }" class="brand link">
          <div class="content-wrapper">
            <span class="text">ConsolidAção</span>
            <i-icon-park-outline-stock-market class="icon"/>
          </div>
        </router-link>

        <router-link :to="{ name: 'brokers' }" class="item link" v-tooltip="'Corretoras'">
          <div class="content-wrapper">
            <span class="text">Corretoras</span>
            <i-mdi-bank class="icon"/>
          </div>
        </router-link>

        <hr class="separator"/>

        <router-link :to="{ name: 'brokerage-list' }" class="item link" v-tooltip="'Notas de corretagem'">
          <div class="content-wrapper">
            <span class="text">Corretagem</span>
            <i-uil-bill class="icon"/>
          </div>
        </router-link>
      </div>

      <div class="bottom">
        <button type="button" class="collapse-button" @click="menu.collapse()">
          <i-mdi-arrow-collapse-left/>
        </button>
        <button type="button" class="expand-button" @click="menu.expand()">
          <i-mdi-arrow-expand-right/>
        </button>
      </div>
    </nav>
    <div class="contentContainer">
      <div class="header">
        <h1 class="title">{{ route.meta.title }}</h1>
      </div>
      <div class="content">
        <router-view/>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.layout {
  @apply w-screen h-screen flex flex-row bg-gray-100 overflow-hidden;

  .menu {
    width: 200px;
    @apply bg-light-blue-700 flex flex-col justify-between;

    .top {
      @apply flex flex-col justify-start;

      .brand,
      .item {
        @apply relative px-2 transition-all duration-300;

        .content-wrapper {
          @apply relative flex flex-row items-center justify-between text-white cursor-pointer px-2 transition-all duration-300;

          .text, .icon {
            @apply relative transition-all duration-300;
          }

          .icon {
            transition-delay: .1s;
          }
        }
      }

      .brand {
        @apply mb-4;

        .content-wrapper {
          @apply pt-2 pb-1 border-b border-white border-opacity-40;

          .text {
            @apply text-2xl;
          }

          .icon {
            @apply w-0 h-0;
          }
        }
      }

      .separator {
        width: 85%;
        height: 1px;
        @apply self-center border-none bg-white bg-opacity-70 my-2;
      }

      .item {
        &.router-link-active {
          .content-wrapper {
            @apply bg-light-blue-900 shadow-lg;
          }
        }

        &:hover:not(&.router-link-active) {
          .content-wrapper {
            @apply bg-light-blue-500;
          }
        }

        .content-wrapper {
          @apply px-2 py-0.5 rounded;

          .text {
            @apply text-xl w-4/5 font-light;
          }

          .icon {
            @apply text-lg;
          }
        }
      }
    }

    .bottom {
      @apply flex flex-row justify-end border-t border-light-blue-800;

      button {
        @apply px-4 pt-2 text-2xl text-white;

        &.collapse-button {
          @apply block border-l border-light-blue-800;
        }
        &.expand-button {
          @apply hidden;
        }
      }
    }

    &.collapsed {
      width: calc(50px + 1rem);
      animation: collapse .3s;

      .top {
        @apply items-center;

        .brand,
        .item {
          .content-wrapper {
            @apply py-2;

            .text {
              @apply hidden;
            }

            .icon {
              height: unset;
              width: unset;
              @apply block text-2xl;
            }
          }
        }
      }

      .bottom {
        button {
          &.collapse-button {
            @apply hidden;
          }
          &.expand-button {
            @apply block flex-grow;
          }
        }
      }
    }

    &.expanded {
      animation: expand .3s;
    }
  }

  .contentContainer {
    @apply h-screen flex-grow flex flex-col overflow-hidden relative;

    .header {
      height: 60px;
      @apply relative p-4 bg-white flex flex-row justify-between shadow z-60;

      .title {
        @apply text-gray-700 text-xl;
      }
    }

    .content {
      height: calc(100% - (60px));
      @apply p-4 overflow-auto relative;
    }
  }
}

@keyframes collapse {
  0% {
    width: 200px;
  }
  100% {
    width: calc(50px + 1rem);
  }
}

@keyframes expand {
  0% {
    width: calc(50px + 1rem);
  }
  100% {
    width: 200px;
  }
}
</style>
