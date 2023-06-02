#[doc = "Register `DCACHE_PRELOAD_CTRL` reader"]
pub struct R(crate::R<DCACHE_PRELOAD_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCACHE_PRELOAD_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCACHE_PRELOAD_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCACHE_PRELOAD_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCACHE_PRELOAD_CTRL` writer"]
pub struct W(crate::W<DCACHE_PRELOAD_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCACHE_PRELOAD_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DCACHE_PRELOAD_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCACHE_PRELOAD_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCACHE_PRELOAD_ENA` reader - The bit is used to enable preload operation. It will be cleared by hardware after preload operation done."]
pub type DCACHE_PRELOAD_ENA_R = crate::BitReader;
#[doc = "Field `DCACHE_PRELOAD_ENA` writer - The bit is used to enable preload operation. It will be cleared by hardware after preload operation done."]
pub type DCACHE_PRELOAD_ENA_W<'a, const O: u8> = crate::BitWriter<'a, DCACHE_PRELOAD_CTRL_SPEC, O>;
#[doc = "Field `DCACHE_PRELOAD_DONE` reader - The bit is used to indicate preload operation is finished."]
pub type DCACHE_PRELOAD_DONE_R = crate::BitReader;
#[doc = "Field `DCACHE_PRELOAD_ORDER` reader - The bit is used to configure the direction of preload operation. 1: descending, 0: ascending."]
pub type DCACHE_PRELOAD_ORDER_R = crate::BitReader;
#[doc = "Field `DCACHE_PRELOAD_ORDER` writer - The bit is used to configure the direction of preload operation. 1: descending, 0: ascending."]
pub type DCACHE_PRELOAD_ORDER_W<'a, const O: u8> =
    crate::BitWriter<'a, DCACHE_PRELOAD_CTRL_SPEC, O>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable preload operation. It will be cleared by hardware after preload operation done."]
    #[inline(always)]
    pub fn dcache_preload_ena(&self) -> DCACHE_PRELOAD_ENA_R {
        DCACHE_PRELOAD_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to indicate preload operation is finished."]
    #[inline(always)]
    pub fn dcache_preload_done(&self) -> DCACHE_PRELOAD_DONE_R {
        DCACHE_PRELOAD_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to configure the direction of preload operation. 1: descending, 0: ascending."]
    #[inline(always)]
    pub fn dcache_preload_order(&self) -> DCACHE_PRELOAD_ORDER_R {
        DCACHE_PRELOAD_ORDER_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_PRELOAD_CTRL")
            .field(
                "dcache_preload_ena",
                &format_args!("{}", self.dcache_preload_ena().bit()),
            )
            .field(
                "dcache_preload_done",
                &format_args!("{}", self.dcache_preload_done().bit()),
            )
            .field(
                "dcache_preload_order",
                &format_args!("{}", self.dcache_preload_order().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DCACHE_PRELOAD_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable preload operation. It will be cleared by hardware after preload operation done."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_preload_ena(&mut self) -> DCACHE_PRELOAD_ENA_W<0> {
        DCACHE_PRELOAD_ENA_W::new(self)
    }
    #[doc = "Bit 2 - The bit is used to configure the direction of preload operation. 1: descending, 0: ascending."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_preload_order(&mut self) -> DCACHE_PRELOAD_ORDER_W<2> {
        DCACHE_PRELOAD_ORDER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcache_preload_ctrl](index.html) module"]
pub struct DCACHE_PRELOAD_CTRL_SPEC;
impl crate::RegisterSpec for DCACHE_PRELOAD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcache_preload_ctrl::R](R) reader structure"]
impl crate::Readable for DCACHE_PRELOAD_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcache_preload_ctrl::W](W) writer structure"]
impl crate::Writable for DCACHE_PRELOAD_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCACHE_PRELOAD_CTRL to value 0x02"]
impl crate::Resettable for DCACHE_PRELOAD_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
