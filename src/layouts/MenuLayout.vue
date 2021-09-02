<script lang="ts" setup>
import { useRoute } from 'vue-router';
import { useMenu } from '../store/menu';

const route = useRoute();
const menu = useMenu();

function toggle() {
  if (menu.isCollapsed) {
    menu.expand();
  } else {
    menu.collapse();
  }
}
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

        <router-link :to="{ name: 'brokers' }" class="item link" v-tooltip.right="'Corretoras'">
          <div class="content-wrapper">
            <span class="text">Corretoras</span>
            <i-mdi-bank class="icon"/>
          </div>
        </router-link>

        <hr class="separator"/>

        <router-link :to="{ name: 'brokerage-list' }" class="item link" v-tooltip.right="'Corretagem'">
          <div class="content-wrapper">
            <span class="text">Corretagem</span>
            <i-uil-bill class="icon"/>
          </div>
        </router-link>
      </div>

      <div class="bottom">
        <router-link :to="{ name: 'config' }" class="item link" v-tooltip.right="'Configurações'">
          <div class="content-wrapper">
            <span class="text">Configurações</span>
            <i-mdi-cog class="icon"/>
          </div>
        </router-link>

        <div class="toggle-button-wrapper">
          <button type="button" class="toggle-button" @click="toggle">
            <i-mdi-arrow-expand-left class="icon" />
          </button>
        </div>
      </div>
    </nav>

    <div class="contentContainer">
      <div class="header">
        <h1 class="title">{{ route.meta.title }}</h1>
        <div id="header-slot"></div>
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
    }

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

    .bottom {
      @apply flex flex-col;

      .toggle-button-wrapper {
        @apply flex flex-row justify-end border-t border-light-blue-800 mt-2;

        .toggle-button {
          @apply pr-4 pl-3.5 pt-2 text-2xl text-white block border-l border-light-blue-800;
        }
      }
    }

    &.collapsed {
      width: calc(50px + 1rem);
      animation: collapse .3s;

      .top {
        @apply items-center;
      }

      .brand,
      .item {
        .content-wrapper {
          @apply py-2 justify-center;

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

      .bottom {
        .toggle-button-wrapper {
          .toggle-button {
            @apply flex-grow;

            .icon {
              animation: collapse-toggle .3s;
              transform: rotate(180deg);
            }
          }
        }
      }
    }

    &.expanded {
      animation: expand .3s;

      .bottom {
        .toggle-button-wrapper {
          .toggle-button {
            .icon {
              animation: expand-toggle .3s;
            }
          }
        }
      }
    }
  }

  .contentContainer {
    @apply h-screen flex-grow flex flex-col overflow-hidden relative;

    .header {
      height: 60px;
      @apply relative bg-white flex flex-row justify-between shadow z-60 items-center;

      .title {
        @apply text-gray-700 text-xl p-4;
      }

      #header-slot {
        @apply flex-grow flex flex-row items-center justify-end px-4;
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

@keyframes collapse-toggle {
  0% {
    transform: rotate(0deg);
  }
  10% {
    transform: rotate(18deg);
  }
  20% {
    transform: rotate(36deg);
  }
  30% {
    transform: rotate(54deg);
  }
  40% {
    transform: rotate(72deg);
  }
  50% {
    transform: rotate(90deg);
  }
  60% {
    transform: rotate(108deg);
  }
  70% {
    transform: rotate(126deg);
  }
  80% {
    transform: rotate(144deg);
  }
  90% {
    transform: rotate(162deg);
  }
  100% {
    transform: rotate(180deg);
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

@keyframes expand-toggle {
  0% {
    transform: rotate(180deg);
  }
  10% {
    transform: rotate(162deg);
  }
  20% {
    transform: rotate(144deg);
  }
  30% {
    transform: rotate(126deg);
  }
  40% {
    transform: rotate(108deg);
  }
  50% {
    transform: rotate(90deg);
  }
  60% {
    transform: rotate(72deg);
  }
  70% {
    transform: rotate(54deg);
  }
  80% {
    transform: rotate(36deg);
  }
  90% {
    transform: rotate(18deg);
  }
  100% {
    transform: rotate(0deg);
  }
}
</style>
