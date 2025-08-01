#[cfg(feature = "UI_WebUI_Core")]
pub mod Core;
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivatedDeferral(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ActivatedDeferral, windows_core::IUnknown, windows_core::IInspectable);
impl ActivatedDeferral {
    pub fn Complete(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Complete)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for ActivatedDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IActivatedDeferral>();
}
unsafe impl windows_core::Interface for ActivatedDeferral {
    type Vtable = <IActivatedDeferral as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IActivatedDeferral as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ActivatedDeferral {
    const NAME: &'static str = "Windows.UI.WebUI.ActivatedDeferral";
}
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::define_interface!(ActivatedEventHandler, ActivatedEventHandler_Vtbl, 0x50f1e730_c5d1_4b6b_9adb_8a11756be29c);
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for ActivatedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ActivatedEventHandler {
    pub fn new<F: FnMut(windows_core::Ref<'_, windows_core::IInspectable>, windows_core::Ref<'_, super::super::ApplicationModel::Activation::IActivatedEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = ActivatedEventHandlerBox { vtable: &ActivatedEventHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, eventargs: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<super::super::ApplicationModel::Activation::IActivatedEventArgs>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi(), eventargs.param().abi()).ok() }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(C)]
#[doc(hidden)]
pub struct ActivatedEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, eventargs: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(C)]
struct ActivatedEventHandlerBox<F: FnMut(windows_core::Ref<'_, windows_core::IInspectable>, windows_core::Ref<'_, super::super::ApplicationModel::Activation::IActivatedEventArgs>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const ActivatedEventHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<F: FnMut(windows_core::Ref<'_, windows_core::IInspectable>, windows_core::Ref<'_, super::super::ApplicationModel::Activation::IActivatedEventArgs>) -> windows_core::Result<()> + Send + 'static> ActivatedEventHandlerBox<F> {
    const VTABLE: ActivatedEventHandler_Vtbl = ActivatedEventHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid == <ActivatedEventHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID {
                &mut (*this).vtable as *mut _ as _
            } else if *iid == <windows_core::imp::IMarshal as windows_core::Interface>::IID {
                (*this).count.add_ref();
                return windows_core::imp::marshaler(core::mem::transmute(&mut (*this).vtable as *mut _ as *mut core::ffi::c_void), interface);
            } else {
                core::ptr::null_mut()
            };
            if (*interface).is_null() {
                windows_core::HRESULT(-2147467262)
            } else {
                (*this).count.add_ref();
                windows_core::HRESULT(0)
            }
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            (*this).count.add_ref()
        }
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            let remaining = (*this).count.release();
            if remaining == 0 {
                let _ = windows_core::imp::Box::from_raw(this);
            }
            remaining
        }
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, eventargs: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(core::mem::transmute_copy(&sender), core::mem::transmute_copy(&eventargs)).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivatedOperation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ActivatedOperation, windows_core::IUnknown, windows_core::IInspectable);
impl ActivatedOperation {
    pub fn GetDeferral(&self) -> windows_core::Result<ActivatedDeferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ActivatedOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IActivatedOperation>();
}
unsafe impl windows_core::Interface for ActivatedOperation {
    type Vtable = <IActivatedOperation as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IActivatedOperation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ActivatedOperation {
    const NAME: &'static str = "Windows.UI.WebUI.ActivatedOperation";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackgroundActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(BackgroundActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
impl BackgroundActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn TaskInstance(&self) -> windows_core::Result<super::super::ApplicationModel::Background::IBackgroundTaskInstance> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TaskInstance)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for BackgroundActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for BackgroundActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for BackgroundActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.BackgroundActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl Send for BackgroundActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl Sync for BackgroundActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::define_interface!(BackgroundActivatedEventHandler, BackgroundActivatedEventHandler_Vtbl, 0xedb19fbb_0761_47cc_9a77_24d7072965ca);
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for BackgroundActivatedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
impl BackgroundActivatedEventHandler {
    pub fn new<F: FnMut(windows_core::Ref<'_, windows_core::IInspectable>, windows_core::Ref<'_, super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = BackgroundActivatedEventHandlerBox { vtable: &BackgroundActivatedEventHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, eventargs: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi(), eventargs.param().abi()).ok() }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(C)]
#[doc(hidden)]
pub struct BackgroundActivatedEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, eventargs: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(C)]
struct BackgroundActivatedEventHandlerBox<F: FnMut(windows_core::Ref<'_, windows_core::IInspectable>, windows_core::Ref<'_, super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const BackgroundActivatedEventHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<F: FnMut(windows_core::Ref<'_, windows_core::IInspectable>, windows_core::Ref<'_, super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs>) -> windows_core::Result<()> + Send + 'static> BackgroundActivatedEventHandlerBox<F> {
    const VTABLE: BackgroundActivatedEventHandler_Vtbl = BackgroundActivatedEventHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid == <BackgroundActivatedEventHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID {
                &mut (*this).vtable as *mut _ as _
            } else if *iid == <windows_core::imp::IMarshal as windows_core::Interface>::IID {
                (*this).count.add_ref();
                return windows_core::imp::marshaler(core::mem::transmute(&mut (*this).vtable as *mut _ as *mut core::ffi::c_void), interface);
            } else {
                core::ptr::null_mut()
            };
            if (*interface).is_null() {
                windows_core::HRESULT(-2147467262)
            } else {
                (*this).count.add_ref();
                windows_core::HRESULT(0)
            }
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            (*this).count.add_ref()
        }
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            let remaining = (*this).count.release();
            if remaining == 0 {
                let _ = windows_core::imp::Box::from_raw(this);
            }
            remaining
        }
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, eventargs: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(core::mem::transmute_copy(&sender), core::mem::transmute_copy(&eventargs)).into()
        }
    }
}
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnteredBackgroundEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel")]
windows_core::imp::interface_hierarchy!(EnteredBackgroundEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::IEnteredBackgroundEventArgs);
#[cfg(feature = "ApplicationModel")]
impl EnteredBackgroundEventArgs {
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel")]
impl windows_core::RuntimeType for EnteredBackgroundEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::IEnteredBackgroundEventArgs>();
}
#[cfg(feature = "ApplicationModel")]
unsafe impl windows_core::Interface for EnteredBackgroundEventArgs {
    type Vtable = <super::super::ApplicationModel::IEnteredBackgroundEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::IEnteredBackgroundEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel")]
impl windows_core::RuntimeName for EnteredBackgroundEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.EnteredBackgroundEventArgs";
}
#[cfg(feature = "ApplicationModel")]
unsafe impl Send for EnteredBackgroundEventArgs {}
#[cfg(feature = "ApplicationModel")]
unsafe impl Sync for EnteredBackgroundEventArgs {}
#[cfg(feature = "ApplicationModel")]
windows_core::imp::define_interface!(EnteredBackgroundEventHandler, EnteredBackgroundEventHandler_Vtbl, 0x2b09a173_b68e_4def_88c1_8de84e5aab2f);
#[cfg(feature = "ApplicationModel")]
impl windows_core::RuntimeType for EnteredBackgroundEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "ApplicationModel")]
impl EnteredBackgroundEventHandler {
    pub fn new<F: FnMut(windows_core::Ref<'_, windows_core::IInspectable>, windows_core::Ref<'_, super::super::ApplicationModel::IEnteredBackgroundEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = EnteredBackgroundEventHandlerBox { vtable: &EnteredBackgroundEventHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<super::super::ApplicationModel::IEnteredBackgroundEventArgs>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi(), e.param().abi()).ok() }
    }
}
#[cfg(feature = "ApplicationModel")]
#[repr(C)]
#[doc(hidden)]
pub struct EnteredBackgroundEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "ApplicationModel")]
#[repr(C)]
struct EnteredBackgroundEventHandlerBox<F: FnMut(windows_core::Ref<'_, windows_core::IInspectable>, windows_core::Ref<'_, super::super::ApplicationModel::IEnteredBackgroundEventArgs>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const EnteredBackgroundEventHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
#[cfg(feature = "ApplicationModel")]
impl<F: FnMut(windows_core::Ref<'_, windows_core::IInspectable>, windows_core::Ref<'_, super::super::ApplicationModel::IEnteredBackgroundEventArgs>) -> windows_core::Result<()> + Send + 'static> EnteredBackgroundEventHandlerBox<F> {
    const VTABLE: EnteredBackgroundEventHandler_Vtbl = EnteredBackgroundEventHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid == <EnteredBackgroundEventHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID {
                &mut (*this).vtable as *mut _ as _
            } else if *iid == <windows_core::imp::IMarshal as windows_core::Interface>::IID {
                (*this).count.add_ref();
                return windows_core::imp::marshaler(core::mem::transmute(&mut (*this).vtable as *mut _ as *mut core::ffi::c_void), interface);
            } else {
                core::ptr::null_mut()
            };
            if (*interface).is_null() {
                windows_core::HRESULT(-2147467262)
            } else {
                (*this).count.add_ref();
                windows_core::HRESULT(0)
            }
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            (*this).count.add_ref()
        }
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            let remaining = (*this).count.release();
            if remaining == 0 {
                let _ = windows_core::imp::Box::from_raw(this);
            }
            remaining
        }
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(core::mem::transmute_copy(&sender), core::mem::transmute_copy(&e)).into()
        }
    }
}
#[cfg(feature = "Graphics_Printing")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HtmlPrintDocumentSource(windows_core::IUnknown);
#[cfg(feature = "Graphics_Printing")]
windows_core::imp::interface_hierarchy!(HtmlPrintDocumentSource, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Graphics_Printing")]
windows_core::imp::required_hierarchy!(HtmlPrintDocumentSource, super::super::Foundation::IClosable, super::super::Graphics::Printing::IPrintDocumentSource);
#[cfg(feature = "Graphics_Printing")]
impl HtmlPrintDocumentSource {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Content(&self) -> windows_core::Result<PrintContent> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Content)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetContent(&self, value: PrintContent) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContent)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LeftMargin(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LeftMargin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLeftMargin(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLeftMargin)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TopMargin(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TopMargin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTopMargin(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTopMargin)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RightMargin(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RightMargin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRightMargin(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRightMargin)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BottomMargin(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BottomMargin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBottomMargin(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBottomMargin)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EnableHeaderFooter(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnableHeaderFooter)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetEnableHeaderFooter(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEnableHeaderFooter)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShrinkToFit(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShrinkToFit)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetShrinkToFit(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetShrinkToFit)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PercentScale(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PercentScale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPercentScale(&self, scalepercent: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPercentScale)(windows_core::Interface::as_raw(this), scalepercent).ok() }
    }
    pub fn PageRange(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PageRange)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn TrySetPageRange(&self, strpagerange: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrySetPageRange)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(strpagerange), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Graphics_Printing")]
impl windows_core::RuntimeType for HtmlPrintDocumentSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHtmlPrintDocumentSource>();
}
#[cfg(feature = "Graphics_Printing")]
unsafe impl windows_core::Interface for HtmlPrintDocumentSource {
    type Vtable = <IHtmlPrintDocumentSource as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHtmlPrintDocumentSource as windows_core::Interface>::IID;
}
#[cfg(feature = "Graphics_Printing")]
impl windows_core::RuntimeName for HtmlPrintDocumentSource {
    const NAME: &'static str = "Windows.UI.WebUI.HtmlPrintDocumentSource";
}
#[cfg(feature = "Graphics_Printing")]
unsafe impl Send for HtmlPrintDocumentSource {}
#[cfg(feature = "Graphics_Printing")]
unsafe impl Sync for HtmlPrintDocumentSource {}
windows_core::imp::define_interface!(IActivatedDeferral, IActivatedDeferral_Vtbl, 0xc3bd1978_a431_49d8_a76a_395a4e03dcf3);
impl windows_core::RuntimeType for IActivatedDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedDeferral_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActivatedEventArgsDeferral, IActivatedEventArgsDeferral_Vtbl, 0xca6d5f74_63c2_44a6_b97b_d9a03c20bc9b);
impl windows_core::RuntimeType for IActivatedEventArgsDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IActivatedEventArgsDeferral, windows_core::IUnknown, windows_core::IInspectable);
impl IActivatedEventArgsDeferral {
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IActivatedEventArgsDeferral {
    const NAME: &'static str = "Windows.UI.WebUI.IActivatedEventArgsDeferral";
}
pub trait IActivatedEventArgsDeferral_Impl: windows_core::IUnknownImpl {
    fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation>;
}
impl IActivatedEventArgsDeferral_Vtbl {
    pub const fn new<Identity: IActivatedEventArgsDeferral_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ActivatedOperation<Identity: IActivatedEventArgsDeferral_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActivatedEventArgsDeferral_Impl::ActivatedOperation(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IActivatedEventArgsDeferral, OFFSET>(),
            ActivatedOperation: ActivatedOperation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActivatedEventArgsDeferral as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedEventArgsDeferral_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ActivatedOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActivatedOperation, IActivatedOperation_Vtbl, 0xb6a0b4bc_c6ca_42fd_9818_71904e45fed7);
impl windows_core::RuntimeType for IActivatedOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedOperation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Graphics_Printing")]
windows_core::imp::define_interface!(IHtmlPrintDocumentSource, IHtmlPrintDocumentSource_Vtbl, 0xcea6469a_0e05_467a_abc9_36ec1d4cdcb6);
#[cfg(feature = "Graphics_Printing")]
impl windows_core::RuntimeType for IHtmlPrintDocumentSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "Graphics_Printing")]
#[repr(C)]
#[doc(hidden)]
pub struct IHtmlPrintDocumentSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Content: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PrintContent) -> windows_core::HRESULT,
    pub SetContent: unsafe extern "system" fn(*mut core::ffi::c_void, PrintContent) -> windows_core::HRESULT,
    pub LeftMargin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetLeftMargin: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub TopMargin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetTopMargin: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub RightMargin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetRightMargin: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub BottomMargin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetBottomMargin: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub EnableHeaderFooter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetEnableHeaderFooter: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ShrinkToFit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetShrinkToFit: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub PercentScale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetPercentScale: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub PageRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TrySetPageRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INewWebUIViewCreatedEventArgs, INewWebUIViewCreatedEventArgs_Vtbl, 0xe8e1b216_be2b_4c9e_85e7_083143ec4be7);
