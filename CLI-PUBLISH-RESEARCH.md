# fpm package 发布 CLI 调研与实施方案

> 调研对象：[`Fuyeor/fpm`](https://github.com/Fuyeor/fpm)；调研基线：`main` 分支提交 `8c45c55`（2026-06-08）。本阶段只调研和拟定方案，不修改业务代码。

## 一、结论摘要

仓库已经完成了 registry 后端的基础发布骨架、组织 scope 管理、数据库模型，以及 Web 端个人 CLI token 的生成、查看和撤销页面；但是，**CLI 本身尚不存在，且当前发布 API 不能用已生成的 `fpm_...` 个人 token 完成认证**。因此缺失的不是单纯的命令行包装，而是“个人 token 认证、CLI 打包上传、发布提交、测试和文档”这一条完整链路。

建议把 CLI 放在新的 `packages/cli/` workspace package 中，采用 **Node.js + TypeScript + ESM**。CLI 负责本地 package 读取、打包、SHA-256 计算和两阶段上传；registry 后端负责 PAT 认证、scope/版本权限校验、预签名对象存储 URL 和发布元数据落库。实施顺序应当是先补齐 PAT 认证，再实现 CLI，最后以本地 registry 或 mock HTTP 服务做端到端验证。

## 二、当前进度

| 能力 | 当前状态 | 现有实现 | 对 CLI 的意义 |
| --- | --- | --- | --- |
| Registry 后端 | 已有基础服务 | Rust 2024 + Axum，监听端口为 `6011`；路由集中在 `apps/registry/back-end/src/main.rs` | CLI 可以直接调用 HTTP API，不需要与数据库耦合 |
| Web 登录 | 已有 | `/auth/signin`、`/auth/refresh-token`，浏览器使用 cookie/JWT | CLI 不应复用浏览器 cookie 流程 |
| 个人 CLI token | Web 端已完成管理 | `/auth/token` 创建、`/auth/tokens` 列表、`DELETE /auth/tokens/{id}` 撤销；明文 token 只返回一次，数据库只保存 SHA-256 hash | 可作为 CLI 的凭据来源，但后端还没有用该 token 认证请求 |
| 组织 scope | 已有 | 组织创建和 scope 可用性校验；发布服务要求包名形如 `@scope/package`，且发布者是组织成员 | `publish` 前应在 CLI 中做格式校验，最终权限仍由后端判断 |
| Package 发布 API | 已有两阶段骨架 | `POST /packages/acquire` 返回预签名 PUT URL 和 15 分钟 upload session；`POST /packages/commit` 写入 manifest、tarball 地址和 shasum | CLI 的核心流程已经有服务端接口可对接 |
| 数据模型 | 已有基础字段 | `organization`、`package`、`package_version`、`token` 等模型已存在 | 足以支持首版发布，但暂不代表已经支持完整安装/索引消费链路 |
| CLI workspace package | 缺失 | `pnpm-workspace.yaml` 目前未包含 CLI 目录；Rust workspace 也只有 registry 后端 | 需要新增 `packages/cli/` 并加入 pnpm workspace |
| 自动化验证 | 明显不足 | 仓库没有现成 CLI 测试；根 `pnpm test` 当前报告没有测试文件；本次 `pnpm -r build` 因本地无法解析外部 workspace 依赖 `@fuyeor/config` 失败；环境中没有 `cargo` 可执行文件 | CLI 应从第一版开始补单元测试和 HTTP 流程测试，不能依赖当前空测试基线 |
| GitHub 协作状态 | 暂无额外待办 | 当前只有 `main` 分支；调研时没有开放 issue 或 pull request，也没有 release tag | CLI 需求尚未被拆分为公开协作任务 |

## 三、关键阻塞与需要推进的任务

### 1. 必须先解决 PAT 与后端认证不兼容

Web 端创建的 token 形如 `fpm_<random>`，后端把它的 hash 写入 `token` 表。但是，当前 `CurrentUser` extractor 只接受 Bearer JWT 或浏览器 `access_token` cookie，并没有查询 `token` 表。换言之，CLI 即使拿到合法的 `fpm_...` token，调用 `/packages/acquire` 仍会被当作无效 JWT 拒绝。

这是当前最明确的功能阻塞。建议扩展认证 extractor：当 `Authorization: Bearer ...` 的值以 `fpm_` 开头时，对明文 token 做 SHA-256，再按 `token_hash` 查询 `token` 表并取得 `user_id`；非 `fpm_` token 继续走现有 JWT 校验，以保持 Web 登录兼容。该路径还需要补充过期/撤销后的行为测试。

### 2. 需要定义 CLI 的凭据保存和 registry 地址约定

首版不建议让用户把 token 直接放在命令行参数中，因为 shell history 可能泄露。建议支持环境变量 `FPM_TOKEN` 用于 CI，并提供交互式 `fpm login`，将 token 保存到用户配置目录下的 JSON 文件，文件权限设为 `0600`；配置文件中只保存 registry URL 和 token，不在普通日志中回显 token。

现有前端客户端使用 `/v1` 作为 API base path，而后端源码直接注册 `/auth/...` 和 `/packages/...` 路由，说明部署层可能负责添加 `/v1` 前缀。CLI 应提供 `--registry` 与 `FPM_REGISTRY` 覆盖机制，默认值需要在实施前确认，避免把部署前缀写死在 CLI 中。

### 3. 需要明确 tarball 与 manifest 的首版契约

`acquire` 请求需要 `name`、`version`、`shasum`；`commit` 请求需要完整 `manifest`。服务端上传对象的 key 当前由 package name 和 version 组成，数据库中的 tarball 地址也按该约定生成。因此 CLI 必须保证：manifest 中的 `name` 与 `version` 是有效字符串；请求中计算的 SHA-256 与实际 PUT 的字节完全一致；打包内容默认遵循 npm package 的文件选择规则，并避免把 `.git`、依赖目录和临时文件上传。

建议首版采用“直接从 package 源目录打包”的策略，不在 CLI 内执行构建脚本；这与 README 中强调的 Direct-to-Source、Build-Free 定位一致。是否允许 `prepublish` 或构建脚本，应另行定义，不能在未确认的情况下执行任意项目脚本。

### 4. 发布提交接口需要做最小安全加固

当前 `commit` 通过 upload session JWT 恢复发布上下文，控制器本身没有显式的 `CurrentUser`；服务端也主要依赖 JWT 解码和数据库 `unwrap`。实施 CLI 时建议同时做两点：一是让 commit 请求继续携带 PAT，并校验 PAT 用户与 upload session 用户一致；二是把 JWT 解码、数据库查询、对象存储预签名和插入失败改为可处理错误，避免线上请求触发 panic。

此外，服务端应在 commit 阶段校验 manifest 的 name/version 与 session 一致，并确保同一版本不能重复提交。是否在 commit 阶段通过 HEAD 请求再次验证对象存在和大小，可作为首版安全增强；至少要保证客户端不会把未成功上传的对象提交为已发布版本。

### 5. 需要补消费侧接口，但不应阻塞首版发布 CLI

目前已看到的包路由只有 acquire 和 commit，没有公开的 package metadata、版本列表、tarball 下载或安装解析接口。因此，本次可以先把目标限定为“发布闭环”，但发布 CLI 完成后，仍需要单独拆分“registry 消费/安装协议”任务，否则 package 虽能写入数据库，用户仍未必能通过 fpm 安装和解析它。

## 四、建议的目录与技术方案

### 目录布局

建议新增如下结构，并只把 `packages/cli` 加入根 `pnpm-workspace.yaml`：

```text
packages/
  cli/
    package.json
    tsconfig.json
    src/
      cli.ts              # 进程入口与命令注册
      commands/
        login.ts
        publish.ts
        config.ts
      api/
        client.ts          # registry HTTP client
        types.ts
      auth/
        config.ts          # XDG 配置、环境变量、0600 权限
      pack/
        manifest.ts        # package.json 读取与校验
        tarball.ts         # 文件选择、tar.gz 生成、临时文件清理
        checksum.ts        # SHA-256
      output.ts             # 人类可读及 JSON 输出
    test/
      pack.test.ts
      api.test.ts
      publish.test.ts
```

不建议把 CLI 放到 `apps/registry/cli/`，因为它不是 registry 服务器的一部分，而是独立分发给 package 作者使用的客户端。也不建议首版另开 Rust CLI workspace；仓库现有 JavaScript 侧已经采用 pnpm、TypeScript、ESM，新增 Node CLI 的集成成本更低。

### 技术选型

| 领域 | 建议技术 | 选择理由 |
| --- | --- | --- |
| 运行时 | Node.js 22 LTS 兼容写法 | 与当前开发环境及 JavaScript 工具链一致 |
| 语言与模块 | TypeScript、ESM | 与现有前端 package 的语言和模块方向一致 |
| 命令解析 | Node 内置 `node:util.parseArgs`，或轻量命令解析库 | 首版命令数量少，优先减少依赖和供应链面 |
| HTTP | Node 内置 `fetch` | 不引入额外 HTTP client；支持 JSON API 和预签名 PUT |
| 打包 | `npm-packlist` + `tar`，或经过确认后调用 `npm pack --ignore-scripts` | 复用 npm package 文件选择语义，同时避免执行未授权构建脚本 |
| 摘要 | `node:crypto` SHA-256 | 与后端 `sha2`/hex 表示保持一致 |
| 配置 | XDG `~/.config/fpm/config.json`，支持 `FPM_TOKEN`、`FPM_REGISTRY` | 兼顾本地使用和 CI；token 不进入 shell history |
| 测试 | Vitest + mock HTTP server | 根 workspace 已有 Vitest，适合覆盖打包、请求顺序、错误处理 |
| 构建 | `tsc` 输出 `dist/`，`bin` 指向编译后的入口 | 便于发布为独立 npm package 或仓库内部执行 |

## 五、建议的命令与发布流程

首版建议只承诺与发布闭环直接相关的命令：`fpm login`、`fpm logout`、`fpm whoami`、`fpm publish [directory]`，以及用于 CI 的 `--token` 不落盘模式或环境变量模式。`fpm config` 可以作为轻量辅助命令，但不应把 token 以普通日志打印出来。

`fpm publish` 的流程建议如下：

1. CLI 定位 package 目录，读取 `package.json`，校验 `name`、`version`，并拒绝明显不合法或未命名的 package。
2. 根据 npm 文件选择规则生成临时 `.tgz`，默认排除 `.git`、`node_modules` 和临时产物；不执行项目脚本。
3. 对最终 tarball 计算 SHA-256，并读取完整 manifest。
4. 使用 `Authorization: Bearer fpm_...` 调用 `POST /packages/acquire`，发送 `name`、`version`、`shasum`。
5. 用返回的预签名 URL 对同一份 tarball 执行 PUT；只在 PUT 成功后进入下一步。
6. 调用 `POST /packages/commit`，发送 upload session 和 manifest；同时携带 PAT，服务端校验 session 用户与 PAT 用户一致。
7. 成功后输出 package name、version、tarball URL 和 shasum；失败时删除临时文件，不泄露 token，并把 HTTP 状态、服务端 message 和可重试性区分开。

为了方便后续演进，API client 应把 acquire、upload、commit 封装成独立函数，命令层只负责编排；这样可以在不改变 CLI 表面命令的情况下替换 registry base URL、增加重试策略或接入安装协议。

## 六、建议拆分的实施任务

| 阶段 | 任务 | 完成标准 |
| --- | --- | --- |
| A | 后端 PAT Bearer 认证 | `fpm_...` 可访问需要 `CurrentUser` 的接口；撤销 token 后立即失效；JWT/cookie 回归不受影响 |
| B | 后端 commit 安全与错误处理 | session、用户、manifest、重复版本关系被校验；错误返回 HTTP response 而不是 panic |
| C | 新建 `packages/cli` | workspace 可安装、TypeScript 可构建、入口命令可运行、配置读写有测试 |
| D | 实现打包和 checksum | 对 fixture package 生成确定性 tarball 或至少稳定的内容集合；SHA-256 与实际上传字节一致 |
| E | 实现 publish 编排 | acquire → PUT → commit 顺序正确；任一步失败都能给出可诊断结果 |
| F | 端到端验证与文档 | 用 mock/local registry 验证成功、重复版本、无权限、token 撤销、PUT 失败和 commit 失败；README 增加安装、login、publish、CI 示例 |
| G | 后续消费协议 | 单独设计 metadata、版本解析、下载和安装，不与首版 publish CLI 混在一个变更中 |

## 七、建议在实施前确认的决策

我建议采用上述默认方案，尤其是 **`packages/cli` + Node.js/TypeScript + `fpm_` PAT Bearer 认证 + 两阶段 acquire/PUT/commit**。开始编码前需要你确认四点：

1. 是否同意 CLI 放在 `packages/cli/`，包名暂定为 `@fuyeor/fpm-cli`，命令名为 `fpm`。
2. 是否同意先修改后端认证，使 Web 端生成的 `fpm_...` token 能用于 Bearer 认证；这属于 CLI 发布闭环的必要配套，不是可选优化。
3. 首版是否只实现 `login/logout/whoami/publish`，暂不实现 package 安装与消费协议。
4. registry 的默认公开地址和 `/v1` 部署前缀是否已经确定；若未确定，我会把它设计为必须可由 `--registry` 或 `FPM_REGISTRY` 覆盖，并避免在代码中硬编码不确定的生产 URL。

## References

[1]: https://github.com/Fuyeor/fpm/blob/8c45c55/README.md "fpm README"
[2]: https://github.com/Fuyeor/fpm/blob/8c45c55/pnpm-workspace.yaml "pnpm workspace configuration"
[3]: https://github.com/Fuyeor/fpm/blob/8c45c55/apps/registry/back-end/src/main.rs "Registry route assembly"
[4]: https://github.com/Fuyeor/fpm/blob/8c45c55/apps/registry/back-end/src/modules/package/controller.rs "Package upload controller"
[5]: https://github.com/Fuyeor/fpm/blob/8c45c55/apps/registry/back-end/src/modules/package/dto.rs "Package upload DTOs"
[6]: https://github.com/Fuyeor/fpm/blob/8c45c55/apps/registry/back-end/src/modules/package/service.rs "Package upload service"
[7]: https://github.com/Fuyeor/fpm/blob/8c45c55/apps/registry/back-end/src/modules/auth/middleware.rs "CurrentUser authentication extractor"
[8]: https://github.com/Fuyeor/fpm/blob/8c45c55/apps/registry/back-end/src/modules/auth/service.rs "Personal token service"
[9]: https://github.com/Fuyeor/fpm/blob/8c45c55/apps/registry/back-end/src/utils/token.rs "Personal token generation and hashing"
[10]: https://github.com/Fuyeor/fpm/blob/8c45c55/packages/prisma/schema.prisma "Registry Prisma schema"
[11]: https://github.com/Fuyeor/fpm/blob/8c45c55/apps/registry/front-end/src/api/index.ts "Frontend API base path"
[12]: https://github.com/Fuyeor/fpm/commits/main "fpm commit history"

## 八、已确认范围与实施结果

经过讨论，本次目标收敛为：先在 staging 打通公开 registry 的发布和 pnpm 安装闭环，再尽快上线；支持任意合法 scoped package，而不是仅支持 `@fuyeor`；下载公开，发布继续沿用组织成员权限；首期发布源码而不是构建产物；首批 package 来自 sibling `monorepo/packages/*`；生产 API 地址为 `https://fpm.fuyeor.com/v1`，R2 公开 tarball 使用 `https://fpm.fuyeor.net/packages/@scope/name/version.tgz`。

当前 feature branch 已实现以下内容：后端支持 `fpm_...` PAT Bearer 认证并保留 JWT/cookie；发布 commit 会校验 token 用户、upload session、manifest name/version、SemVer、SHA-256 和对象存储对象存在；新增 npm abbreviated metadata、`latest` SemVer 计算和 `/health`；CLI 位于 `packages/cli/`，使用 Node.js/TypeScript/ESM、原生 fetch、`tar-stream`/zlib、`.gitignore` 与 `files` 规则，并自动把 monorepo `workspace:*` 依赖转换为实际版本；新增 Dockerfile、staging compose 和部署文档。

验证结果包括：Rust `cargo fmt --check`、`cargo check`、3 个后端单元测试通过；CLI 在 Node.js 24.19.0 + pnpm 11.22.0 下构建通过，CLI 单元测试 3 个通过；真实 CLI publish e2e 通过；本地 npm-compatible metadata/tarball server 下，`pnpm add`、`pnpm add @package@version` 和重复 `pnpm install` e2e 均通过；fpm 前端生产构建通过。当前 sandbox 没有 Docker daemon，因此尚未在本地执行真实 Docker image build，也没有连接 staging PostgreSQL/R2 做真实发布；`STAGING.md` 中列出了部署主机上的验证步骤。
