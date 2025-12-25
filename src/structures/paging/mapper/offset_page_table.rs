#![cfg(target_pointer_width = "64")]

use crate::structures::paging::{mapper::*, page_table::{PageTable, PageTableIndex}};

/// A Mapper implementation that requires that the complete physical memory is mapped at some
/// offset in the virtual address space.
#[derive(Debug)]
pub struct OffsetPageTable<'a> {
    inner: MappedPageTable<'a, PhysOffset>,
}

impl<'a> OffsetPageTable<'a> {
    /// Creates a new `OffsetPageTable` that uses the given offset for converting virtual
    /// to physical addresses.
    ///
    /// The complete physical memory must be mapped in the virtual address space starting at
    /// address `phys_offset`. This means that for example physical address `0x5000` can be
    /// accessed through virtual address `phys_offset + 0x5000`. This mapping is required because
    /// the mapper needs to access page tables, which are not mapped into the virtual address
    /// space by default.
    ///
    /// ## Safety
    ///
    /// This function is unsafe because the caller must guarantee that the passed `phys_offset`
    /// is correct. Also, the passed `level_4_table` must point to the level 4 page table
    /// of a valid page table hierarchy. Otherwise this function might break memory safety, e.g.
    /// by writing to an illegal memory location.
    #[inline]
    pub unsafe fn new(level_4_table: &'a mut PageTable, phys_offset: VirtAddr) -> Self {
        let phys_offset = PhysOffset {
            offset: phys_offset,
        };
        Self {
            inner: unsafe { MappedPageTable::new(level_4_table, phys_offset) },
        }
    }

    /// Returns an immutable reference to the wrapped level 4 `PageTable` instance.
    pub fn level_4_table(&self) -> &PageTable {
        self.inner.level_4_table()
    }

    /// Returns a mutable reference to the wrapped level 4 `PageTable` instance.
    pub fn level_4_table_mut(&mut self) -> &mut PageTable {
        self.inner.level_4_table_mut()
    }

    /// Returns the offset used for converting virtual to physical addresses.
    pub fn phys_offset(&self) -> VirtAddr {
        self.inner.page_table_frame_mapping().offset
    }
}

#[derive(Debug)]
struct PhysOffset {
    offset: VirtAddr,
}

unsafe impl PageTableFrameMapping for PhysOffset {
    fn frame_to_pointer(&self, frame: PhysFrame) -> *mut PageTable {
        let virt = self.offset + frame.start_address().as_u64();
        virt.as_mut_ptr()
    }
}

// delegate all trait implementations to inner

