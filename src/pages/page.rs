use crate::{
    object::{BucketItem, FileDetail, FileVersion, Object, ObjectItem},
    pages::{
        bucket_list::BucketListPage, help::HelpPage, initializing::InitializingPage,
        object_detail::ObjectDetailPage, object_list::ObjectListPage,
        object_preview::ObjectPreviewPage,
    },
    widget::ScrollListState,
};

#[derive(Debug)]
pub enum Page {
    Initializing(Box<InitializingPage>),
    BucketList(Box<BucketListPage>),
    ObjectList(Box<ObjectListPage>),
    ObjectDetail(Box<ObjectDetailPage>),
    ObjectPreview(Box<ObjectPreviewPage>),
    Help(Box<HelpPage>),
}

impl Page {
    pub fn of_initializing() -> Self {
        Self::Initializing(Box::new(InitializingPage::new()))
    }

    pub fn of_bucket_list(bucket_items: Vec<BucketItem>) -> Self {
        Self::BucketList(Box::new(BucketListPage::new(bucket_items)))
    }

    pub fn of_object_list(object_items: Vec<ObjectItem>) -> Self {
        Self::ObjectList(Box::new(ObjectListPage::new(object_items)))
    }

    pub fn of_object_detail(
        file_detail: FileDetail,
        file_versions: Vec<FileVersion>,
        object_items: Vec<ObjectItem>,
        list_state: ScrollListState,
    ) -> Self {
        Self::ObjectDetail(Box::new(ObjectDetailPage::new(
            file_detail,
            file_versions,
            object_items,
            list_state,
        )))
    }

    pub fn of_object_preview(file_detail: FileDetail, object: Object, path: String) -> Self {
        Self::ObjectPreview(Box::new(ObjectPreviewPage::new(file_detail, object, path)))
    }

    pub fn of_help(helps: Vec<String>) -> Self {
        Self::Help(Box::new(HelpPage::new(helps)))
    }

    pub fn as_bucket_list(&self) -> &BucketListPage {
        match self {
            Self::BucketList(page) => page,
            page => panic!("Page is not BucketList: {:?}", page),
        }
    }

    pub fn as_mut_bucket_list(&mut self) -> &mut BucketListPage {
        match self {
            Self::BucketList(page) => &mut *page,
            page => panic!("Page is not BucketList: {:?}", page),
        }
    }

    pub fn as_object_list(&self) -> &ObjectListPage {
        match self {
            Self::ObjectList(page) => page,
            page => panic!("Page is not ObjectList: {:?}", page),
        }
    }

    pub fn as_mut_object_list(&mut self) -> &mut ObjectListPage {
        match self {
            Self::ObjectList(page) => &mut *page,
            page => panic!("Page is not ObjectList: {:?}", page),
        }
    }

    pub fn as_object_detail(&self) -> &ObjectDetailPage {
        match self {
            Self::ObjectDetail(page) => page,
            page => panic!("Page is not ObjectDetail: {:?}", page),
        }
    }

    pub fn as_mut_object_detail(&mut self) -> &mut ObjectDetailPage {
        match self {
            Self::ObjectDetail(page) => &mut *page,
            page => panic!("Page is not ObjectDetail: {:?}", page),
        }
    }

    pub fn as_object_preview(&self) -> &ObjectPreviewPage {
        match self {
            Self::ObjectPreview(page) => page,
            page => panic!("Page is not ObjectPreview: {:?}", page),
        }
    }

    pub fn as_mut_object_preview(&mut self) -> &mut ObjectPreviewPage {
        match self {
            Self::ObjectPreview(page) => &mut *page,
            page => panic!("Page is not ObjectPreview: {:?}", page),
        }
    }
}

pub struct PageStack {
    stack: Vec<Page>,
}

impl PageStack {
    pub fn new() -> PageStack {
        PageStack {
            stack: vec![Page::of_initializing()],
        }
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn push(&mut self, page: Page) {
        self.stack.push(page);
    }

    pub fn pop(&mut self) -> Page {
        self.stack.pop().unwrap()
    }

    pub fn clear(&mut self) {
        self.stack.truncate(1);
    }

    pub fn current_page(&self) -> &Page {
        self.stack.last().unwrap()
    }

    pub fn current_page_mut(&mut self) -> &mut Page {
        self.stack.last_mut().unwrap()
    }

    pub fn head(&self) -> &Page {
        self.stack.first().unwrap()
    }

    pub fn iter(&self) -> std::slice::Iter<Page> {
        self.stack.iter()
    }
}