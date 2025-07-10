# 快速开始指南

## 运行项目

1. 确保已安装 Rust 和 Trunk
2. 在项目目录运行：
   ```bash
   trunk serve
   ```
3. 打开浏览器访问 http://127.0.0.1:8080

## 快速定制

### 修改品牌名称和内容
编辑 `src/main.rs` 文件顶部的配置：

```rust
const CONFIG: ProductConfig = ProductConfig {
    brand_name: "你的产品名称",           // 修改这里
    hero_title: "你的主标题",            // 修改这里
    hero_subtitle: "你的产品描述...",     // 修改这里
};
```

### 修改功能特性
编辑 `src/main.rs` 中的功能特性：

```rust
const FEATURES: &[(&str, &str, &str)] = &[
    ("🚀", "特性1", "特性1描述"),
    ("🛡️", "特性2", "特性2描述"), 
    ("📱", "特性3", "特性3描述"),
    // 添加更多特性...
];
```

### 修改颜色主题
编辑 `style.css` 文件中的 CSS 变量：

```css
:root {
  --color-primary: #你的主色调;
  --gradient-primary: linear-gradient(你的渐变);
  --gradient-secondary: linear-gradient(你的渐变);
}
```

### 构建生产版本
```bash
trunk build --release
```

生成的文件在 `dist/` 目录中，可以直接部署到任何静态文件服务器。

## 项目特点

✅ 现代响应式设计  
✅ 基于 Rust + Leptos 高性能  
✅ 简洁易定制的代码结构  
✅ 包含邮箱收集功能  
✅ 支持移动设备  
✅ 平滑滚动和动画效果