impl windows_core::RuntimeType for INewWebUIViewCreatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct INewWebUIViewCreatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Web_UI")]
    pub WebUIView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Web_UI"))]
    WebUIView: usize,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub ActivatedEventArgs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    ActivatedEventArgs: usize,
    pub HasPendingNavigate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebUIActivationStatics, IWebUIActivationStatics_Vtbl, 0x351b86bd_43b3_482b_85db_35d87b517ad9);
impl windows_core::RuntimeType for IWebUIActivationStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIActivationStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub Activated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    Activated: usize,
    pub RemoveActivated: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel")]
    pub Suspending: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    Suspending: usize,
    pub RemoveSuspending: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Resuming: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveResuming: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Navigated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveNavigated: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebUIActivationStatics2, IWebUIActivationStatics2_Vtbl, 0xc8e88696_4d78_4aa4_8f06_2a9eadc6c40a);
impl windows_core::RuntimeType for IWebUIActivationStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIActivationStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel")]
    pub LeavingBackground: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    LeavingBackground: usize,
    pub RemoveLeavingBackground: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel")]
    pub EnteredBackground: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    EnteredBackground: usize,
    pub RemoveEnteredBackground: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub EnablePrelaunch: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebUIActivationStatics3, IWebUIActivationStatics3_Vtbl, 0x91abb686_1af5_4445_b49f_9459f40fc8de);
impl windows_core::RuntimeType for IWebUIActivationStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIActivationStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Core")]
    pub RequestRestartAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Core"))]
    RequestRestartAsync: usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "System"))]
    pub RequestRestartForUserAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "System")))]
    RequestRestartForUserAsync: usize,
}
windows_core::imp::define_interface!(IWebUIActivationStatics4, IWebUIActivationStatics4_Vtbl, 0x5e391429_183f_478d_8a25_67f80d03935b);
impl windows_core::RuntimeType for IWebUIActivationStatics4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIActivationStatics4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NewWebUIViewCreated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveNewWebUIViewCreated: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub BackgroundActivated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    BackgroundActivated: usize,
    pub RemoveBackgroundActivated: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebUIBackgroundTaskInstance, IWebUIBackgroundTaskInstance_Vtbl, 0x23f12c25_e2f7_4741_bc9c_394595de24dc);
