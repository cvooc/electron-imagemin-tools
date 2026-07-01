//! Stack widget — 将多个子元素叠放渲染（类似 CSS `position: absolute` 容器）。
//! 第一个子元素是底色层，后面的子元素绘制在上层。

use iced::advanced::layout;
use iced::advanced::renderer;
use iced::advanced::widget::{self, Operation, Tree};
use iced::advanced::{Clipboard, Layout, Shell};
use iced::event;
use iced::mouse;
use iced::{Element, Event, Length, Rectangle, Size};

/// 叠放容器：所有子元素占据相同空间，按顺序从底到顶绘制。
#[allow(missing_debug_implementations)]
pub struct Stack<'a, Message, Theme, Renderer> {
    width: Length,
    height: Length,
    children: Vec<Element<'a, Message, Theme, Renderer>>,
}

impl<'a, Message, Theme, Renderer> Stack<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    pub fn new() -> Self {
        Self {
            width: Length::Fill,
            height: Length::Fill,
            children: Vec::new(),
        }
    }

    /// 设置宽度
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// 设置高度
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// 添加一个子元素
    pub fn push(
        mut self,
        child: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        self.children.push(child.into());
        self
    }
}

impl<'a, Message, Theme, Renderer> widget::Widget<Message, Theme, Renderer>
    for Stack<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Renderer: iced::advanced::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn children(&self) -> Vec<Tree> {
        self.children.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.children);
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.width(self.width).height(self.height);
        let max_size = limits.max();

        let child_nodes: Vec<layout::Node> = self
            .children
            .iter()
            .zip(&mut tree.children)
            .map(|(child, state)| {
                // 每个子元素都铺满整个可用空间
                let child_limits =
                    layout::Limits::new(Size::ZERO, max_size);
                let node =
                    child.as_widget().layout(state, renderer, &child_limits);
                // 对齐到 (0, 0)
                node.move_to(iced::Point::ORIGIN)
            })
            .collect();

        layout::Node::with_children(max_size, child_nodes)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        // 从底到顶依次绘制
        for ((child, state), child_layout) in self
            .children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
        {
            child.as_widget().draw(
                state,
                renderer,
                theme,
                style,
                child_layout,
                cursor,
                viewport,
            );
        }
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        // 从顶层（最后一个子元素）向底层路由事件
        // 收集子元素和 layout，然后反向迭代
        let mut children_and_layouts: Vec<(
            &mut Element<'a, Message, Theme, Renderer>,
            &mut Tree,
            Layout<'_>,
        )> = self
            .children
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
            .map(|((c, t), l)| (c, t, l))
            .collect();

        for (child, state, child_layout) in children_and_layouts.iter_mut().rev() {
            let status = child.as_widget_mut().on_event(
                state,
                event.clone(),
                *child_layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );
            if status == event::Status::Captured {
                return event::Status::Captured;
            }
        }
        event::Status::Ignored
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
            .map(|((child, state), child_layout)| {
                child.as_widget().mouse_interaction(
                    state,
                    child_layout,
                    cursor,
                    viewport,
                    renderer,
                )
            })
            .max()
            .unwrap_or_default()
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            self.children
                .iter()
                .zip(&mut tree.children)
                .zip(layout.children())
                .for_each(|((child, state), child_layout)| {
                    child.as_widget().operate(
                        state,
                        child_layout,
                        renderer,
                        operation,
                    );
                });
        });
    }
}

impl<'a, Message, Theme, Renderer> From<Stack<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: 'a,
    Renderer: iced::advanced::Renderer + 'a,
{
    fn from(stack: Stack<'a, Message, Theme, Renderer>) -> Self {
        Self::new(stack)
    }
}
