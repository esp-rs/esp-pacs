#[doc = "Register `ICACHE_CTRL1` reader"]
pub struct R(crate::R<ICACHE_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICACHE_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICACHE_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICACHE_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICACHE_CTRL1` writer"]
pub struct W(crate::W<ICACHE_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICACHE_CTRL1_SPEC>;
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
impl From<crate::W<ICACHE_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICACHE_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICACHE_SHUT_CORE0_BUS` reader - The bit is used to disable core0 ibus, 0: enable, 1: disable"]
pub type ICACHE_SHUT_CORE0_BUS_R = crate::BitReader;
#[doc = "Field `ICACHE_SHUT_CORE0_BUS` writer - The bit is used to disable core0 ibus, 0: enable, 1: disable"]
pub type ICACHE_SHUT_CORE0_BUS_W<'a, const O: u8> = crate::BitWriter<'a, ICACHE_CTRL1_SPEC, O>;
#[doc = "Field `ICACHE_SHUT_CORE1_BUS` reader - The bit is used to disable core1 ibus, 0: enable, 1: disable"]
pub type ICACHE_SHUT_CORE1_BUS_R = crate::BitReader;
#[doc = "Field `ICACHE_SHUT_CORE1_BUS` writer - The bit is used to disable core1 ibus, 0: enable, 1: disable"]
pub type ICACHE_SHUT_CORE1_BUS_W<'a, const O: u8> = crate::BitWriter<'a, ICACHE_CTRL1_SPEC, O>;
impl R {
    #[doc = "Bit 0 - The bit is used to disable core0 ibus, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn icache_shut_core0_bus(&self) -> ICACHE_SHUT_CORE0_BUS_R {
        ICACHE_SHUT_CORE0_BUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to disable core1 ibus, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn icache_shut_core1_bus(&self) -> ICACHE_SHUT_CORE1_BUS_R {
        ICACHE_SHUT_CORE1_BUS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE_CTRL1")
            .field(
                "icache_shut_core0_bus",
                &format_args!("{}", self.icache_shut_core0_bus().bit()),
            )
            .field(
                "icache_shut_core1_bus",
                &format_args!("{}", self.icache_shut_core1_bus().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ICACHE_CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to disable core0 ibus, 0: enable, 1: disable"]
    #[inline(always)]
    #[must_use]
    pub fn icache_shut_core0_bus(&mut self) -> ICACHE_SHUT_CORE0_BUS_W<0> {
        ICACHE_SHUT_CORE0_BUS_W::new(self)
    }
    #[doc = "Bit 1 - The bit is used to disable core1 ibus, 0: enable, 1: disable"]
    #[inline(always)]
    #[must_use]
    pub fn icache_shut_core1_bus(&mut self) -> ICACHE_SHUT_CORE1_BUS_W<1> {
        ICACHE_SHUT_CORE1_BUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_ctrl1](index.html) module"]
pub struct ICACHE_CTRL1_SPEC;
impl crate::RegisterSpec for ICACHE_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icache_ctrl1::R](R) reader structure"]
impl crate::Readable for ICACHE_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icache_ctrl1::W](W) writer structure"]
impl crate::Writable for ICACHE_CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICACHE_CTRL1 to value 0x03"]
impl crate::Resettable for ICACHE_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
