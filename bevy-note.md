# Bevy Note
* 开源, 免费的rust游戏引擎

## 核心概念

### ECS
* E: entity, 实体
    * 可以理解为表结构的id, 唯一标识一组数据
    * 实体被删除后, id可以重复利用
    * 自动创建
* C: Component, 组件
    * 可以理解为字段, 表结构中的列
    * 本身是结构体, 派生`Component`的trait, 但是ecs的设计模式要求组件最小化, 也就是变成了一个字段
    * 由于孤儿原则, 不能直接对外部类派生外部的`Component`的trait, 需要创建一个结构体作为包装来派生
    * 组件可以为空(结构体), 作为一个标识符(像一个tag), 用于query的条件
* S: System, 系统
    * 也就是方法(也可以是获得所有权的闭包), 注册进入app, 在特定时机调用
    * 可以访问资源, 实体, 组件, 世界, 发送接收事件, 还可以做创建和销毁操作
    * 可以控制执行顺序(串行, 并行)
        * 获取world(整个游戏世界)时, 由于访问权限太大, 无法和其他系统并行
        * 先注册到系统, 才能通过before/after指定这个系统. 而chain(串行)不需要先注册
        * 如果是系统集, 则可以直接控制
    * 可以控制触发时机(`Startup`, `Update`等)
    * 可以控制条件之下(`run_if`)
    * 系统集(枚举, 派生`SystemSet`等trait), 在add_system时, 系统额外执行`in_set`来指定放入系统集中
        * 通过`configure_set`来配置系统集什么执行执行, 执行顺序
        * 系统集中注册的系统和系统集的执行时机需要相同
    * 系统的参数数量存在限制, 超过限制可以用元组嵌套
    * 通过`register_one_shot_system`可以注册一次性系统拿到id, 通过`commands.run_system`来单独完成一次操作

### App
* 所有功能的入口, 通过`new`创建实例
* 负责集成插件, 管理反射, 管理系统/系统集, 管理资源, 管理状态, 管理事件
* 最后通过`run`来启动应用

### Plugin
* 插件, 模块化资源
* `DefaultPlugins`包含了很多功能, 比如窗口
* `MinimalPlugins`只包含局部功能
* `WorldInspectorPlugin`能够获取实例的当前状态, 也能修改
* `BlendyCamerasPlugin `可以在运行时调整摄像机
* 可以实现`Plugin`的trait来创建自己的插件, 通过`build`方法完成对app的处理
* 插件之前安装声明顺序安装, 需要先安装默认插件

### Resource
* 资源, 全局单例
* 本质是派生了`Resource`的结构体
    * 实现`from_world`方法可以被初始化
    * 还可以派生`Default`, 也可以被初始化
* 包含一下内置资源, 比如计时器
* 作为系统的参数, 通过`Res/ResMut`访问资源

### Bundle
* 结构体, 派生`Bundle`的trait, 每个属性都是一个组件
* bundle层面可以做嵌套, 但是最后生成到实体的时候, 会是扁平结构体
* 聚合组件, 组成一个完整的结构, 可以理解为表结构中的一整行数据

### Query
* 查询, 作为系统的参数, 通过指定类型为`Query<Component>`, 获得包含该元组的所有实体
* 范型可以指定为元组进行嵌套, 获取多个实体
* 还可以用Option包裹做空处理
* 除了获取组件, 还可以查询出`Entity`, 也就是获取包含这个组件的实体(id)
* 查询的结果还可以通过`iter/iter_mut`进行迭代
* 查询结果还可以通过`get/get_mut`方法传入实体id来获取特定实体
    * 还有`get_many`/`iter_many`根据id数组来拿到多个实体
    * 还有`single/get_single`获取唯一的实体
* 用iter_combinations获取组件的组合(比如判断2个实体是否碰撞, 需要每2个实体进行组合), 会很慢
* 通过`With/Without`附加额外条件, 而不会获取该组件
    * 多个条件放入元组是and条件
    * 使用`Or<>`包裹变成or条件

### Commands
* 命令, 作为系统的参数来获取
* `spawn`创建实体
* `entity`根据id获取实体
    * `insert`添加组件
    * `remove`删除组件
    * `retain`保留特定组件, 删除其他组件
    * `despawn`删除实体
* 不会立即生效, 需要等其他并行系统结束才能安全的更新
    * 会确保串行的下一个系统开始前生效
    * 会确保在下一个schedule开始前生效
* 可以执行`commands.add`中添加并执行自定义命令, 来操作`world`
    * 自定义命令实现`Command`这个`trait`, 在`apply`方法操作world
* 还可以对`Commands`进行扩展, 让`Commands`实现一个自己的`trait`, 后续执行自定义的方法

### 坐标系

#### 2d/3d
* x向右, y向上, z向前

#### ui/光标 
* 原点在左上
* x向右, y向下

