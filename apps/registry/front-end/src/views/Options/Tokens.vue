<!-- @/views/Options/Tokens.vue -->
<template>
  <HeaderBar :title="t('token.new')" />

  <div class="content-layout">
    <HeaderSection
      :title="t('settings.tokens')"
      :description="t('settings.username.reserve.tip')"
      :action-text="t('token.new')"
      @action="openModal"
    />

    <!-- A. 加载状态 -->
    <StateDisplay v-if="isLoadingTokens" type="loading" />

    <!-- B. 空状态：还没生成过令牌 -->
    <EmptyCard
      v-else-if="isEmpty"
      :title="t('token.empty.title')"
      :message="t('token.empty.desc')"
    />

    <!-- C. 成功状态：渲染已有的令牌卡片 -->
    <ul v-else class="clients-list">
      <OptionItemCard
        v-for="item in tokens"
        :key="item.id"
        icon-text="Token"
        :item-name="item.name"
        :created-at="item.createdAt"
        :item-status="t('token.status.active')"
      >
        <!-- 自定义卡片右侧的插槽：放入吊销按钮 -->
        <button
          class="btn-revoke"
          @click="handleRevoke(item.id)"
          :disabled="isRevoking"
        >
          {{ isRevoking ? t('revoking') : t('revoke') }}
        </button>
      </OptionItemCard>
    </ul>
  </div>

  <!-- 创建令牌弹窗 -->
  <Modal v-model="showModal" strict>
    <template #header>
      <h3>{{ t('token.new') }}</h3>
    </template>

    <!-- 场景 A：令牌生成成功，展示明文 -->
    <CopyClipboard
      v-if="plainToken"
      :value="plainToken"
      :description="t('token.secret.tip')"
    />

    <!-- 场景 B：初始输入表单 -->
    <FormField
      v-else
      :label="t('token.create.name')"
      :tip="t('token.create.name.desc')"
      :error="tokenError ? t('token.create.failed') : ''"
    >
      <input
        type="text"
        v-model="tokenName"
        :disabled="isGenerating"
        @keyup.enter="handleCreate"
      />
    </FormField>

    <template #footer>
      <!-- 成功展示时只有“关闭”按钮 -->
      <button v-if="plainToken" class="btn-primary" @click="closeModal">
        {{ t('close') }}
      </button>

      <!-- 输入时有“取消”和“创建” -->
      <div v-else class="modal-actions">
        <button
          class="btn-secondary"
          @click="closeModal"
          :disabled="isGenerating"
        >
          {{ t('cancel') }}
        </button>
        <button
          class="btn-primary"
          @click="handleCreate"
          :disabled="!tokenName.trim() || isGenerating"
        >
          {{ isGenerating ? t('creating') : t('create') }}
        </button>
      </div>
    </template>
  </Modal>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useLocale } from '@fuyeor/locale';
import {
  HeaderSection,
  HeaderBar,
  StateDisplay,
  FormField,
  Modal,
  OptionItemCard,
  CopyClipboard,
} from '@fuyeor/interactify';
import {
  useCreateTokenMutation,
  useTokensQuery,
  useRevokeTokenMutation,
} from '@/composables/api/useTokens';

const { t } = useLocale();
const {
  mutateAsync: generateToken,
  isPending: isGenerating,
  error: tokenError,
} = useCreateTokenMutation();
const { data: tokens, isLoading: isLoadingTokens, isEmpty } = useTokensQuery();
const { mutateAsync: revokeToken, isPending: isRevoking } =
  useRevokeTokenMutation();

const showModal = ref(false);
const tokenName = ref('');
const plainToken = ref('');
const copied = ref(false);

const openModal = () => {
  tokenName.value = '';
  plainToken.value = '';
  copied.value = false;
  showModal.value = true;
};

const closeModal = () => {
  showModal.value = false;
};

const handleCreate = async () => {
  const name = tokenName.value.trim();
  if (!name || isGenerating.value) return;
  try {
    const res = await generateToken({ name });
    plainToken.value = res.token;
  } catch (err) {
    console.error('[Tokens] Create failed:', err);
  }
};

const handleRevoke = async (tokenId: string) => {
  if (!confirm(t('token.revoke.confirm'))) return;
  try {
    await revokeToken(tokenId);
  } catch (err) {
    console.error('[Tokens] Revoke failed:', err);
  }
};
</script>
