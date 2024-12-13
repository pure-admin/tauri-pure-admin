<template>
  <el-config-provider :locale="currentLocale">
    <router-view />
    <ReDialog />
  </el-config-provider>
</template>

<script lang="tsx">
import { defineComponent } from "vue";
import { ElConfigProvider } from "element-plus";
import { ReDialog } from "@/components/ReDialog";
import zhCn from "element-plus/es/locale/lang/zh-cn";
import { addDialog } from "@/components/ReDialog";
import { name, version } from "../package.json";
import { listen } from "@tauri-apps/api/event";

export default defineComponent({
  name: "app",
  components: {
    [ElConfigProvider.name]: ElConfigProvider,
    ReDialog
  },
  computed: {
    currentLocale() {
      return zhCn;
    }
  },
  mounted() {
    // @ts-expect-error
    if (window.__TAURI_INTERNALS__) {
      listen("about-event", () => {
        addDialog({
          title: name,
          width: "40%",
          center: true,
          hideFooter: true,
          closeOnClickModal: false,
          contentRenderer: () => (
            <div class="text-center">
              <p>版本 {version}</p>
              <p class="mt-2">
                Copyright © 2020-{new Date().getFullYear()} pure-admin | MIT
                License
              </p>
            </div>
          )
        });
      });
    }
  }
});
</script>