### Transform
* 控制移动, 旋转, 缩放的组件, 作为系统的参数来获取
* 分为local和global这2种, 也可以都添加, 还可以通过`TransformBundle`一起添加
* 父级做变化时, 子实例会一起发生变化
* 全局的globalTransform不能直接访问, 需要计算后再取值
    * 全局的不能立刻拿到值, 需要在`PostUpdate`传播更新后, 并且在`TransformSystem::TransformPropagate`这个系统集后才能拿到最新的值
* 平移和缩放可以直接改, 旋转难以计算所以提供了方法

### Local
* 系统的本地变量
* 只有这个系统能够访问到

### Time
* 全局资源, 每帧更新

#### 增量时间(delta_time)
* 是2帧的间隔时间, 通过`time.delta_seconds`方法获取

#### 计时器(Timer)/秒表(Stopwatch)
* 注意`Timer`和`Stopwatch`不是 组件/资源, 需要创建一个 组件/资源 来包裹
* 不会自动更新自身时间, 需要在系统执行`tick(time.delta())`方法来更新时间
* 计时器可以设置最大值, 从0开始累加到最大值, 可以配置的2种模式
    * 重复运行, 到时间后会清0重新开始
    * 运行一次, 到时间后不动
* 判断是否到达最大时间
    * finished 已经到达, 如果是Once模式, 到达最大后一直是true
    * just_finished 正好到达
* 秒表没有最大值, 只有`elapsed/elapsed_secs`来统计时间, 可以`reset`

### Camera
* 负责驱动渲染. 要显示画面, 必须有相机
* 默认提供`Camera2dBundle`和`Camera2dBundle`, 来绘制所有可见的可渲染实体
* 通过`Transform`来移动相机
* 通过`Projection`来缩放相机
    * 正交通过scale调整, scale越小, 物体越大
        * 正交: 无论物体多远, 看起来始终一样大
        * 2d只能是正交
    * 透视通过fov调整, 一般是π除一个数
        * 透视: 物体越远, 看起来越小
        * 3d默认是透视
* 渲染层(RenderLayers)组件, 将物体放置在层上
    * 默认在0层
    * 可以切换所在层
    * 实体和相机都能添加该组件
    * 相机和实例在同一层才会显示
* 相机可以禁用和重启, 修改`is_active`
* `DirectionLightBundle`用于创建定向光, 模拟太阳

### AssetServer
* 加载资产的资源
* `load`方法进行加载, 默认从`asset`路径开始加载, 返回`handle`句柄
* 加载是异步的, 直到加载完成是无法立即获取实际数据的, 在加载好之后才显示. 但是句柄可以放入实体中
    * 多次调用不会额外浪费资源, 会检查加载状态再决定是否去加载
* 加载图片可以用`SpritBundle`, 句柄放入`texture`组件中
* `gltf`资源中包含多种资产的情况下, 可以通过`#tag`决定加载哪些内容
    * 或者通过`GltfAssetLable`的枚举来加载
* 通过Scence
* 自定义的材质需要注册到`MaterialPlugin`中

### Keyboard
* `ButtonInput<KeyCode>`, 作为系统的参数来获取键盘按下的键位
* `pressed/released` 获取是否已经按下和释放
* `just_` 开头用于获取正在按下和释放
* 可以用`input_just_pressed`方法作为`run_if`的条件
* `EventReader<KeyboardInput>`可以获取键盘完整输入, 包括逻辑键位, 通过`read`方法获取`KeyboardInput`, 再去获取详细信息
    * 逻辑键通过`Charcter`枚举获取输入内容
* `NodeBundle`用于创建UI

### Mouse
* `ButtonInput<MouseButton>`, 作为系统的参数来获取鼠标的操作
* 同样能够获取已经按下释放和正在按下释放, 也能作为运行条件
* `EventReader<MouseWheel>`可以获取鼠标滚动输入, 通过`read`方法获取`MouseWheel`, 再去获取详细信息
    * 先判断移动的单位, 可能是行(Line)或者像素(Pixel), 受到硬件和操作系统影响
    * 行适用于固定步长的硬件, 桌面鼠标滚轮. 可能不是整数.
    * 像素适用于平滑滚动的硬件, 笔记本触摸板
* `EventReader<MouseMotion>`用于获取2帧之间鼠标在窗口中移动的增量, 通过`read`方法获取`MouseMotion`后再取得`delta`
* `Window`能够获取窗口内鼠标的实际位置`cursor_position`, 在通过相机把窗口中的位置转换为世界中的位置, 例如`camera.viewport_to_world_2d`
* `EventReader<CursorMoved>`可以获取鼠标移动事件, 通过`read`方法获取`CursorMoved`, 再去获取详细信息
    * 获取到的`position`也需要通过相机进行转换
* `Gizmos`用于绘制图形

### Window
* 配置标题, id(name), 模式(窗口, 全屏, 无边框等), 位置, 透明(背景色也要透明才行), 分辨率, 最大最小尺寸, 光标(图标, 位置等)等
* 在`DefaultPlugins`中, 可以调用`set`方法进行配置, 最后通过`..default()`填充剩余默认配置

### ClearColor
* 背景色