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
* 相机的背景色
* 除了通过资源配置相机的默认背景色, 还可以在相机上单独配置
* 会影响已经创建的相机

### AudioBundle
* 播放声音的Bundle, 创建实体后自动添加`AudioSink`组件来进行控制
* 可以配置为 播放一次/循环播放/播放后删除实体/播放后删除组件(实体还在)
* 已经播放声音, 通过`GlobalVolume`资源调节全局音量不会生效
* 通过`AudioSink`组件, 可以设置音量, 播放, 暂停, 控制速度
    * 注意`stop`后无法再次`play`

### Event
* 结构体或者枚举派生`Event`这个trait, 然后通过`add_event`添加到`app`
* 通过`EventWriter<T>`组件发送事件(广播), send方法发送
* 通过`EventReader<T>`组件接收事件, read方法读
    * 多个系统可以处理同一个事件
    * 同一个系统不会对同一个事件重复响应(因为存储了`Local`变量)
* 事件会保留2个帧或者2个步长之后自动销毁, 如果选择手动销毁然后忘了就会浪费内存
    * 手动处理`Events`组件, 但是bevy内置的事件类型不行, 只能自己管理自定义事件
    * 如果接收事件的A系统由于条件并未执行, 过了2帧A系统再执行时, 事件已经被销毁, 无法接收到了
* 事件队列本质就是一个`vector`
* 优先使用事件, 性能高于变更检测
* 事件相关系统会受到执行顺序影响, 需要控制执行顺序
    * 如果A系统接收事件的情况下, A系统已经运行, B系统再发事件, A只能在下一帧才能接收到了

### Local
* 系统的本地变量, 作为系统的参数, 只有这个系统能够访问到, 独立于实体和组件
* 一个系统可以有多个相同类型的`Local`
* 类型需要实现`Default`或者`FromWorld`来自动初始化
* 如果需要自定义初始化, 还可以使用闭包, 在组册系统时候传入参数

### Schedule
* 负责调度系统的运行
* 注册系统时, 可以设置条件, 设置执行顺序, 通过系统集控制顺序
* 执行系统需要满足下列条件, 默认并行
    * 没有其他系统访问需要的可变引用
    * 依赖的前置位系统全都执行完成或者被跳过
    * 自身条件表达式为`true`
* 3个主要的`Schedule`
    * `Main`运行其他的一系列`Schedule`
        * 启动执行
            * PreStartup
            * Startup
            * PostStartup
        * 每帧执行 (由于和帧数相关, 每秒执行次数不固定)
            * First
            * PreUpdate
            * Update
            * StateTransition: 由于有状态的判断条件, 不一定会执行
            * RunFixedMainLoop: 按照`FixedTime`的固定次数执行 (不会每帧执行, 但总执行次数固定. 例如第一帧没执行, 那么下一帧会额外执行一次补回第一帧的次数)
                * FixedFirst
                * FixedPreUpdate
                * FixedUpdate
                * FixedPostUpdate
                * FixedLast
            * PostUpdate
            * Last
    * `ExtractSchedule`从`Main`提取数据给`Render`
    * `Render`渲染视图
    * 执行过程为: 上一帧的`Render`和当前帧的`Main`并行, `Main`结束后执行`ExtractSchedule`, 结束后再执行`Render`同时并行下一帧的`Main`
* 可以创建自定义的`Schedule`
    * 需要添加`ScheduleLable`组件作为标签
    * 通过`add_schedule`注册到`app`中
    * 通过`app.world_mut().resource_mut::<MainScheduleOrder>()`来控制执行顺序
        * 例如执行`insert_after`把自定义的`Schedule`放在某个`Schedule`之后执行

### State
* 状态, 枚举类型, 派生`States`等一系列`trait`
* 设置了`#[default]`可以做`init_state`, 不然只能`insert_state`
* 注册系统时, 可以通过`in_state`限定在某些状态执行
    * 还可以通过系统集的形式, 为系统集配置`run_if(in_state)`, 然后添加系统到系统集
