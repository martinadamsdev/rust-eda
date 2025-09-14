# 阶段二：原理图编辑器详细规划

## 📅 时间线：2025年4月 - 2025年6月

## 🎯 阶段目标

在基础架构之上，实现完整的电路原理图设计功能，包括元件库管理、智能连线、电气规则检查等核心功能，达到可以设计实用电路的水平。

## 🔧 核心功能模块

### 1. 元件库系统

#### 元件数据结构

```typescript
interface Component {
  id: string
  name: string
  category: ComponentCategory
  symbol: Symbol
  pins: Pin[]
  properties: Property[]
  footprint?: string
  datasheet?: string
  supplier?: SupplierInfo[]
}

interface Pin {
  id: string
  name: string
  number: string
  type: PinType // Input, Output, Power, Ground, Bidirectional
  position: Point
  orientation: Orientation
  electricalType: ElectricalType
}

interface Symbol {
  graphics: GraphicElement[]
  boundingBox: Rectangle
  origin: Point
  alternates?: Symbol[] // 备选符号
}
```

#### 标准元件库内容

- **基础元件**（25个）
  - 电阻、电容、电感
  - 二极管、LED、齐纳二极管
  - NPN/PNP 晶体管、MOSFET
  - 电源、地、测试点
- **逻辑门**（15个）
  - AND、OR、NOT、NAND、NOR、XOR、XNOR
  - 缓冲器、施密特触发器
- **集成电路**（20个）
  - 运算放大器（LM358、TL084等）
  - 稳压器（7805、LM317等）
  - 微控制器（Arduino、STM32基础型号）
  - 存储器（EEPROM、Flash）
- **连接器**（15个）
  - 排针、排母
  - USB、RJ45
  - 端子、测试点

### 2. 原理图绘制功能

#### 绘制工具

```typescript
class SchematicEditor {
  // 工具状态机
  currentTool: Tool

  // 元件放置
  placeComponent(component: Component, position: Point): void {
    // 1. 创建元件实例
    // 2. 添加到画布
    // 3. 更新网络列表
  }

  // 连线绘制
  drawWire(start: Point, end: Point): Wire {
    // 1. 智能寻径
    // 2. 避障处理
    // 3. 正交约束
  }

  // 标注功能
  addLabel(wire: Wire, text: string): void {
    // 添加网络标签
  }
}
```

#### 智能连线算法

```rust
// src-tauri/src/core/routing/schematic_router.rs
pub struct SchematicRouter {
    grid: Grid,
    obstacles: Vec<BoundingBox>,
}

impl SchematicRouter {
    pub fn route(&self, start: Point, end: Point) -> Vec<Segment> {
        // 使用 A* 算法进行正交路径规划
        let mut path = Vec::new();

        // 1. 网格化起点和终点
        let start_grid = self.grid.snap_to_grid(start);
        let end_grid = self.grid.snap_to_grid(end);

        // 2. A* 寻路
        let grid_path = self.astar_search(start_grid, end_grid);

        // 3. 路径优化（减少拐点）
        let optimized = self.optimize_path(grid_path);

        // 4. 转换为线段
        for i in 0..optimized.len() - 1 {
            path.push(Segment {
                start: optimized[i],
                end: optimized[i + 1],
            });
        }

        path
    }

    fn astar_search(&self, start: GridPoint, end: GridPoint) -> Vec<GridPoint> {
        // A* 算法实现
        // 使用曼哈顿距离作为启发函数
    }

    fn optimize_path(&self, path: Vec<GridPoint>) -> Vec<Point> {
        // 道格拉斯-普克算法简化路径
    }
}
```

### 3. 电气规则检查（ERC）

#### ERC 规则集

```rust
pub enum ERCRule {
    // 连接性检查
    FloatingPin,           // 悬空引脚
    UnconnectedPowerPin,   // 未连接的电源引脚
    SinglePinNet,          // 只有一个引脚的网络

    // 冲突检查
    OutputConflict,        // 多个输出连接
    PowerConflict,         // 电源冲突

    // 标注检查
    DuplicateReference,    // 重复的元件标号
    MissingValue,          // 缺少数值

    // 电气特性检查
    VoltageMismatch,       // 电压不匹配
    CurrentOverload,       // 电流过载
}

pub struct ERCChecker {
    rules: Vec<ERCRule>,
    severity_map: HashMap<ERCRule, Severity>,
}

impl ERCChecker {
    pub fn check(&self, schematic: &Schematic) -> Vec<ERCViolation> {
        let mut violations = Vec::new();

        for rule in &self.rules {
            match rule {
                ERCRule::FloatingPin => {
                    violations.extend(self.check_floating_pins(schematic));
                }
                ERCRule::OutputConflict => {
                    violations.extend(self.check_output_conflicts(schematic));
                }
                // ... 其他规则
            }
        }

        violations
    }
}
```

### 4. 层次化设计

#### 层次结构

```typescript
interface HierarchicalDesign {
  rootSheet: Sheet
  subSheets: Map<string, Sheet>
  hierarchy: TreeNode<Sheet>
}

interface Sheet {
  id: string
  name: string
  components: Component[]
  wires: Wire[]
  hierarchicalBlocks: HierarchicalBlock[]
  ports: Port[] // 层次端口
}

interface HierarchicalBlock {
  id: string
  sheetReference: string
  instanceName: string
  position: Point
  portConnections: Map<string, string>
}
```