impl windows_core::RuntimeType for IWebUIBackgroundTaskInstance {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IWebUIBackgroundTaskInstance, windows_core::IUnknown, windows_core::IInspectable);
impl IWebUIBackgroundTaskInstance {
    pub fn Succeeded(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Succeeded)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSucceeded(&self, succeeded: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSucceeded)(windows_core::Interface::as_raw(this), succeeded).ok() }
    }
}
impl windows_core::RuntimeName for IWebUIBackgroundTaskInstance {
    const NAME: &'static str = "Windows.UI.WebUI.IWebUIBackgroundTaskInstance";
}
pub trait IWebUIBackgroundTaskInstance_Impl: windows_core::IUnknownImpl {
    fn Succeeded(&self) -> windows_core::Result<bool>;
    fn SetSucceeded(&self, succeeded: bool) -> windows_core::Result<()>;
}
impl IWebUIBackgroundTaskInstance_Vtbl {
    pub const fn new<Identity: IWebUIBackgroundTaskInstance_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Succeeded<Identity: IWebUIBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebUIBackgroundTaskInstance_Impl::Succeeded(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSucceeded<Identity: IWebUIBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, succeeded: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebUIBackgroundTaskInstance_Impl::SetSucceeded(this, succeeded).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebUIBackgroundTaskInstance, OFFSET>(),
            Succeeded: Succeeded::<Identity, OFFSET>,
            SetSucceeded: SetSucceeded::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebUIBackgroundTaskInstance as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIBackgroundTaskInstance_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Succeeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetSucceeded: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebUIBackgroundTaskInstanceStatics, IWebUIBackgroundTaskInstanceStatics_Vtbl, 0x9c7a5291_19ae_4ca3_b94b_fe4ec744a740);
impl windows_core::RuntimeType for IWebUIBackgroundTaskInstanceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIBackgroundTaskInstanceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebUINavigatedDeferral, IWebUINavigatedDeferral_Vtbl, 0xd804204d_831f_46e2_b432_3afce211f962);
impl windows_core::RuntimeType for IWebUINavigatedDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUINavigatedDeferral_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebUINavigatedEventArgs, IWebUINavigatedEventArgs_Vtbl, 0xa75841b8_2499_4030_a69d_15d2d9cfe524);
impl windows_core::RuntimeType for IWebUINavigatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IWebUINavigatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl IWebUINavigatedEventArgs {
    pub fn NavigatedOperation(&self) -> windows_core::Result<WebUINavigatedOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NavigatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IWebUINavigatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.IWebUINavigatedEventArgs";
}
pub trait IWebUINavigatedEventArgs_Impl: windows_core::IUnknownImpl {
    fn NavigatedOperation(&self) -> windows_core::Result<WebUINavigatedOperation>;
}
impl IWebUINavigatedEventArgs_Vtbl {
    pub const fn new<Identity: IWebUINavigatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NavigatedOperation<Identity: IWebUINavigatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebUINavigatedEventArgs_Impl::NavigatedOperation(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebUINavigatedEventArgs, OFFSET>(),
            NavigatedOperation: NavigatedOperation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebUINavigatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUINavigatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NavigatedOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebUINavigatedOperation, IWebUINavigatedOperation_Vtbl, 0x7a965f08_8182_4a89_ab67_8492e8750d4b);
impl windows_core::RuntimeType for IWebUINavigatedOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUINavigatedOperation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebUIView, IWebUIView_Vtbl, 0x6783f64f_52da_4fd7_be69_8ef6284b423c);
impl windows_core::RuntimeType for IWebUIView {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ApplicationViewId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Web_UI")]
    pub Closed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "Web_UI"))]
    Closed: usize,
    pub RemoveClosed: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Web_UI"))]
    pub Activated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Activation", feature = "Web_UI")))]
    Activated: usize,
    pub RemoveActivated: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub IgnoreApplicationContentUriRulesNavigationRestrictions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIgnoreApplicationContentUriRulesNavigationRestrictions: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebUIViewStatics, IWebUIViewStatics_Vtbl, 0xb591e668_8e59_44f9_8803_1b24c9149d30);