* `OnEnter`进入状态, `OnExit`退出状态, `OnTransition`切换状态(可以指定切换前后的状态)
* 自定义插件时, 可以把状态作为泛型参数, 这样在插件内就能适配不同的状态切换了
* 通过`Res<State>`的`get`方法来获取状态, `ResMut<NextState>`的`set`方法更新状态
* 切换状态流程
    * 广播`StateTransitionEvent`事件, 适合不关系切换前后状态的情况
        * 发生在`PreUpdate`和`Update`/`FixedMain`中间
    * 执行`OnExit`, 退出旧状态
    * 执行`OnTrainsition`, 携带切换前后的状态
    * 执行`OnEnter`, 进入新状态
* 如果一帧切换一次状态还不够, 可以通过`apply_state_transition`额外切换

### Plugin
* 插件, 模块化资源
* `DefaultPlugins`包含了很多功能, 比如窗口
* `MinimalPlugins`只包含局部功能
* `WorldInspectorPlugin`能够获取实例的当前状态, 也能修改
* `BlendyCamerasPlugin `可以在运行时调整摄像机
* `BillboardPlugin`用于在3d中显示文字
* 可以实现`Plugin`的trait来创建自己的插件, 通过`build`方法完成对app的处理.  除了`build`, 还有其他方法可以实现
    * `is_unique`插件能否多次实例化
    * `name`定义插件名字
    * `ready`所有插件都`build`后, 再遍历所有插件, 顺序执行这个方法
        * 如果`build`包含异步操作, `ready`里需要判断异步操作是否结束, 结束再返回`true`
    * `finish`所有插件的`ready`都返回true后, 再遍历所有插件, 顺序执行这个方法
    * 也可以写一个方法接收`&mut App`参数作为插件, 但是不能像结构体一样进行配置
* 插件之前安装声明顺序安装, 需要先安装默认插件
* 自定义插件可以不必暴露出所以方法, 只需要暴露一个入口来接收`app`即可
    * 数据也可以进行私有化, 不去暴露组件的类型, 这样外面没办法直接查询
* 最佳实践
    * 为不同的状态创建插件
    * 为不同的子系统创建插件
* 通过插件组(实现`PluginGroup`的结构体)来插入多个插件, 同时提供`set`方法替换某些插件, 提供`disable`方法禁用某些插件
    * 禁用后仍然会编译到代码中, 想彻底删除需要调整`Cargo.toml`的`features`属性
* 运行时不可变的属性, 放到插件里. 运行时可变的, 放到资源里

### Changed
* 变化检测, 作为Query的条件
    * Added会在组件添加到实体, 或者创建带有组件的实体时触发
    * Changed会在组件被修改, 或者添加到实体时触发
* 通过Ref包裹作为条件时, 可以通过不可变引用获取是否添加(`is_added`)和是否修改(`is_changeed`)
* 资源也提供了`is_added`和`is_changeed`这2个方法
* 修改组件时, 新旧值即使相同, 也会触发变化检测

### Parent
* `ecs`是扁平架构, 带层次的结构的情况使用父子关系来实现
* 实体可以创建时就带着父子结构, 也可以后期修改
* 实体可以通过`parent`和`children`组件访问父子组件
* 子实体可以从父实体继承位置和可见性
* 父子关系和实体不是强关联的, 单独删除实体不会解除父子关系, 需要单独维护

### Visibility
* 可见性需要3个组件
    * `Visibility`可写, 控制是否渲染的组件, 可以从父实体继承或者手动控制隐藏和显示
    * `InheritedVisibility`只读, 从父实体继承得到的可见性, 在`PostUpdate`的`VisibilitySystem::VisibilityPropagate`之后才能拿到最新值
    * `ViewVisibility`只读, 上面2个组件叠加后, 是否在相机和光源之内的真实可见性, 在`PostUpdate`的`VisibilitySystem::CheckVsibility`之后才能拿到最新值
        * 用于性能优化, 不可见就不渲染了
* 通过`VisibilityBundle`可以快速创建上面3个组件
    * 通过`SpatialBundel`可以快速创建可见和变换组件

### AssetEvent


## 物理引擎
* Avian, 专为bevy开发的物理引擎, 稳定性不太好
* Rapier3d, 通用物理引擎套了个bevy的桥