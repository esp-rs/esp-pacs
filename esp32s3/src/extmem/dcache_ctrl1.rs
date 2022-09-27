#[doc = "Register `DCACHE_CTRL1` reader"]
pub struct R(crate::R<DCACHE_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCACHE_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCACHE_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCACHE_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCACHE_CTRL1` writer"]
pub struct W(crate::W<DCACHE_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCACHE_CTRL1_SPEC>;
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
impl From<crate::W<DCACHE_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCACHE_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCACHE_SHUT_CORE0_BUS` reader - The bit is used to disable core0 dbus, 0: enable, 1: disable"]
pub type DCACHE_SHUT_CORE0_BUS_R = crate::BitReader<bool>;
#[doc = "Field `DCACHE_SHUT_CORE0_BUS` writer - The bit is used to disable core0 dbus, 0: enable, 1: disable"]
pub type DCACHE_SHUT_CORE0_BUS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCACHE_CTRL1_SPEC, bool, O>;
#[doc = "Field `DCACHE_SHUT_CORE1_BUS` reader - The bit is used to disable core1 dbus, 0: enable, 1: disable"]
pub type DCACHE_SHUT_CORE1_BUS_R = crate::BitReader<bool>;
#[doc = "Field `DCACHE_SHUT_CORE1_BUS` writer - The bit is used to disable core1 dbus, 0: enable, 1: disable"]
pub type DCACHE_SHUT_CORE1_BUS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCACHE_CTRL1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - The bit is used to disable core0 dbus, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn dcache_shut_core0_bus(&self) -> DCACHE_SHUT_CORE0_BUS_R {
        DCACHE_SHUT_CORE0_BUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to disable core1 dbus, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn dcache_shut_core1_bus(&self) -> DCACHE_SHUT_CORE1_BUS_R {
        DCACHE_SHUT_CORE1_BUS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to disable core0 dbus, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn dcache_shut_core0_bus(&mut self) -> DCACHE_SHUT_CORE0_BUS_W<0> {
        DCACHE_SHUT_CORE0_BUS_W::new(self)
    }
    #[doc = "Bit 1 - The bit is used to disable core1 dbus, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn dcache_shut_core1_bus(&mut self) -> DCACHE_SHUT_CORE1_BUS_W<1> {
        DCACHE_SHUT_CORE1_BUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcache_ctrl1](index.html) module"]
pub struct DCACHE_CTRL1_SPEC;
impl crate::RegisterSpec for DCACHE_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcache_ctrl1::R](R) reader structure"]
impl crate::Readable for DCACHE_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcache_ctrl1::W](W) writer structure"]
impl crate::Writable for DCACHE_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCACHE_CTRL1 to value 0x03"]
impl crate::Resettable for DCACHE_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
