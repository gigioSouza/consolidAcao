import '~/assets/scss/tooltip.scss';
import uniqueId from 'lodash-es/uniqueId';

export default {
  install
}

function install(Vue) {
  Vue.directive('tooltip', {
    ids: {},
    created,
    mounted: bindTooltip,
    updated: bindTooltip,
    unmounted
  });
}

function created(el, binding) {
  const id = uniqueId('tooltip_');
  el.dataset.id = id;
  binding.dir.ids[id] = {
    tooltipElement: null,
    visible: false,
    onMouseenter: null,
    onMouseleave: null
  };
}

function bindTooltip(el, binding) {
  const ref = binding.dir.ids[el.dataset.id];
  ref.tooltipElement = buildTooltip(el, binding, ref.tooltipElement);

  if (ref.onMouseenter === null) {
    ref.onMouseenter = bindOnMouseenter(el, binding);
    ref.onMouseleave = bindOnMouseleave(el, binding);
    el.addEventListener('mouseenter', ref.onMouseenter);
    el.addEventListener('mouseleave', ref.onMouseleave);
  }
}

function buildTooltip(el, binding, tooltip) {
  if (tooltip === null) {
    tooltip = document.createElement('div');
    tooltip.appendChild(document.createTextNode(binding.value));
    tooltip.classList.add('tooltip');
    tooltip.classList.add('right'); // TODO multiple placement
  } else {
    tooltip.textContent = binding.value;
  }
  return tooltip;
}

function bindOnMouseenter(el, binding) {
  return () => {
    const ref = binding.dir.ids[el.dataset.id];
    if (ref.visible) return;
    ref.visible = true;
    const { tooltipElement } = ref;
    document.body.appendChild(tooltipElement);
    const elRect = el.getBoundingClientRect();
    const tooltipRect = tooltipElement.getBoundingClientRect();
    tooltipElement.style.position = 'absolute';
    tooltipElement.style.left = `${elRect.x + elRect.width + 8}px`;
    tooltipElement.style.top = `${elRect.y + (elRect.height/2) - (tooltipRect.height/2)}px`;
  }
}


function bindOnMouseleave(el, binding) {
  return () => {
    const ref = binding.dir.ids[el.dataset.id];
    if (!ref.visible) return;
    ref.visible = false;
    document.body.removeChild(ref.tooltipElement);
  }
}

function unmounted(el, binding) {
  const id = el.dataset.id;
  const { onMouseenter, onMouseleave } = binding.dir.ids[id];
  el.removeEventListener('mouseenter', onMouseenter);
  el.removeEventListener('mouseleave', onMouseleave);
  delete binding.dir.ids[id];
}
