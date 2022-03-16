use crate::context::HasContext;
use crate::core::*;

/// The basic data type used for each index in an element buffer.
pub trait ElementBufferDataType: internal::DataType {
    ///
    /// Converts the index to `u32`.
    ///
    fn as_u32(&self) -> u32;
}
impl ElementBufferDataType for u8 {
    fn as_u32(&self) -> u32 {
        *self as u32
    }
}
impl ElementBufferDataType for u16 {
    fn as_u32(&self) -> u32 {
        *self as u32
    }
}
impl ElementBufferDataType for u32 {
    fn as_u32(&self) -> u32 {
        *self
    }
}

///
/// A buffer containing 3 indices for each triangle to be rendered, which is why it is also known as an index buffer.
/// The three indices refer to three places in a set of [VertexBuffer] where the data (position, normal etc.) is found for the three vertices of the triangle.
/// See for example [Program::draw_elements] to use this for drawing.
///
pub struct ElementBuffer<T: ElementBufferDataType> {
    context: Context,
    id: glow::Buffer,
    count: usize,
    _dummy: T,
}

impl<T: ElementBufferDataType> ElementBuffer<T> {
    ///
    /// Creates a new empty element buffer.
    ///
    pub fn new(context: &Context) -> ThreeDResult<Self> {
        let id = unsafe {
            context
                .create_buffer()
                .map_err(|e| CoreError::BufferCreation(e))?
        };
        Ok(Self {
            context: context.clone(),
            id,
            count: 0,
            _dummy: T::default(),
        })
    }

    ///
    /// Creates a new element buffer and fills it with the given indices which must be divisable by 3.
    ///
    pub fn new_with_data(context: &Context, data: &[T]) -> ThreeDResult<Self> {
        let mut buffer = Self::new(context)?;
        if data.len() > 0 {
            buffer.fill(data)?;
        }
        Ok(buffer)
    }

    ///
    /// Creates a new element buffer and fills it with the given indices which must be divisable by 3.
    ///
    #[deprecated = "use new_with_data()"]
    pub fn new_with(context: &Context, data: &[T]) -> ThreeDResult<Self> {
        Self::new_with_data(context, data)
    }

    ///
    /// Fills the buffer with the given indices which must be divisable by 3.
    ///
    pub fn fill(&mut self, data: &[T]) -> ThreeDResult<()> {
        self.bind();
        unsafe {
            self.context.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                super::internal::to_byte_slice(data),
                glow::STATIC_DRAW,
            );
            self.context.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
        }
        self.count = data.len();
        self.context.error_check()
    }

    ///
    /// Fills the buffer with the given indices which must be divisable by 3.
    ///
    #[deprecated = "use fill()"]
    pub fn fill_with(&mut self, data: &[T]) -> ThreeDResult<()> {
        self.fill(data)
    }

    ///
    /// The number of values in the buffer.
    ///
    pub fn count(&self) -> usize {
        self.count
    }

    ///
    /// The number of triangles in the buffer.
    ///
    pub fn triangle_count(&self) -> usize {
        self.count / 3
    }

    pub(crate) fn bind(&self) {
        unsafe {
            self.context
                .bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.id));
        }
    }
}

impl<T: ElementBufferDataType> Drop for ElementBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            self.context.delete_buffer(self.id);
        }
    }
}
