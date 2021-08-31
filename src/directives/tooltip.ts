import '~/assets/scss/tooltip.scss';
import uniqueId from 'lodash-es/uniqueId';
import { App } from 'vue';
import { DirectiveBinding } from '@vue/runtime-core';

export default {
  install
}

interface TooltipDefinition {
  element: null | HTMLElement,
  visible: boolean,
  onMouseenter: null | ((el: HTMLElement, binding: DirectiveBinding) => void),
  onMouseleave: null | ((el: HTMLElement, binding: DirectiveBinding) => void)
}

const instances: Map<string, TooltipDefinition> = new Map<string, TooltipDefinition>();

function install(Vue: App) {
  Vue.directive('tooltip', {
    created,
    mounted: bindTooltip,
    updated: bindTooltip,
    unmounted
  });
}

function created(el: HTMLElement) {
  const id = uniqueId('tooltip_');
  el.dataset.id = id;
  instances.set(id, {
    element: null,
    visible: false,
    onMouseenter: null,
    onMouseleave: null
  });
}

function bindTooltip(el: HTMLElement, binding: DirectiveBinding) {
  const instance = instances.get(el.dataset.id);
  buildTooltip(instance, binding);

  if (instance.onMouseenter == null) {
    instance.onMouseenter = bindOnMouseenter(el, binding);
    instance.onMouseleave = bindOnMouseleave(el, binding);
    el.addEventListener('mouseenter', instance.onMouseenter);
    el.addEventListener('mouseleave', instance.onMouseleave);
  }
}

function buildTooltip(instance: TooltipDefinition, binding: DirectiveBinding) {
  if (instance.element == null) {
    instance.element = document.createElement('div');
    instance.element.appendChild(document.createTextNode(binding.value));
    instance.element.classList.add('tooltip');
    instance.element.classList.add('right'); // TODO multiple placement
  } else {
    instance.element.textContent = binding.value;
  }
}

function bindOnMouseenter(el: HTMLElement) {
  return () => {
    const instance = instances.get(el.dataset.id);
    if (instance.visible) return;
    instance.visible = true;
    const { element } = instance;
    document.body.appendChild(element);
    const elRect = el.getBoundingClientRect();
    const tooltipRect = element.getBoundingClientRect();
    element.style.position = 'absolute';
    element.style.left = `${elRect.x + elRect.width + 8}px`;
    element.style.top = `${elRect.y + (elRect.height/2) - (tooltipRect.height/2)}px`;
  }
}


function bindOnMouseleave(el: HTMLElement) {
  return () => {
    const instance = instances.get(el.dataset.id);
    if (!instance.visible) return;
    instance.visible = false;
    document.body.removeChild(instance.element);
  }
}

function unmounted(el: HTMLElement) {
  const id = el.dataset.id;
  const { onMouseenter, onMouseleave } = instances.get(id);
  el.removeEventListener('mouseenter', onMouseenter);
  el.removeEventListener('mouseleave', onMouseleave);
  instances.delete(id);
}
