use std::pin::Pin;

/*
避免自引用：
- 当前文件为方法一
- 方法二：避免自引用，使用索引或 ID 代替引用。在许多情况下，自引用并不是唯一方案，使用容器（如 Vec）和索引访问结构通常更安全。



使用 unsafe 是有风险的，必须确保 data 不会被移动。
 */
struct SelfRef {
    data: String,
    reference: *const str, // 原生指针绕过 borrow checker
}

impl SelfRef {
    fn new(data: String) -> Pin<Box<SelfRef>> {
        let mut s = Box::pin(SelfRef {
            reference: std::ptr::null(),
            data,
        });

        let ptr = &s.data as *const String as *const str;
        unsafe {
            let mut_ref = Pin::as_mut(&mut s);
            Pin::get_unchecked_mut(mut_ref).reference = ptr;
        }

        s
    }

    fn get_ref(&self) -> &str {
        unsafe { &*self.reference }
    }
}