impl windows_core::RuntimeType for IWebUIViewStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIViewStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Web_UI")]
    pub CreateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Web_UI"))]
    CreateAsync: usize,
    #[cfg(feature = "Web_UI")]
    pub CreateWithUriAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Web_UI"))]
    CreateWithUriAsync: usize,
}
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LeavingBackgroundEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel")]
windows_core::imp::interface_hierarchy!(LeavingBackgroundEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::ILeavingBackgroundEventArgs);
#[cfg(feature = "ApplicationModel")]
impl LeavingBackgroundEventArgs {
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel")]
impl windows_core::RuntimeType for LeavingBackgroundEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::ILeavingBackgroundEventArgs>();
}
#[cfg(feature = "ApplicationModel")]
unsafe impl windows_core::Interface for LeavingBackgroundEventArgs {
    type Vtable = <super::super::ApplicationModel::ILeavingBackgroundEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::ILeavingBackgroundEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel")]
impl windows_core::RuntimeName for LeavingBackgroundEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.LeavingBackgroundEventArgs";
}
#[cfg(feature = "ApplicationModel")]
unsafe impl Send for LeavingBackgroundEventArgs {}
#[cfg(feature = "ApplicationModel")]
unsafe impl Sync for LeavingBackgroundEventArgs {}
#[cfg(feature = "ApplicationModel")]
windows_core::imp::define_interface!(LeavingBackgroundEventHandler, LeavingBackgroundEventHandler_Vtbl, 0x00b4ccd9_7a9c_4b6b_9ac4_13474f268bc4);
#[cfg(feature = "ApplicationModel")]
impl windows_core::RuntimeType for LeavingBackgroundEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "ApplicationModel")]
impl LeavingBackgroundEventHandler {
    pub fn new<F: FnMut(windows_core::Ref<'_, windows_core::IInspectable>, windows_core::Ref<'_, super::super::ApplicationModel::ILeavingBackgroundEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = LeavingBackgroundEventHandlerBox { vtable: &LeavingBackgroundEventHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<super::super::ApplicationModel::ILeavingBackgroundEventArgs>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi(), e.param().abi()).ok() }
    }
}
#[cfg(feature = "ApplicationModel")]
#[repr(C)]
#[doc(hidden)]
pub struct LeavingBackgroundEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "ApplicationModel")]
#[repr(C)]
struct LeavingBackgroundEventHandlerBox<F: FnMut(windows_core::Ref<'_, windows_core::IInspectable>, windows_core::Ref<'_, super::super::ApplicationModel::ILeavingBackgroundEventArgs>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const LeavingBackgroundEventHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
#[cfg(feature = "ApplicationModel")]
impl<F: FnMut(windows_core::Ref<'_, windows_core::IInspectable>, windows_core::Ref<'_, super::super::ApplicationModel::ILeavingBackgroundEventArgs>) -> windows_core::Result<()> + Send + 'static> LeavingBackgroundEventHandlerBox<F> {
    const VTABLE: LeavingBackgroundEventHandler_Vtbl = LeavingBackgroundEventHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid == <LeavingBackgroundEventHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID {
                &mut (*this).vtable as *mut _ as _
            } else if *iid == <windows_core::imp::IMarshal as windows_core::Interface>::IID {
                (*this).count.add_ref();
                return windows_core::imp::marshaler(core::mem::transmute(&mut (*this).vtable as *mut _ as *mut core::ffi::c_void), interface);
            } else {
                core::ptr::null_mut()
            };
            if (*interface).is_null() {
                windows_core::HRESULT(-2147467262)
            } else {
                (*this).count.add_ref();
                windows_core::HRESULT(0)
            }
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            (*this).count.add_ref()
        }
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            let remaining = (*this).count.release();
            if remaining == 0 {
                let _ = windows_core::imp::Box::from_raw(this);
            }
            remaining
        }
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(core::mem::transmute_copy(&sender), core::mem::transmute_copy(&e)).into()
        }
    }
}
windows_core::imp::define_interface!(NavigatedEventHandler, NavigatedEventHandler_Vtbl, 0x7af46fe6_40ca_4e49_a7d6_dbdb330cd1a3);
impl windows_core::RuntimeType for NavigatedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl NavigatedEventHandler {
    pub fn new<F: FnMut(windows_core::Ref<'_, windows_core::IInspectable>, windows_core::Ref<'_, IWebUINavigatedEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = NavigatedEventHandlerBox { vtable: &NavigatedEventHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<IWebUINavigatedEventArgs>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi(), e.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct NavigatedEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(C)]
struct NavigatedEventHandlerBox<F: FnMut(windows_core::Ref<'_, windows_core::IInspectable>, windows_core::Ref<'_, IWebUINavigatedEventArgs>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const NavigatedEventHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(windows_core::Ref<'_, windows_core::IInspectable>, windows_core::Ref<'_, IWebUINavigatedEventArgs>) -> windows_core::Result<()> + Send + 'static> NavigatedEventHandlerBox<F> {
    const VTABLE: NavigatedEventHandler_Vtbl = NavigatedEventHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid == <NavigatedEventHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID {
                &mut (*this).vtable as *mut _ as _
            } else if *iid == <windows_core::imp::IMarshal as windows_core::Interface>::IID {
                (*this).count.add_ref();
                return windows_core::imp::marshaler(core::mem::transmute(&mut (*this).vtable as *mut _ as *mut core::ffi::c_void), interface);
            } else {
                core::ptr::null_mut()
            };
            if (*interface).is_null() {
                windows_core::HRESULT(-2147467262)
            } else {
                (*this).count.add_ref();
                windows_core::HRESULT(0)
            }
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            (*this).count.add_ref()
        }
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            let remaining = (*this).count.release();
            if remaining == 0 {
                let _ = windows_core::imp::Box::from_raw(this);
            }
            remaining
        }
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(core::mem::transmute_copy(&sender), core::mem::transmute_copy(&e)).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NewWebUIViewCreatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NewWebUIViewCreatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl NewWebUIViewCreatedEventArgs {
    #[cfg(feature = "Web_UI")]
    pub fn WebUIView(&self) -> windows_core::Result<WebUIView> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WebUIView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ActivatedEventArgs(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::IActivatedEventArgs> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedEventArgs)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HasPendingNavigate(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasPendingNavigate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for NewWebUIViewCreatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INewWebUIViewCreatedEventArgs>();
}
unsafe impl windows_core::Interface for NewWebUIViewCreatedEventArgs {
    type Vtable = <INewWebUIViewCreatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INewWebUIViewCreatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NewWebUIViewCreatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.NewWebUIViewCreatedEventArgs";
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PrintContent(pub i32);
impl PrintContent {
    pub const AllPages: Self = Self(0i32);
    pub const CurrentPage: Self = Self(1i32);
    pub const CustomPageRange: Self = Self(2i32);
    pub const CurrentSelection: Self = Self(3i32);
}
impl windows_core::TypeKind for PrintContent {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PrintContent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.WebUI.PrintContent;i4)");
}
windows_core::imp::define_interface!(ResumingEventHandler, ResumingEventHandler_Vtbl, 0x26599ba9_a22d_4806_a728_acadc1d075fa);
impl windows_core::RuntimeType for ResumingEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ResumingEventHandler {
    pub fn new<F: FnMut(windows_core::Ref<'_, windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = ResumingEventHandlerBox { vtable: &ResumingEventHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, sender: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ResumingEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(C)]
struct ResumingEventHandlerBox<F: FnMut(windows_core::Ref<'_, windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const ResumingEventHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(windows_core::Ref<'_, windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static> ResumingEventHandlerBox<F> {
    const VTABLE: ResumingEventHandler_Vtbl = ResumingEventHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid == <ResumingEventHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID {
                &mut (*this).vtable as *mut _ as _
            } else if *iid == <windows_core::imp::IMarshal as windows_core::Interface>::IID {
                (*this).count.add_ref();
                return windows_core::imp::marshaler(core::mem::transmute(&mut (*this).vtable as *mut _ as *mut core::ffi::c_void), interface);
            } else {
                core::ptr::null_mut()
            };
            if (*interface).is_null() {
                windows_core::HRESULT(-2147467262)
            } else {
                (*this).count.add_ref();
                windows_core::HRESULT(0)
            }
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            (*this).count.add_ref()
        }
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            let remaining = (*this).count.release();
            if remaining == 0 {
                let _ = windows_core::imp::Box::from_raw(this);
            }
            remaining
        }
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(core::mem::transmute_copy(&sender)).into()
        }
    }
}
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SuspendingDeferral(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel")]
windows_core::imp::interface_hierarchy!(SuspendingDeferral, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::ISuspendingDeferral);
#[cfg(feature = "ApplicationModel")]
impl SuspendingDeferral {
    pub fn Complete(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Complete)(windows_core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "ApplicationModel")]
impl windows_core::RuntimeType for SuspendingDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::ISuspendingDeferral>();
}
#[cfg(feature = "ApplicationModel")]
unsafe impl windows_core::Interface for SuspendingDeferral {
    type Vtable = <super::super::ApplicationModel::ISuspendingDeferral as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::ISuspendingDeferral as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel")]
impl windows_core::RuntimeName for SuspendingDeferral {
    const NAME: &'static str = "Windows.UI.WebUI.SuspendingDeferral";
}
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SuspendingEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel")]
windows_core::imp::interface_hierarchy!(SuspendingEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::ISuspendingEventArgs);
#[cfg(feature = "ApplicationModel")]
impl SuspendingEventArgs {
    pub fn SuspendingOperation(&self) -> windows_core::Result<super::super::ApplicationModel::SuspendingOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SuspendingOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel")]
impl windows_core::RuntimeType for SuspendingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::ISuspendingEventArgs>();
}
#[cfg(feature = "ApplicationModel")]
unsafe impl windows_core::Interface for SuspendingEventArgs {
    type Vtable = <super::super::ApplicationModel::ISuspendingEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::ISuspendingEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel")]
impl windows_core::RuntimeName for SuspendingEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.SuspendingEventArgs";
}
#[cfg(feature = "ApplicationModel")]
windows_core::imp::define_interface!(SuspendingEventHandler, SuspendingEventHandler_Vtbl, 0x509c429c_78e2_4883_abc8_8960dcde1b5c);
#[cfg(feature = "ApplicationModel")]
impl windows_core::RuntimeType for SuspendingEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "ApplicationModel")]
impl SuspendingEventHandler {
    pub fn new<F: FnMut(windows_core::Ref<'_, windows_core::IInspectable>, windows_core::Ref<'_, super::super::ApplicationModel::ISuspendingEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = SuspendingEventHandlerBox { vtable: &SuspendingEventHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<super::super::ApplicationModel::ISuspendingEventArgs>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi(), e.param().abi()).ok() }
    }
}
#[cfg(feature = "ApplicationModel")]
#[repr(C)]
#[doc(hidden)]
pub struct SuspendingEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "ApplicationModel")]
#[repr(C)]
struct SuspendingEventHandlerBox<F: FnMut(windows_core::Ref<'_, windows_core::IInspectable>, windows_core::Ref<'_, super::super::ApplicationModel::ISuspendingEventArgs>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const SuspendingEventHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
#[cfg(feature = "ApplicationModel")]
impl<F: FnMut(windows_core::Ref<'_, windows_core::IInspectable>, windows_core::Ref<'_, super::super::ApplicationModel::ISuspendingEventArgs>) -> windows_core::Result<()> + Send + 'static> SuspendingEventHandlerBox<F> {
    const VTABLE: SuspendingEventHandler_Vtbl = SuspendingEventHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid == <SuspendingEventHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID {
                &mut (*this).vtable as *mut _ as _
            } else if *iid == <windows_core::imp::IMarshal as windows_core::Interface>::IID {
                (*this).count.add_ref();
                return windows_core::imp::marshaler(core::mem::transmute(&mut (*this).vtable as *mut _ as *mut core::ffi::c_void), interface);
            } else {
                core::ptr::null_mut()
            };
            if (*interface).is_null() {
                windows_core::HRESULT(-2147467262)
            } else {
                (*this).count.add_ref();
                windows_core::HRESULT(0)
            }
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            (*this).count.add_ref()
        }
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            let remaining = (*this).count.release();
            if remaining == 0 {
                let _ = windows_core::imp::Box::from_raw(this);
            }
            remaining
        }
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(core::mem::transmute_copy(&sender), core::mem::transmute_copy(&e)).into()
        }
    }
}
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SuspendingOperation(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel")]
windows_core::imp::interface_hierarchy!(SuspendingOperation, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::ISuspendingOperation);
#[cfg(feature = "ApplicationModel")]
impl SuspendingOperation {
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::ApplicationModel::SuspendingDeferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Deadline(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Deadline)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "ApplicationModel")]
impl windows_core::RuntimeType for SuspendingOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::ISuspendingOperation>();
}
#[cfg(feature = "ApplicationModel")]
unsafe impl windows_core::Interface for SuspendingOperation {
    type Vtable = <super::super::ApplicationModel::ISuspendingOperation as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::ISuspendingOperation as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel")]
impl windows_core::RuntimeName for SuspendingOperation {
    const NAME: &'static str = "Windows.UI.WebUI.SuspendingOperation";
}
pub struct WebUIApplication;
impl WebUIApplication {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Activated<P0>(handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<ActivatedEventHandler>,
    {
        Self::IWebUIActivationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Activated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveActivated(token: i64) -> windows_core::Result<()> {
        Self::IWebUIActivationStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveActivated)(windows_core::Interface::as_raw(this), token).ok() })
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn Suspending<P0>(handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<SuspendingEventHandler>,
    {
        Self::IWebUIActivationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Suspending)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveSuspending(token: i64) -> windows_core::Result<()> {
        Self::IWebUIActivationStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveSuspending)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn Resuming<P0>(handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<ResumingEventHandler>,
    {
        Self::IWebUIActivationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Resuming)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveResuming(token: i64) -> windows_core::Result<()> {
        Self::IWebUIActivationStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveResuming)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn Navigated<P0>(handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<NavigatedEventHandler>,
    {
        Self::IWebUIActivationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Navigated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveNavigated(token: i64) -> windows_core::Result<()> {
        Self::IWebUIActivationStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveNavigated)(windows_core::Interface::as_raw(this), token).ok() })
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn LeavingBackground<P0>(handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<LeavingBackgroundEventHandler>,
    {
        Self::IWebUIActivationStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LeavingBackground)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveLeavingBackground(token: i64) -> windows_core::Result<()> {
        Self::IWebUIActivationStatics2(|this| unsafe { (windows_core::Interface::vtable(this).RemoveLeavingBackground)(windows_core::Interface::as_raw(this), token).ok() })
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn EnteredBackground<P0>(handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<EnteredBackgroundEventHandler>,
    {
        Self::IWebUIActivationStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnteredBackground)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveEnteredBackground(token: i64) -> windows_core::Result<()> {
        Self::IWebUIActivationStatics2(|this| unsafe { (windows_core::Interface::vtable(this).RemoveEnteredBackground)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn EnablePrelaunch(value: bool) -> windows_core::Result<()> {
        Self::IWebUIActivationStatics2(|this| unsafe { (windows_core::Interface::vtable(this).EnablePrelaunch)(windows_core::Interface::as_raw(this), value).ok() })
    }
    #[cfg(feature = "ApplicationModel_Core")]
    pub fn RequestRestartAsync(launcharguments: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<super::super::ApplicationModel::Core::AppRestartFailureReason>> {
        Self::IWebUIActivationStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestRestartAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(launcharguments), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "ApplicationModel_Core", feature = "System"))]
    pub fn RequestRestartForUserAsync<P0>(user: P0, launcharguments: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<super::super::ApplicationModel::Core::AppRestartFailureReason>>
    where
        P0: windows_core::Param<super::super::System::User>,
    {
        Self::IWebUIActivationStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestRestartForUserAsync)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(launcharguments), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn NewWebUIViewCreated<P0>(handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<NewWebUIViewCreatedEventArgs>>,
    {
        Self::IWebUIActivationStatics4(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NewWebUIViewCreated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveNewWebUIViewCreated(token: i64) -> windows_core::Result<()> {
        Self::IWebUIActivationStatics4(|this| unsafe { (windows_core::Interface::vtable(this).RemoveNewWebUIViewCreated)(windows_core::Interface::as_raw(this), token).ok() })
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn BackgroundActivated<P0>(handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<BackgroundActivatedEventHandler>,
    {
        Self::IWebUIActivationStatics4(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackgroundActivated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveBackgroundActivated(token: i64) -> windows_core::Result<()> {
        Self::IWebUIActivationStatics4(|this| unsafe { (windows_core::Interface::vtable(this).RemoveBackgroundActivated)(windows_core::Interface::as_raw(this), token).ok() })
    }
    fn IWebUIActivationStatics<R, F: FnOnce(&IWebUIActivationStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebUIApplication, IWebUIActivationStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IWebUIActivationStatics2<R, F: FnOnce(&IWebUIActivationStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebUIApplication, IWebUIActivationStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IWebUIActivationStatics3<R, F: FnOnce(&IWebUIActivationStatics3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebUIApplication, IWebUIActivationStatics3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IWebUIActivationStatics4<R, F: FnOnce(&IWebUIActivationStatics4) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebUIApplication, IWebUIActivationStatics4> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for WebUIApplication {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIApplication";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIAppointmentsProviderAddAppointmentActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIAppointmentsProviderAddAppointmentActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IAppointmentsProviderAddAppointmentActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIAppointmentsProviderAddAppointmentActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser, super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn AddAppointmentOperation(&self) -> windows_core::Result<super::super::ApplicationModel::Appointments::AppointmentsProvider::AddAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddAppointmentOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IAppointmentsProviderAddAppointmentActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IAppointmentsProviderAddAppointmentActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IAppointmentsProviderAddAppointmentActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIAppointmentsProviderAddAppointmentActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IAppointmentsProviderRemoveAppointmentActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser, super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn RemoveAppointmentOperation(&self) -> windows_core::Result<super::super::ApplicationModel::Appointments::AppointmentsProvider::RemoveAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveAppointmentOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IAppointmentsProviderRemoveAppointmentActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IAppointmentsProviderRemoveAppointmentActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IAppointmentsProviderRemoveAppointmentActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IAppointmentsProviderReplaceAppointmentActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser, super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn ReplaceAppointmentOperation(&self) -> windows_core::Result<super::super::ApplicationModel::Appointments::AppointmentsProvider::ReplaceAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReplaceAppointmentOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IAppointmentsProviderReplaceAppointmentActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IAppointmentsProviderReplaceAppointmentActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IAppointmentsProviderReplaceAppointmentActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser, super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn InstanceStartDate(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstanceStartDate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LocalId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn RoamingId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoamingId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IAppointmentsProviderShowTimeFrameActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser, super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn TimeToShow(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeToShow)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Duration(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IAppointmentsProviderShowTimeFrameActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IAppointmentsProviderShowTimeFrameActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IAppointmentsProviderShowTimeFrameActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs";
}
pub struct WebUIBackgroundTaskInstance;
impl WebUIBackgroundTaskInstance {
    pub fn Current() -> windows_core::Result<IWebUIBackgroundTaskInstance> {
        Self::IWebUIBackgroundTaskInstanceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Current)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IWebUIBackgroundTaskInstanceStatics<R, F: FnOnce(&IWebUIBackgroundTaskInstanceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebUIBackgroundTaskInstance, IWebUIBackgroundTaskInstanceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for WebUIBackgroundTaskInstance {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIBackgroundTaskInstance";
}
#[cfg(feature = "ApplicationModel_Background")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIBackgroundTaskInstanceRuntimeClass(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Background")]
windows_core::imp::interface_hierarchy!(WebUIBackgroundTaskInstanceRuntimeClass, windows_core::IUnknown, windows_core::IInspectable, IWebUIBackgroundTaskInstance);
#[cfg(feature = "ApplicationModel_Background")]
windows_core::imp::required_hierarchy!(WebUIBackgroundTaskInstanceRuntimeClass, super::super::ApplicationModel::Background::IBackgroundTaskInstance);
#[cfg(feature = "ApplicationModel_Background")]
impl WebUIBackgroundTaskInstanceRuntimeClass {
    pub fn InstanceId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Background::IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstanceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Task(&self) -> windows_core::Result<super::super::ApplicationModel::Background::BackgroundTaskRegistration> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Background::IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Task)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Progress(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Background::IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Progress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetProgress(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Background::IBackgroundTaskInstance>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProgress)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TriggerDetails(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Background::IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TriggerDetails)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Canceled<P0>(&self, cancelhandler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::ApplicationModel::Background::BackgroundTaskCanceledEventHandler>,
    {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Background::IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Canceled)(windows_core::Interface::as_raw(this), cancelhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCanceled(&self, cookie: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Background::IBackgroundTaskInstance>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveCanceled)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn SuspendedCount(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Background::IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SuspendedCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::ApplicationModel::Background::BackgroundTaskDeferral> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Background::IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Succeeded(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Succeeded)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSucceeded(&self, succeeded: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSucceeded)(windows_core::Interface::as_raw(this), succeeded).ok() }
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl windows_core::RuntimeType for WebUIBackgroundTaskInstanceRuntimeClass {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebUIBackgroundTaskInstance>();
}
#[cfg(feature = "ApplicationModel_Background")]
unsafe impl windows_core::Interface for WebUIBackgroundTaskInstanceRuntimeClass {
    type Vtable = <IWebUIBackgroundTaskInstance as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWebUIBackgroundTaskInstance as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Background")]
impl windows_core::RuntimeName for WebUIBackgroundTaskInstanceRuntimeClass {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIBackgroundTaskInstanceRuntimeClass";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIBarcodeScannerPreviewActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIBarcodeScannerPreviewActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IBarcodeScannerPreviewActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIBarcodeScannerPreviewActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIBarcodeScannerPreviewActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConnectionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIBarcodeScannerPreviewActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IBarcodeScannerPreviewActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIBarcodeScannerPreviewActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IBarcodeScannerPreviewActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IBarcodeScannerPreviewActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIBarcodeScannerPreviewActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIBarcodeScannerPreviewActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl Send for WebUIBarcodeScannerPreviewActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl Sync for WebUIBarcodeScannerPreviewActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUICachedFileUpdaterActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUICachedFileUpdaterActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::ICachedFileUpdaterActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUICachedFileUpdaterActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUICachedFileUpdaterActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Provider")]
    pub fn CachedFileUpdaterUI(&self) -> windows_core::Result<super::super::Storage::Provider::CachedFileUpdaterUI> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CachedFileUpdaterUI)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUICachedFileUpdaterActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::ICachedFileUpdaterActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUICachedFileUpdaterActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::ICachedFileUpdaterActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::ICachedFileUpdaterActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUICachedFileUpdaterActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUICachedFileUpdaterActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUICameraSettingsActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUICameraSettingsActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::ICameraSettingsActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUICameraSettingsActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUICameraSettingsActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn VideoDeviceController(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoDeviceController)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn VideoDeviceExtension(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoDeviceExtension)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUICameraSettingsActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::ICameraSettingsActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUICameraSettingsActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::ICameraSettingsActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::ICameraSettingsActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUICameraSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUICameraSettingsActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUICommandLineActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUICommandLineActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::ICommandLineActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUICommandLineActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUICommandLineActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Operation(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::CommandLineActivationOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Operation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUICommandLineActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::ICommandLineActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUICommandLineActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::ICommandLineActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::ICommandLineActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUICommandLineActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUICommandLineActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl Send for WebUICommandLineActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl Sync for WebUICommandLineActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIContactCallActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIContactCallActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IContactCallActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIContactCallActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IContactActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactCallActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ServiceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ServiceUserId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceUserId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> windows_core::Result<super::super::ApplicationModel::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contact)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIContactCallActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IContactCallActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIContactCallActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IContactCallActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IContactCallActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIContactCallActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIContactMapActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIContactMapActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IContactMapActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIContactMapActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IContactActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactMapActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Address(&self) -> windows_core::Result<super::super::ApplicationModel::Contacts::ContactAddress> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Address)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> windows_core::Result<super::super::ApplicationModel::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contact)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIContactMapActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IContactMapActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIContactMapActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IContactMapActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IContactMapActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIContactMapActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactMapActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIContactMessageActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIContactMessageActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IContactMessageActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIContactMessageActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IContactActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactMessageActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ServiceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ServiceUserId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceUserId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> windows_core::Result<super::super::ApplicationModel::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contact)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIContactMessageActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IContactMessageActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIContactMessageActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IContactMessageActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IContactMessageActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIContactMessageActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactMessageActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIContactPanelActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIContactPanelActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IContactPanelActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIContactPanelActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactPanelActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn ContactPanel(&self) -> windows_core::Result<super::super::ApplicationModel::Contacts::ContactPanel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContactPanel)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> windows_core::Result<super::super::ApplicationModel::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contact)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIContactPanelActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IContactPanelActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIContactPanelActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IContactPanelActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IContactPanelActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIContactPanelActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactPanelActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl Send for WebUIContactPanelActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl Sync for WebUIContactPanelActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIContactPickerActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIContactPickerActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IContactPickerActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIContactPickerActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactPickerActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts_Provider")]
    pub fn ContactPickerUI(&self) -> windows_core::Result<super::super::ApplicationModel::Contacts::Provider::ContactPickerUI> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContactPickerUI)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIContactPickerActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IContactPickerActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIContactPickerActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IContactPickerActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IContactPickerActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIContactPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactPickerActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIContactPostActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIContactPostActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IContactPostActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIContactPostActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IContactActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactPostActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ServiceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ServiceUserId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceUserId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> windows_core::Result<super::super::ApplicationModel::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contact)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIContactPostActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IContactPostActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIContactPostActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IContactPostActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IContactPostActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIContactPostActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactPostActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIContactVideoCallActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIContactVideoCallActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IContactVideoCallActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIContactVideoCallActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IContactActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactVideoCallActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ServiceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ServiceUserId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceUserId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> windows_core::Result<super::super::ApplicationModel::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contact)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIContactVideoCallActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IContactVideoCallActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIContactVideoCallActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IContactVideoCallActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IContactVideoCallActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIContactVideoCallActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactVideoCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIDeviceActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIDeviceActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IDeviceActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIDeviceActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser, super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIDeviceActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceInformationId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceInformationId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIDeviceActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IDeviceActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIDeviceActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IDeviceActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IDeviceActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIDeviceActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIDeviceActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIDevicePairingActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIDevicePairingActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IDevicePairingActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIDevicePairingActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIDevicePairingActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceInformation(&self) -> windows_core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceInformation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIDevicePairingActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IDevicePairingActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIDevicePairingActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IDevicePairingActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IDevicePairingActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIDevicePairingActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIDevicePairingActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIDialReceiverActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIDialReceiverActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IDialReceiverActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIDialReceiverActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser, super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs, super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIDialReceiverActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AppName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Arguments(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Arguments)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn TileId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TileId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIDialReceiverActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IDialReceiverActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIDialReceiverActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IDialReceiverActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IDialReceiverActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIDialReceiverActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIDialReceiverActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIFileActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIFileActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IFileActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIFileActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser, super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs, super::super::ApplicationModel::Activation::IFileActivatedEventArgsWithNeighboringFiles);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIFileActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage")]
    pub fn Files(&self) -> windows_core::Result<windows_collections::IVectorView<super::super::Storage::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Files)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Storage_Search")]
    pub fn NeighboringFilesQuery(&self) -> windows_core::Result<super::super::Storage::Search::StorageFileQueryResult> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IFileActivatedEventArgsWithNeighboringFiles>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NeighboringFilesQuery)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIFileActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IFileActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIFileActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IFileActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IFileActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIFileActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIFileActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIFileOpenPickerActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIFileOpenPickerActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIFileOpenPickerActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser, super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs2);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIFileOpenPickerActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub fn FileOpenPickerUI(&self) -> windows_core::Result<super::super::Storage::Pickers::Provider::FileOpenPickerUI> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileOpenPickerUI)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CallerPackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallerPackageFamilyName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIFileOpenPickerActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIFileOpenPickerActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIFileOpenPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIFileOpenPickerActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIFileOpenPickerContinuationEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIFileOpenPickerContinuationEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IFileOpenPickerContinuationEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIFileOpenPickerContinuationEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser, super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIFileOpenPickerContinuationEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContinuationData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Files(&self) -> windows_core::Result<windows_collections::IVectorView<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Files)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIFileOpenPickerContinuationEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IFileOpenPickerContinuationEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIFileOpenPickerContinuationEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IFileOpenPickerContinuationEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IFileOpenPickerContinuationEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIFileOpenPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIFileOpenPickerContinuationEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIFileSavePickerActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIFileSavePickerActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIFileSavePickerActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser, super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs2);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIFileSavePickerActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub fn FileSavePickerUI(&self) -> windows_core::Result<super::super::Storage::Pickers::Provider::FileSavePickerUI> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileSavePickerUI)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CallerPackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallerPackageFamilyName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn EnterpriseId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnterpriseId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIFileSavePickerActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIFileSavePickerActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIFileSavePickerActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIFileSavePickerActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIFileSavePickerContinuationEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIFileSavePickerContinuationEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IFileSavePickerContinuationEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIFileSavePickerContinuationEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser, super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIFileSavePickerContinuationEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContinuationData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn File(&self) -> windows_core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).File)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIFileSavePickerContinuationEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IFileSavePickerContinuationEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIFileSavePickerContinuationEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IFileSavePickerContinuationEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IFileSavePickerContinuationEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIFileSavePickerContinuationEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIFileSavePickerContinuationEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIFolderPickerContinuationEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIFolderPickerContinuationEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IFolderPickerContinuationEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIFolderPickerContinuationEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser, super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIFolderPickerContinuationEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContinuationData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Search")]
    pub fn Folder(&self) -> windows_core::Result<super::super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Folder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIFolderPickerContinuationEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IFolderPickerContinuationEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIFolderPickerContinuationEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IFolderPickerContinuationEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IFolderPickerContinuationEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIFolderPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIFolderPickerContinuationEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUILaunchActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUILaunchActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUILaunchActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser, super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs, super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs2, super::super::ApplicationModel::Activation::IPrelaunchActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUILaunchActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Arguments(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Arguments)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn TileId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TileId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn TileActivatedInfo(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::TileActivatedInfo> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TileActivatedInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PrelaunchActivated(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IPrelaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrelaunchActivated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUILaunchActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUILaunchActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUILaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUILaunchActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUILockScreenActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUILockScreenActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::ILockScreenActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUILockScreenActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser, super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUILockScreenActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Info(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Info)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUILockScreenActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::ILockScreenActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUILockScreenActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::ILockScreenActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::ILockScreenActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUILockScreenActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUILockScreenActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUILockScreenCallActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUILockScreenCallActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::ILockScreenCallActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUILockScreenCallActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs, super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUILockScreenCallActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Arguments(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Arguments)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn TileId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TileId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Calls")]
    pub fn CallUI(&self) -> windows_core::Result<super::super::ApplicationModel::Calls::LockScreenCallUI> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallUI)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUILockScreenCallActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::ILockScreenCallActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUILockScreenCallActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::ILockScreenCallActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::ILockScreenCallActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUILockScreenCallActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUILockScreenCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUILockScreenComponentActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUILockScreenComponentActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUILockScreenComponentActivatedEventArgs, IActivatedEventArgsDeferral);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUILockScreenComponentActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUILockScreenComponentActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUILockScreenComponentActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUILockScreenComponentActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUILockScreenComponentActivatedEventArgs";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUINavigatedDeferral(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebUINavigatedDeferral, windows_core::IUnknown, windows_core::IInspectable);
impl WebUINavigatedDeferral {
    pub fn Complete(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Complete)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for WebUINavigatedDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebUINavigatedDeferral>();
}
unsafe impl windows_core::Interface for WebUINavigatedDeferral {
    type Vtable = <IWebUINavigatedDeferral as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWebUINavigatedDeferral as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebUINavigatedDeferral {
    const NAME: &'static str = "Windows.UI.WebUI.WebUINavigatedDeferral";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUINavigatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebUINavigatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IWebUINavigatedEventArgs);
impl WebUINavigatedEventArgs {
    pub fn NavigatedOperation(&self) -> windows_core::Result<WebUINavigatedOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NavigatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WebUINavigatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebUINavigatedEventArgs>();
}
unsafe impl windows_core::Interface for WebUINavigatedEventArgs {
    type Vtable = <IWebUINavigatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWebUINavigatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebUINavigatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUINavigatedEventArgs";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUINavigatedOperation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebUINavigatedOperation, windows_core::IUnknown, windows_core::IInspectable);
impl WebUINavigatedOperation {
    pub fn GetDeferral(&self) -> windows_core::Result<WebUINavigatedDeferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WebUINavigatedOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebUINavigatedOperation>();
}
unsafe impl windows_core::Interface for WebUINavigatedOperation {
    type Vtable = <IWebUINavigatedOperation as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWebUINavigatedOperation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebUINavigatedOperation {
    const NAME: &'static str = "Windows.UI.WebUI.WebUINavigatedOperation";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIPhoneCallActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIPhoneCallActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IPhoneCallActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIPhoneCallActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIPhoneCallActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LineId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIPhoneCallActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IPhoneCallActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIPhoneCallActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IPhoneCallActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IPhoneCallActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIPhoneCallActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIPhoneCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl Send for WebUIPhoneCallActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl Sync for WebUIPhoneCallActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIPrint3DWorkflowActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIPrint3DWorkflowActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IPrint3DWorkflowActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIPrint3DWorkflowActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIPrint3DWorkflowActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub fn Workflow(&self) -> windows_core::Result<super::super::Devices::Printers::Extensions::Print3DWorkflow> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Workflow)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIPrint3DWorkflowActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IPrint3DWorkflowActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIPrint3DWorkflowActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IPrint3DWorkflowActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IPrint3DWorkflowActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIPrint3DWorkflowActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIPrint3DWorkflowActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIPrintTaskSettingsActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIPrintTaskSettingsActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IPrintTaskSettingsActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIPrintTaskSettingsActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIPrintTaskSettingsActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub fn Configuration(&self) -> windows_core::Result<super::super::Devices::Printers::Extensions::PrintTaskConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Configuration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIPrintTaskSettingsActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IPrintTaskSettingsActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIPrintTaskSettingsActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IPrintTaskSettingsActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IPrintTaskSettingsActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIPrintTaskSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIPrintTaskSettingsActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIPrintWorkflowForegroundTaskActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIPrintWorkflowForegroundTaskActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIPrintWorkflowForegroundTaskActivatedEventArgs, IActivatedEventArgsDeferral);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIPrintWorkflowForegroundTaskActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIProtocolActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIProtocolActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIProtocolActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser, super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs, super::super::ApplicationModel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIProtocolActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Uri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CallerPackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallerPackageFamilyName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Data(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Data)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIProtocolActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIProtocolActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIProtocolActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIProtocolActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIProtocolForResultsActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIProtocolForResultsActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IProtocolForResultsActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIProtocolForResultsActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser, super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs, super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs, super::super::ApplicationModel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIProtocolForResultsActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Uri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CallerPackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallerPackageFamilyName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Data(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Data)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn ProtocolForResultsOperation(&self) -> windows_core::Result<super::super::System::ProtocolForResultsOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtocolForResultsOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIProtocolForResultsActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IProtocolForResultsActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIProtocolForResultsActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IProtocolForResultsActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IProtocolForResultsActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIProtocolForResultsActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIProtocolForResultsActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIRestrictedLaunchActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIRestrictedLaunchActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IRestrictedLaunchActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIRestrictedLaunchActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIRestrictedLaunchActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SharedContext(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SharedContext)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIRestrictedLaunchActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IRestrictedLaunchActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIRestrictedLaunchActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IRestrictedLaunchActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IRestrictedLaunchActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIRestrictedLaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIRestrictedLaunchActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUISearchActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUISearchActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::ISearchActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUISearchActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs, super::super::ApplicationModel::Activation::ISearchActivatedEventArgsWithLinguisticDetails);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUISearchActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn QueryText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QueryText)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Search")]
    pub fn LinguisticDetails(&self) -> windows_core::Result<super::super::ApplicationModel::Search::SearchPaneQueryLinguisticDetails> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::ISearchActivatedEventArgsWithLinguisticDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LinguisticDetails)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUISearchActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::ISearchActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUISearchActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::ISearchActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::ISearchActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUISearchActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUISearchActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIShareTargetActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIShareTargetActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IShareTargetActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIShareTargetActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIShareTargetActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
    pub fn ShareOperation(&self) -> windows_core::Result<super::super::ApplicationModel::DataTransfer::ShareTarget::ShareOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShareOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIShareTargetActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IShareTargetActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIShareTargetActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IShareTargetActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IShareTargetActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIShareTargetActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIShareTargetActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIStartupTaskActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIStartupTaskActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IStartupTaskActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIStartupTaskActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIStartupTaskActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TaskId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TaskId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIStartupTaskActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IStartupTaskActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIStartupTaskActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IStartupTaskActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IStartupTaskActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIStartupTaskActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIStartupTaskActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl Send for WebUIStartupTaskActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl Sync for WebUIStartupTaskActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIToastNotificationActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIToastNotificationActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IToastNotificationActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIToastNotificationActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIToastNotificationActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Argument(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Argument)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UserInput(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserInput)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIToastNotificationActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IToastNotificationActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIToastNotificationActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IToastNotificationActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IToastNotificationActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIToastNotificationActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIToastNotificationActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIUserDataAccountProviderActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIUserDataAccountProviderActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IUserDataAccountProviderActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIUserDataAccountProviderActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIUserDataAccountProviderActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
    pub fn Operation(&self) -> windows_core::Result<super::super::ApplicationModel::UserDataAccounts::Provider::IUserDataAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Operation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIUserDataAccountProviderActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IUserDataAccountProviderActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIUserDataAccountProviderActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IUserDataAccountProviderActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IUserDataAccountProviderActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIUserDataAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIUserDataAccountProviderActivatedEventArgs";
}
#[cfg(feature = "Web_UI")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIView(windows_core::IUnknown);
#[cfg(feature = "Web_UI")]
windows_core::imp::interface_hierarchy!(WebUIView, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Web_UI")]
windows_core::imp::required_hierarchy!(WebUIView, super::super::Web::UI::IWebViewControl, super::super::Web::UI::IWebViewControl2);
#[cfg(feature = "Web_UI")]
impl WebUIView {
    pub fn ApplicationViewId(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApplicationViewId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Closed<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<WebUIView, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Closed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveClosed(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveClosed)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Activated<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<WebUIView, super::super::ApplicationModel::Activation::IActivatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Activated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveActivated(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveActivated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn IgnoreApplicationContentUriRulesNavigationRestrictions(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IgnoreApplicationContentUriRulesNavigationRestrictions)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIgnoreApplicationContentUriRulesNavigationRestrictions(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIgnoreApplicationContentUriRulesNavigationRestrictions)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateAsync() -> windows_core::Result<windows_future::IAsyncOperation<WebUIView>> {
        Self::IWebUIViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithUriAsync<P0>(uri: P0) -> windows_core::Result<windows_future::IAsyncOperation<WebUIView>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        Self::IWebUIViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithUriAsync)(windows_core::Interface::as_raw(this), uri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Source(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Source)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSource<P0>(&self, source: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSource)(windows_core::Interface::as_raw(this), source.param().abi()).ok() }
    }
    pub fn DocumentTitle(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DocumentTitle)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CanGoBack(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanGoBack)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanGoForward(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanGoForward)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDefaultBackgroundColor(&self, value: super::Color) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultBackgroundColor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DefaultBackgroundColor(&self) -> windows_core::Result<super::Color> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultBackgroundColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ContainsFullScreenElement(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContainsFullScreenElement)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Settings(&self) -> windows_core::Result<super::super::Web::UI::WebViewControlSettings> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Settings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeferredPermissionRequests(&self) -> windows_core::Result<windows_collections::IVectorView<super::super::Web::UI::WebViewControlDeferredPermissionRequest>> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeferredPermissionRequests)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GoForward(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GoForward)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GoBack(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GoBack)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Refresh(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Refresh)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Navigate<P0>(&self, source: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Navigate)(windows_core::Interface::as_raw(this), source.param().abi()).ok() }
    }
    pub fn NavigateToString(&self, text: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).NavigateToString)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text)).ok() }
    }
    #[cfg(feature = "Web")]
    pub fn NavigateToLocalStreamUri<P0, P1>(&self, source: P0, streamresolver: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<super::super::Web::IUriToStreamResolver>,
    {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).NavigateToLocalStreamUri)(windows_core::Interface::as_raw(this), source.param().abi(), streamresolver.param().abi()).ok() }
    }
    #[cfg(feature = "Web_Http")]
    pub fn NavigateWithHttpRequestMessage<P0>(&self, requestmessage: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Web::Http::HttpRequestMessage>,
    {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).NavigateWithHttpRequestMessage)(windows_core::Interface::as_raw(this), requestmessage.param().abi()).ok() }
    }
    pub fn InvokeScriptAsync<P1>(&self, scriptname: &windows_core::HSTRING, arguments: P1) -> windows_core::Result<windows_future::IAsyncOperation<windows_core::HSTRING>>
    where
        P1: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InvokeScriptAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(scriptname), arguments.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CapturePreviewToStreamAsync<P0>(&self, stream: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CapturePreviewToStreamAsync)(windows_core::Interface::as_raw(this), stream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_DataTransfer")]
    pub fn CaptureSelectedContentToDataPackageAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<super::super::ApplicationModel::DataTransfer::DataPackage>> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CaptureSelectedContentToDataPackageAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BuildLocalStreamUri(&self, contentidentifier: &windows_core::HSTRING, relativepath: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BuildLocalStreamUri)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(contentidentifier), core::mem::transmute_copy(relativepath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferredPermissionRequestById(&self, id: u32, result: &mut Option<super::super::Web::UI::WebViewControlDeferredPermissionRequest>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetDeferredPermissionRequestById)(windows_core::Interface::as_raw(this), id, result as *mut _ as _).ok() }
    }
    pub fn NavigationStarting<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlNavigationStartingEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NavigationStarting)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveNavigationStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveNavigationStarting)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ContentLoading<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlContentLoadingEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentLoading)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveContentLoading(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveContentLoading)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn DOMContentLoaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlDOMContentLoadedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DOMContentLoaded)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDOMContentLoaded(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveDOMContentLoaded)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn NavigationCompleted<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlNavigationCompletedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NavigationCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveNavigationCompleted(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveNavigationCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn FrameNavigationStarting<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlNavigationStartingEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameNavigationStarting)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFrameNavigationStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveFrameNavigationStarting)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn FrameContentLoading<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlContentLoadingEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameContentLoading)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFrameContentLoading(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveFrameContentLoading)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn FrameDOMContentLoaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlDOMContentLoadedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameDOMContentLoaded)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFrameDOMContentLoaded(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveFrameDOMContentLoaded)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn FrameNavigationCompleted<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlNavigationCompletedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameNavigationCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFrameNavigationCompleted(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveFrameNavigationCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ScriptNotify<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlScriptNotifyEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScriptNotify)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveScriptNotify(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveScriptNotify)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn LongRunningScriptDetected<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlLongRunningScriptDetectedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LongRunningScriptDetected)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveLongRunningScriptDetected(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveLongRunningScriptDetected)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn UnsafeContentWarningDisplaying<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnsafeContentWarningDisplaying)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUnsafeContentWarningDisplaying(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveUnsafeContentWarningDisplaying)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn UnviewableContentIdentified<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlUnviewableContentIdentifiedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnviewableContentIdentified)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUnviewableContentIdentified(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveUnviewableContentIdentified)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PermissionRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlPermissionRequestedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PermissionRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePermissionRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePermissionRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn UnsupportedUriSchemeIdentified<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlUnsupportedUriSchemeIdentifiedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnsupportedUriSchemeIdentified)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUnsupportedUriSchemeIdentified(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveUnsupportedUriSchemeIdentified)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn NewWindowRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlNewWindowRequestedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NewWindowRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveNewWindowRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveNewWindowRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ContainsFullScreenElementChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContainsFullScreenElementChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveContainsFullScreenElementChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveContainsFullScreenElementChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn WebResourceRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlWebResourceRequestedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WebResourceRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveWebResourceRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveWebResourceRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AddInitializeScript(&self, script: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Web::UI::IWebViewControl2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AddInitializeScript)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(script)).ok() }
    }
    fn IWebUIViewStatics<R, F: FnOnce(&IWebUIViewStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebUIView, IWebUIViewStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "Web_UI")]