impl Mapper<Size1GiB> for OffsetPageTable<'_> {
    #[inline]
    unsafe fn map_to_with_table_flags<A>(
        &mut self,
        page: Page<Size1GiB>,
        frame: PhysFrame<Size1GiB>,
        flags: PageTableFlags,
        parent_table_flags: PageTableFlags,
        allocator: &mut A,
    ) -> Result<MapperFlush<Size1GiB>, MapToError<Size1GiB>>
    where
        A: FrameAllocator<Size4KiB> + ?Sized,
    {
        unsafe {
            self.inner
                .map_to_with_table_flags(page, frame, flags, parent_table_flags, allocator)
        }
    }

    #[inline]
    fn unmap(
        &mut self,
        page: Page<Size1GiB>,
    ) -> Result<(PhysFrame<Size1GiB>, MapperFlush<Size1GiB>), UnmapError> {
        self.inner.unmap(page)
    }

    #[inline]
    unsafe fn update_flags(
        &mut self,
        page: Page<Size1GiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlush<Size1GiB>, FlagUpdateError> {
        unsafe { self.inner.update_flags(page, flags) }
    }

    #[inline]
    unsafe fn set_flags_p4_entry(
        &mut self,
        page: Page<Size1GiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlushAll, FlagUpdateError> {
        unsafe { self.inner.set_flags_p4_entry(page, flags) }
    }

    #[inline]
    unsafe fn set_flags_p3_entry(
        &mut self,
        page: Page<Size1GiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlushAll, FlagUpdateError> {
        unsafe { self.inner.set_flags_p3_entry(page, flags) }
    }

    #[inline]
    unsafe fn set_flags_p2_entry(
        &mut self,
        page: Page<Size1GiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlushAll, FlagUpdateError> {
        unsafe { self.inner.set_flags_p2_entry(page, flags) }
    }

    #[inline]
    fn translate_page(&self, page: Page<Size1GiB>) -> Result<PhysFrame<Size1GiB>, TranslateError> {
        self.inner.translate_page(page)
    }
}

impl Mapper<Size2MiB> for OffsetPageTable<'_> {
    #[inline]
    unsafe fn map_to_with_table_flags<A>(
        &mut self,
        page: Page<Size2MiB>,
        frame: PhysFrame<Size2MiB>,
        flags: PageTableFlags,
        parent_table_flags: PageTableFlags,
        allocator: &mut A,
    ) -> Result<MapperFlush<Size2MiB>, MapToError<Size2MiB>>
    where
        A: FrameAllocator<Size4KiB> + ?Sized,
    {
        unsafe {
            self.inner
                .map_to_with_table_flags(page, frame, flags, parent_table_flags, allocator)
        }
    }

    #[inline]
    fn unmap(
        &mut self,
        page: Page<Size2MiB>,
    ) -> Result<(PhysFrame<Size2MiB>, MapperFlush<Size2MiB>), UnmapError> {
        self.inner.unmap(page)
    }

    #[inline]
    unsafe fn update_flags(
        &mut self,
        page: Page<Size2MiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlush<Size2MiB>, FlagUpdateError> {
        unsafe { self.inner.update_flags(page, flags) }
    }

    #[inline]
    unsafe fn set_flags_p4_entry(
        &mut self,
        page: Page<Size2MiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlushAll, FlagUpdateError> {
        unsafe { self.inner.set_flags_p4_entry(page, flags) }
    }

    #[inline]
    unsafe fn set_flags_p3_entry(
        &mut self,
        page: Page<Size2MiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlushAll, FlagUpdateError> {
        unsafe { self.inner.set_flags_p3_entry(page, flags) }
    }

    #[inline]
    unsafe fn set_flags_p2_entry(
        &mut self,
        page: Page<Size2MiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlushAll, FlagUpdateError> {
        unsafe { self.inner.set_flags_p2_entry(page, flags) }
    }

    #[inline]
    fn translate_page(&self, page: Page<Size2MiB>) -> Result<PhysFrame<Size2MiB>, TranslateError> {
        self.inner.translate_page(page)
    }
}

impl Mapper<Size4KiB> for OffsetPageTable<'_> {
    #[inline]
    unsafe fn map_to_with_table_flags<A>(
        &mut self,
        page: Page<Size4KiB>,
        frame: PhysFrame<Size4KiB>,
        flags: PageTableFlags,
        parent_table_flags: PageTableFlags,
        allocator: &mut A,
    ) -> Result<MapperFlush<Size4KiB>, MapToError<Size4KiB>>
    where
        A: FrameAllocator<Size4KiB> + ?Sized,
    {
        unsafe {
            self.inner
                .map_to_with_table_flags(page, frame, flags, parent_table_flags, allocator)
        }
    }

    #[inline]
    fn unmap(
        &mut self,
        page: Page<Size4KiB>,
    ) -> Result<(PhysFrame<Size4KiB>, MapperFlush<Size4KiB>), UnmapError> {
        self.inner.unmap(page)
    }

    #[inline]
    unsafe fn update_flags(
        &mut self,
        page: Page<Size4KiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlush<Size4KiB>, FlagUpdateError> {
        unsafe { self.inner.update_flags(page, flags) }
    }

    #[inline]
    unsafe fn set_flags_p4_entry(
        &mut self,
        page: Page<Size4KiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlushAll, FlagUpdateError> {
        unsafe { self.inner.set_flags_p4_entry(page, flags) }
    }

    #[inline]
    unsafe fn set_flags_p3_entry(
        &mut self,
        page: Page<Size4KiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlushAll, FlagUpdateError> {
        unsafe { self.inner.set_flags_p3_entry(page, flags) }
    }

    #[inline]
    unsafe fn set_flags_p2_entry(
        &mut self,
        page: Page<Size4KiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlushAll, FlagUpdateError> {
        unsafe { self.inner.set_flags_p2_entry(page, flags) }
    }

    #[inline]
    fn translate_page(&self, page: Page<Size4KiB>) -> Result<PhysFrame<Size4KiB>, TranslateError> {
        self.inner.translate_page(page)
    }
}

impl Translate for OffsetPageTable<'_> {
    #[inline]
    fn translate(&self, addr: VirtAddr) -> TranslateResult {
        self.inner.translate(addr)
    }
}

impl CleanUp for OffsetPageTable<'_> {
    #[inline]
    unsafe fn clean_up<D>(&mut self, frame_deallocator: &mut D)
    where
        D: FrameDeallocator<Size4KiB>,
    {
        unsafe { self.inner.clean_up(frame_deallocator) }
    }

    #[inline]
    unsafe fn clean_up_addr_range<D>(
        &mut self,
        range: PageRangeInclusive,
        frame_deallocator: &mut D,
    ) where
        D: FrameDeallocator<Size4KiB>,
    {
        unsafe { self.inner.clean_up_addr_range(range, frame_deallocator) }
    }
}

// =============================================================================
// 5-Level Paging (LA57) Support
// =============================================================================

/// A Mapper implementation for 5-level paging (LA57) that requires the complete
/// physical memory to be mapped at some offset in the virtual address space.
///
/// This is the 5-level paging equivalent of `OffsetPageTable`. Use this when
/// CR4.LA57 is set and CR3 points to a PML5 (level 5) table instead of PML4.
#[derive(Debug)]
pub struct OffsetPageTable5<'a> {
    level_5_table: &'a mut PageTable,
    phys_offset: PhysOffset5,
}

#[derive(Debug, Clone, Copy)]
struct PhysOffset5 {
    offset: VirtAddr,
}

unsafe impl PageTableFrameMapping for PhysOffset5 {
    fn frame_to_pointer(&self, frame: PhysFrame) -> *mut PageTable {
        let virt = self.offset + frame.start_address().as_u64();
        virt.as_mut_ptr()
    }
}

impl<'a> OffsetPageTable5<'a> {
    /// Creates a new `OffsetPageTable5` for 5-level paging (LA57).
    ///
    /// ## Safety
    ///
    /// This function is unsafe because the caller must guarantee that:
    /// - LA57 (5-level paging) is enabled in CR4
    /// - The passed `level_5_table` points to the active PML5 table
    /// - The complete physical memory is mapped at `phys_offset`
    #[inline]
    pub unsafe fn new(level_5_table: &'a mut PageTable, phys_offset: VirtAddr) -> Self {
        Self {
            level_5_table,
            phys_offset: PhysOffset5 { offset: phys_offset },
        }
    }

    /// Returns an immutable reference to the wrapped level 5 `PageTable` instance.
    pub fn level_5_table(&self) -> &PageTable {
        self.level_5_table
    }

    /// Returns a mutable reference to the wrapped level 5 `PageTable` instance.
    pub fn level_5_table_mut(&mut self) -> &mut PageTable {
        self.level_5_table
    }

    /// Returns the offset used for converting virtual to physical addresses.
    pub fn phys_offset(&self) -> VirtAddr {
        self.phys_offset.offset
    }

    /// Gets the P4 table for a given virtual address, walking from P5.
    fn get_p4(&self, addr: &VirtAddr) -> Result<&PageTable, TranslateError> {
        let p5_entry = &self.level_5_table[addr.p5_index()];
        if p5_entry.is_unused() {
            return Err(TranslateError::PageNotMapped);
        }
        let p4_frame = p5_entry.frame().map_err(|_| TranslateError::PageNotMapped)?;
        let p4_ptr = self.phys_offset.frame_to_pointer(p4_frame);
        Ok(unsafe { &*p4_ptr })
    }

    /// Gets or creates the P4 table for a page, walking from P5.
    fn get_or_create_p4<A>(
        &mut self,
        page: &Page<impl PageSize>,
        parent_table_flags: PageTableFlags,
        allocator: &mut A,
    ) -> Result<&mut PageTable, MapToError<Size4KiB>>
    where
        A: FrameAllocator<Size4KiB> + ?Sized,
    {
        let p5_index = page.start_address().p5_index();
        let p5_entry = &mut self.level_5_table[p5_index];

        if p5_entry.is_unused() {
            // Allocate a new P4 table
            let frame = allocator.allocate_frame().ok_or(MapToError::FrameAllocationFailed)?;
            p5_entry.set_frame(frame, parent_table_flags);
            // Zero out the new table
            let p4_ptr = self.phys_offset.frame_to_pointer(frame);
            unsafe { (*p4_ptr).zero() };
        }

        let p4_frame = p5_entry.frame().map_err(|_| MapToError::ParentEntryHugePage)?;
        let p4_ptr = self.phys_offset.frame_to_pointer(p4_frame);
        Ok(unsafe { &mut *p4_ptr })
    }
}

// Mapper implementations for OffsetPageTable5

impl Mapper<Size4KiB> for OffsetPageTable5<'_> {
    #[inline]
    unsafe fn map_to_with_table_flags<A>(
        &mut self,
        page: Page<Size4KiB>,
        frame: PhysFrame<Size4KiB>,
        flags: PageTableFlags,
        parent_table_flags: PageTableFlags,
        allocator: &mut A,
    ) -> Result<MapperFlush<Size4KiB>, MapToError<Size4KiB>>
    where
        A: FrameAllocator<Size4KiB> + ?Sized,
    {
        let phys_offset = self.phys_offset;
        let p4 = self.get_or_create_p4(&page, parent_table_flags, allocator)?;
        let mut mapper = unsafe { MappedPageTable::new(p4, phys_offset) };
        unsafe { mapper.map_to_with_table_flags(page, frame, flags, parent_table_flags, allocator) }
    }

    fn unmap(&mut self, page: Page<Size4KiB>) -> Result<(PhysFrame<Size4KiB>, MapperFlush<Size4KiB>), UnmapError> {
        let phys_offset = self.phys_offset;
        let p4 = self.get_or_create_p4(&page, PageTableFlags::PRESENT | PageTableFlags::WRITABLE, &mut NoopAllocator)
            .map_err(|_| UnmapError::PageNotMapped)?;
        let mut mapper = unsafe { MappedPageTable::new(p4, phys_offset) };
        mapper.unmap(page)
    }

    #[inline]
    unsafe fn update_flags(
        &mut self,
        page: Page<Size4KiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlush<Size4KiB>, FlagUpdateError> {
        let phys_offset = self.phys_offset;
        let p4 = self.get_or_create_p4(&page, PageTableFlags::PRESENT | PageTableFlags::WRITABLE, &mut NoopAllocator)
            .map_err(|_| FlagUpdateError::PageNotMapped)?;
        let mut mapper = unsafe { MappedPageTable::new(p4, phys_offset) };
        unsafe { mapper.update_flags(page, flags) }
    }

    #[inline]
    unsafe fn set_flags_p4_entry(
        &mut self,
        page: Page<Size4KiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlushAll, FlagUpdateError> {
        let phys_offset = self.phys_offset;
        let p4 = self.get_or_create_p4(&page, PageTableFlags::PRESENT | PageTableFlags::WRITABLE, &mut NoopAllocator)
            .map_err(|_| FlagUpdateError::PageNotMapped)?;
        let mut mapper = unsafe { MappedPageTable::new(p4, phys_offset) };
        unsafe { mapper.set_flags_p4_entry(page, flags) }
    }

    #[inline]
    unsafe fn set_flags_p3_entry(
        &mut self,
        page: Page<Size4KiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlushAll, FlagUpdateError> {
        let phys_offset = self.phys_offset;
        let p4 = self.get_or_create_p4(&page, PageTableFlags::PRESENT | PageTableFlags::WRITABLE, &mut NoopAllocator)
            .map_err(|_| FlagUpdateError::PageNotMapped)?;
        let mut mapper = unsafe { MappedPageTable::new(p4, phys_offset) };
        unsafe { mapper.set_flags_p3_entry(page, flags) }
    }

    #[inline]
    unsafe fn set_flags_p2_entry(
        &mut self,
        page: Page<Size4KiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlushAll, FlagUpdateError> {
        let phys_offset = self.phys_offset;
        let p4 = self.get_or_create_p4(&page, PageTableFlags::PRESENT | PageTableFlags::WRITABLE, &mut NoopAllocator)
            .map_err(|_| FlagUpdateError::PageNotMapped)?;
        let mut mapper = unsafe { MappedPageTable::new(p4, phys_offset) };
        unsafe { mapper.set_flags_p2_entry(page, flags) }
    }

    fn translate_page(&self, page: Page<Size4KiB>) -> Result<PhysFrame<Size4KiB>, TranslateError> {
        let p4 = self.get_p4(&page.start_address())?;

        // Walk P4 -> P3 -> P2 -> P1 manually for 4KiB pages
        let p4_entry = &p4[page.start_address().p4_index()];
        if p4_entry.is_unused() {
            return Err(TranslateError::PageNotMapped);
        }
        let p3_frame = p4_entry.frame().map_err(|_| TranslateError::ParentEntryHugePage)?;
        let p3 = unsafe { &*self.phys_offset.frame_to_pointer(p3_frame) };

        let p3_entry = &p3[page.start_address().p3_index()];
        if p3_entry.is_unused() {
            return Err(TranslateError::PageNotMapped);
        }
        if p3_entry.flags().contains(PageTableFlags::HUGE_PAGE) {
            return Err(TranslateError::ParentEntryHugePage);
        }
        let p2_frame = p3_entry.frame().map_err(|_| TranslateError::ParentEntryHugePage)?;
        let p2 = unsafe { &*self.phys_offset.frame_to_pointer(p2_frame) };

        let p2_entry = &p2[page.start_address().p2_index()];
        if p2_entry.is_unused() {
            return Err(TranslateError::PageNotMapped);
        }
        if p2_entry.flags().contains(PageTableFlags::HUGE_PAGE) {
            return Err(TranslateError::ParentEntryHugePage);
        }
        let p1_frame = p2_entry.frame().map_err(|_| TranslateError::ParentEntryHugePage)?;
        let p1 = unsafe { &*self.phys_offset.frame_to_pointer(p1_frame) };

        let p1_entry = &p1[page.start_address().p1_index()];
        if p1_entry.is_unused() {
            return Err(TranslateError::PageNotMapped);
        }
        PhysFrame::from_start_address(p1_entry.addr())
            .map_err(|_| TranslateError::InvalidFrameAddress(p1_entry.addr()))
    }
}

impl Mapper<Size2MiB> for OffsetPageTable5<'_> {
    #[inline]
    unsafe fn map_to_with_table_flags<A>(
        &mut self,
        page: Page<Size2MiB>,
        frame: PhysFrame<Size2MiB>,
        flags: PageTableFlags,
        parent_table_flags: PageTableFlags,
        allocator: &mut A,
    ) -> Result<MapperFlush<Size2MiB>, MapToError<Size2MiB>>
    where
        A: FrameAllocator<Size4KiB> + ?Sized,
    {
        let phys_offset = self.phys_offset;
        let p4 = self.get_or_create_p4(&page, parent_table_flags, &mut UpcastAllocator(allocator))
            .map_err(|e| match e {
                MapToError::FrameAllocationFailed => MapToError::FrameAllocationFailed,
                MapToError::ParentEntryHugePage => MapToError::ParentEntryHugePage,
                MapToError::PageAlreadyMapped(_) => MapToError::ParentEntryHugePage,
            })?;
        let mut mapper = unsafe { MappedPageTable::new(p4, phys_offset) };
        unsafe { mapper.map_to_with_table_flags(page, frame, flags, parent_table_flags, allocator) }
    }

    fn unmap(&mut self, page: Page<Size2MiB>) -> Result<(PhysFrame<Size2MiB>, MapperFlush<Size2MiB>), UnmapError> {
        let phys_offset = self.phys_offset;
        let p4 = self.get_or_create_p4(&page, PageTableFlags::PRESENT | PageTableFlags::WRITABLE, &mut NoopAllocator)
            .map_err(|_| UnmapError::PageNotMapped)?;
        let mut mapper = unsafe { MappedPageTable::new(p4, phys_offset) };
        mapper.unmap(page)
    }

    #[inline]
    unsafe fn update_flags(
        &mut self,
        page: Page<Size2MiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlush<Size2MiB>, FlagUpdateError> {
        let phys_offset = self.phys_offset;
        let p4 = self.get_or_create_p4(&page, PageTableFlags::PRESENT | PageTableFlags::WRITABLE, &mut NoopAllocator)
            .map_err(|_| FlagUpdateError::PageNotMapped)?;
        let mut mapper = unsafe { MappedPageTable::new(p4, phys_offset) };
        unsafe { mapper.update_flags(page, flags) }
    }

    #[inline]
    unsafe fn set_flags_p4_entry(
        &mut self,
        page: Page<Size2MiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlushAll, FlagUpdateError> {
        let phys_offset = self.phys_offset;
        let p4 = self.get_or_create_p4(&page, PageTableFlags::PRESENT | PageTableFlags::WRITABLE, &mut NoopAllocator)
            .map_err(|_| FlagUpdateError::PageNotMapped)?;
        let mut mapper = unsafe { MappedPageTable::new(p4, phys_offset) };
        unsafe { mapper.set_flags_p4_entry(page, flags) }
    }

    #[inline]
    unsafe fn set_flags_p3_entry(
        &mut self,
        page: Page<Size2MiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlushAll, FlagUpdateError> {
        let phys_offset = self.phys_offset;
        let p4 = self.get_or_create_p4(&page, PageTableFlags::PRESENT | PageTableFlags::WRITABLE, &mut NoopAllocator)
            .map_err(|_| FlagUpdateError::PageNotMapped)?;
        let mut mapper = unsafe { MappedPageTable::new(p4, phys_offset) };
        unsafe { mapper.set_flags_p3_entry(page, flags) }
    }

    #[inline]
    unsafe fn set_flags_p2_entry(
        &mut self,
        page: Page<Size2MiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlushAll, FlagUpdateError> {
        let phys_offset = self.phys_offset;
        let p4 = self.get_or_create_p4(&page, PageTableFlags::PRESENT | PageTableFlags::WRITABLE, &mut NoopAllocator)
            .map_err(|_| FlagUpdateError::PageNotMapped)?;
        let mut mapper = unsafe { MappedPageTable::new(p4, phys_offset) };
        unsafe { mapper.set_flags_p2_entry(page, flags) }
    }

    fn translate_page(&self, page: Page<Size2MiB>) -> Result<PhysFrame<Size2MiB>, TranslateError> {
        let p4 = self.get_p4(&page.start_address())?;

        // Walk P4 -> P3 -> P2 for 2MiB pages
        let p4_entry = &p4[page.start_address().p4_index()];
        if p4_entry.is_unused() {
            return Err(TranslateError::PageNotMapped);
        }
        let p3_frame = p4_entry.frame().map_err(|_| TranslateError::ParentEntryHugePage)?;
        let p3 = unsafe { &*self.phys_offset.frame_to_pointer(p3_frame) };

        let p3_entry = &p3[page.start_address().p3_index()];
        if p3_entry.is_unused() {
            return Err(TranslateError::PageNotMapped);
        }
        if p3_entry.flags().contains(PageTableFlags::HUGE_PAGE) {
            return Err(TranslateError::ParentEntryHugePage);
        }
        let p2_frame = p3_entry.frame().map_err(|_| TranslateError::ParentEntryHugePage)?;
        let p2 = unsafe { &*self.phys_offset.frame_to_pointer(p2_frame) };

        let p2_entry = &p2[page.start_address().p2_index()];
        if p2_entry.is_unused() {
            return Err(TranslateError::PageNotMapped);
        }
        if !p2_entry.flags().contains(PageTableFlags::HUGE_PAGE) {
            return Err(TranslateError::PageNotMapped);
        }
        PhysFrame::from_start_address(p2_entry.addr())
            .map_err(|_| TranslateError::InvalidFrameAddress(p2_entry.addr()))
    }
}

impl Mapper<Size1GiB> for OffsetPageTable5<'_> {
    #[inline]
    unsafe fn map_to_with_table_flags<A>(
        &mut self,
        page: Page<Size1GiB>,
        frame: PhysFrame<Size1GiB>,
        flags: PageTableFlags,
        parent_table_flags: PageTableFlags,
        allocator: &mut A,
    ) -> Result<MapperFlush<Size1GiB>, MapToError<Size1GiB>>
    where
        A: FrameAllocator<Size4KiB> + ?Sized,
    {
        let phys_offset = self.phys_offset;
        let p4 = self.get_or_create_p4(&page, parent_table_flags, &mut UpcastAllocator(allocator))
            .map_err(|e| match e {
                MapToError::FrameAllocationFailed => MapToError::FrameAllocationFailed,
                MapToError::ParentEntryHugePage => MapToError::ParentEntryHugePage,
                MapToError::PageAlreadyMapped(_) => MapToError::ParentEntryHugePage,
            })?;
        let mut mapper = unsafe { MappedPageTable::new(p4, phys_offset) };
        unsafe { mapper.map_to_with_table_flags(page, frame, flags, parent_table_flags, allocator) }
    }

    fn unmap(&mut self, page: Page<Size1GiB>) -> Result<(PhysFrame<Size1GiB>, MapperFlush<Size1GiB>), UnmapError> {
        let phys_offset = self.phys_offset;
        let p4 = self.get_or_create_p4(&page, PageTableFlags::PRESENT | PageTableFlags::WRITABLE, &mut NoopAllocator)
            .map_err(|_| UnmapError::PageNotMapped)?;
        let mut mapper = unsafe { MappedPageTable::new(p4, phys_offset) };
        mapper.unmap(page)
    }

    #[inline]
    unsafe fn update_flags(
        &mut self,
        page: Page<Size1GiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlush<Size1GiB>, FlagUpdateError> {
        let phys_offset = self.phys_offset;
        let p4 = self.get_or_create_p4(&page, PageTableFlags::PRESENT | PageTableFlags::WRITABLE, &mut NoopAllocator)
            .map_err(|_| FlagUpdateError::PageNotMapped)?;
        let mut mapper = unsafe { MappedPageTable::new(p4, phys_offset) };
        unsafe { mapper.update_flags(page, flags) }
    }

    #[inline]
    unsafe fn set_flags_p4_entry(
        &mut self,
        page: Page<Size1GiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlushAll, FlagUpdateError> {
        let phys_offset = self.phys_offset;
        let p4 = self.get_or_create_p4(&page, PageTableFlags::PRESENT | PageTableFlags::WRITABLE, &mut NoopAllocator)
            .map_err(|_| FlagUpdateError::PageNotMapped)?;
        let mut mapper = unsafe { MappedPageTable::new(p4, phys_offset) };
        unsafe { mapper.set_flags_p4_entry(page, flags) }
    }

    #[inline]
    unsafe fn set_flags_p3_entry(
        &mut self,
        page: Page<Size1GiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlushAll, FlagUpdateError> {
        let phys_offset = self.phys_offset;
        let p4 = self.get_or_create_p4(&page, PageTableFlags::PRESENT | PageTableFlags::WRITABLE, &mut NoopAllocator)
            .map_err(|_| FlagUpdateError::PageNotMapped)?;
        let mut mapper = unsafe { MappedPageTable::new(p4, phys_offset) };
        unsafe { mapper.set_flags_p3_entry(page, flags) }
    }

    #[inline]
    unsafe fn set_flags_p2_entry(
        &mut self,
        page: Page<Size1GiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlushAll, FlagUpdateError> {
        let phys_offset = self.phys_offset;
        let p4 = self.get_or_create_p4(&page, PageTableFlags::PRESENT | PageTableFlags::WRITABLE, &mut NoopAllocator)
            .map_err(|_| FlagUpdateError::PageNotMapped)?;
        let mut mapper = unsafe { MappedPageTable::new(p4, phys_offset) };
        unsafe { mapper.set_flags_p2_entry(page, flags) }
    }

    fn translate_page(&self, page: Page<Size1GiB>) -> Result<PhysFrame<Size1GiB>, TranslateError> {
        let p4 = self.get_p4(&page.start_address())?;

        // Walk P4 -> P3 for 1GiB pages
        let p4_entry = &p4[page.start_address().p4_index()];
        if p4_entry.is_unused() {
            return Err(TranslateError::PageNotMapped);
        }
        let p3_frame = p4_entry.frame().map_err(|_| TranslateError::ParentEntryHugePage)?;
        let p3 = unsafe { &*self.phys_offset.frame_to_pointer(p3_frame) };

        let p3_entry = &p3[page.start_address().p3_index()];
        if p3_entry.is_unused() {
            return Err(TranslateError::PageNotMapped);
        }
        if !p3_entry.flags().contains(PageTableFlags::HUGE_PAGE) {
            return Err(TranslateError::PageNotMapped);
        }
        PhysFrame::from_start_address(p3_entry.addr())
            .map_err(|_| TranslateError::InvalidFrameAddress(p3_entry.addr()))
    }
}

impl Translate for OffsetPageTable5<'_> {
    fn translate(&self, addr: VirtAddr) -> TranslateResult {
        let p5_entry = &self.level_5_table[addr.p5_index()];

        if p5_entry.is_unused() {
            return TranslateResult::NotMapped;
        }

        let p4_frame = match p5_entry.frame() {
            Ok(f) => f,
            Err(_) => return TranslateResult::NotMapped,
        };

        let p4_ptr = self.phys_offset.frame_to_pointer(p4_frame);
        let p4 = unsafe { &*p4_ptr };

        // Walk P4 -> P3 -> P2 -> P1
        let p4_entry = &p4[addr.p4_index()];
        if p4_entry.is_unused() {
            return TranslateResult::NotMapped;
        }

        let p3_frame = match p4_entry.frame() {
            Ok(f) => f,
            Err(_) => return TranslateResult::NotMapped,
        };
        let p3 = unsafe { &*self.phys_offset.frame_to_pointer(p3_frame) };

        let p3_entry = &p3[addr.p3_index()];
        if p3_entry.is_unused() {
            return TranslateResult::NotMapped;
        }
        if p3_entry.flags().contains(PageTableFlags::HUGE_PAGE) {
            // 1GiB page
            let frame = PhysFrame::<Size1GiB>::containing_address(p3_entry.addr());
            let offset = addr.as_u64() & 0x3fff_ffff; // 30-bit offset
            return TranslateResult::Mapped {
                frame: MappedFrame::Size1GiB(frame),
                offset,
                flags: p3_entry.flags(),
            };
        }

        let p2_frame = match p3_entry.frame() {
            Ok(f) => f,
            Err(_) => return TranslateResult::NotMapped,
        };
        let p2 = unsafe { &*self.phys_offset.frame_to_pointer(p2_frame) };

        let p2_entry = &p2[addr.p2_index()];
        if p2_entry.is_unused() {
            return TranslateResult::NotMapped;
        }
        if p2_entry.flags().contains(PageTableFlags::HUGE_PAGE) {
            // 2MiB page
            let frame = PhysFrame::<Size2MiB>::containing_address(p2_entry.addr());
            let offset = addr.as_u64() & 0x1f_ffff; // 21-bit offset
            return TranslateResult::Mapped {
                frame: MappedFrame::Size2MiB(frame),
                offset,
                flags: p2_entry.flags(),
            };
        }

        let p1_frame = match p2_entry.frame() {
            Ok(f) => f,
            Err(_) => return TranslateResult::NotMapped,
        };
        let p1 = unsafe { &*self.phys_offset.frame_to_pointer(p1_frame) };

        let p1_entry = &p1[addr.p1_index()];
        if p1_entry.is_unused() {
            return TranslateResult::NotMapped;
        }

        let frame = PhysFrame::containing_address(p1_entry.addr());
        let offset = u64::from(addr.page_offset());
        TranslateResult::Mapped {
            frame: MappedFrame::Size4KiB(frame),
            offset,
            flags: p1_entry.flags(),
        }
    }
}

// Helper struct for allocator upcasting
struct UpcastAllocator<'a, A: FrameAllocator<Size4KiB> + ?Sized>(&'a mut A);

unsafe impl<A: FrameAllocator<Size4KiB> + ?Sized> FrameAllocator<Size4KiB> for UpcastAllocator<'_, A> {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        self.0.allocate_frame()
    }
}

// Noop allocator for operations that don't need to allocate
struct NoopAllocator;

unsafe impl FrameAllocator<Size4KiB> for NoopAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        None
    }
}
