<template>
  <n-dialog-provider>
    <n-notification-provider>
      <n-message-provider>
        <slot name="default"></slot>
      </n-message-provider>
    </n-notification-provider>
  </n-dialog-provider>
</template>


<script lang="ts">
import {NDialogProvider, NNotificationProvider, NMessageProvider} from 'naive-ui';

export default {
  name: "Application",
  components: {
    NDialogProvider,
    NNotificationProvider,
    NMessageProvider,
  },
  setup() {
    return {};
  }
}
</script>

<style scoped>

</style>