impl windows_core::RuntimeType for WebUIView {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebUIView>();
}
#[cfg(feature = "Web_UI")]
unsafe impl windows_core::Interface for WebUIView {
    type Vtable = <IWebUIView as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWebUIView as windows_core::Interface>::IID;
}
#[cfg(feature = "Web_UI")]
impl windows_core::RuntimeName for WebUIView {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIView";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIVoiceCommandActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIVoiceCommandActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IVoiceCommandActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIVoiceCommandActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIVoiceCommandActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_SpeechRecognition")]
    pub fn Result(&self) -> windows_core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Result)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIVoiceCommandActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IVoiceCommandActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIVoiceCommandActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IVoiceCommandActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IVoiceCommandActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIVoiceCommandActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIVoiceCommandActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIWalletActionActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIWalletActionActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IWalletActionActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIWalletActionActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIWalletActionActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ItemId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ItemId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Wallet")]
    pub fn ActionKind(&self) -> windows_core::Result<super::super::ApplicationModel::Wallet::WalletActionKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActionKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ActionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActionId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIWalletActionActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IWalletActionActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIWalletActionActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IWalletActionActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IWalletActionActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIWalletActionActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIWalletActionActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIWebAccountProviderActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIWebAccountProviderActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IWebAccountProviderActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIWebAccountProviderActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIWebAccountProviderActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Authentication_Web_Provider")]
    pub fn Operation(&self) -> windows_core::Result<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Operation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIWebAccountProviderActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IWebAccountProviderActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIWebAccountProviderActivatedEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IWebAccountProviderActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IWebAccountProviderActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIWebAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIWebAccountProviderActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebUIWebAuthenticationBrokerContinuationEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(WebUIWebAuthenticationBrokerContinuationEventArgs, windows_core::IUnknown, windows_core::IInspectable, super::super::ApplicationModel::Activation::IWebAuthenticationBrokerContinuationEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(WebUIWebAuthenticationBrokerContinuationEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, IActivatedEventArgsDeferral, super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIWebAuthenticationBrokerContinuationEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatedOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContinuationData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Authentication_Web")]
    pub fn WebAuthenticationResult(&self) -> windows_core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WebAuthenticationResult)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for WebUIWebAuthenticationBrokerContinuationEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Activation::IWebAuthenticationBrokerContinuationEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for WebUIWebAuthenticationBrokerContinuationEventArgs {
    type Vtable = <super::super::ApplicationModel::Activation::IWebAuthenticationBrokerContinuationEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Activation::IWebAuthenticationBrokerContinuationEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for WebUIWebAuthenticationBrokerContinuationEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIWebAuthenticationBrokerContinuationEventArgs";
}