## 📊 Sprint 计划

### Sprint 7-8：元件库系统（第1-4周）

#### 任务列表

- [ ] 设计元件数据模型
- [ ] 实现元件库管理器
- [ ] 创建标准元件库（50+ 元件）
- [ ] 实现元件搜索和过滤
- [ ] 实现元件预览功能
- [ ] 支持自定义元件创建

### Sprint 9-10：原理图绘制（第5-8周）

#### 任务列表

- [ ] 实现元件放置工具
- [ ] 实现连线工具
- [ ] 实现智能连线算法
- [ ] 实现总线功能
- [ ] 实现文本标注
- [ ] 实现复制/粘贴功能

### Sprint 11-12：ERC 和导出（第9-12周）

#### 任务列表

- [ ] 实现 ERC 规则引擎
- [ ] 创建 ERC 规则集
- [ ] 实现 ERC 报告生成
- [ ] 实现 PDF 导出
- [ ] 实现网表生成
- [ ] 实现 BOM 导出

## 🎨 UI/UX 设计

### 元件库面板

```vue
<template>
  <div class="component-library-panel">
    <!-- 搜索栏 -->
    <div class="search-bar">
      <input v-model="searchQuery" placeholder="搜索元件..." />
      <select v-model="selectedCategory">
        <option value="all">所有类别</option>
        <option value="basic">基础元件</option>
        <option value="ic">集成电路</option>
        <option value="connector">连接器</option>
      </select>
    </div>

    <!-- 元件列表 -->
    <div class="component-list">
      <div
        v-for="component in filteredComponents"
        :key="component.id"
        class="component-item"
        draggable="true"
        @dragstart="onDragStart(component)"
      >
        <ComponentPreview :component="component" />
        <span>{{ component.name }}</span>
      </div>
    </div>

    <!-- 元件详情 -->
    <div class="component-details" v-if="selectedComponent">
      <h3>{{ selectedComponent.name }}</h3>
      <p>{{ selectedComponent.description }}</p>
      <div class="properties">
        <!-- 属性列表 -->
      </div>
    </div>
  </div>
</template>
```

### 属性编辑器

```vue
<template>
  <div class="property-editor">
    <h3>元件属性</h3>
    <div v-for="prop in properties" :key="prop.name" class="property-row">
      <label>{{ prop.label }}:</label>
      <input v-if="prop.type === 'text'" v-model="prop.value" @change="updateProperty(prop)" />
      <select
        v-else-if="prop.type === 'select'"
        v-model="prop.value"
        @change="updateProperty(prop)"
      >
        <option v-for="opt in prop.options" :value="opt.value">
          {{ opt.label }}
        </option>
      </select>
    </div>
  </div>
</template>
```

## 🧪 测试策略

### 功能测试

```typescript
describe('Schematic Editor', () => {
  describe('Component Placement', () => {
    it('should place component on canvas', () => {
      const editor = new SchematicEditor()
      const resistor = library.getComponent('R')
      editor.placeComponent(resistor, { x: 100, y: 100 })
      expect(editor.components.length).toBe(1)
    })
  })

  describe('Wire Routing', () => {
    it('should route wire with obstacles', () => {
      const router = new WireRouter()
      const path = router.route({ x: 0, y: 0 }, { x: 100, y: 100 }, obstacles)
      expect(path).not.toIntersectWith(obstacles)
    })
  })
})
```

### 性能测试

```typescript
describe('Performance', () => {
  it('should handle 1000+ components', () => {
    const startTime = performance.now()

    for (let i = 0; i < 1000; i++) {
      editor.placeComponent(component, randomPosition())
    }

    const renderTime = performance.now() - startTime
    expect(renderTime).toBeLessThan(1000) // < 1秒
  })
})
```

## 📈 性能优化

### 渲染优化

- **组件实例化**：使用对象池复用组件实例
- **批量渲染**：合并相同类型的绘制调用
- **视口剔除**：只渲染可见区域的对象
- **LOD 系统**：根据缩放级别调整渲染细节

### 数据结构优化

- **空间索引**：使用四叉树加速碰撞检测
- **网表缓存**：增量更新网表
- **撤销历史压缩**：合并连续的小操作

## 🎯 阶段二交付标准

### 功能要求

- [ ] 能够放置和编辑元件
- [ ] 能够绘制和编辑连线
- [ ] 支持层次化设计
- [ ] ERC 检查功能完整
- [ ] 能够导出 PDF 和网表

### 质量要求

- [ ] 测试覆盖率 > 85%
- [ ] 无阻塞性 Bug
- [ ] 文档完整
- [ ] 性能达标

### 用户体验

- [ ] 操作流畅自然
- [ ] 快捷键完整
- [ ] 错误提示清晰
- [ ] 学习曲线平缓

## 🚀 下一步计划

完成阶段二后，将进入阶段三（PCB 设计器）的开发，主要任务包括：

- PCB 布局工具
- 自动布线引擎
- 3D 可视化
- DRC 检查
- 制造文件生成
