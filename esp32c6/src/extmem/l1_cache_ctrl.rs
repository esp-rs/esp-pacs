#[doc = "Register `L1_CACHE_CTRL` reader"]
pub struct R(crate::R<L1_CACHE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1_CACHE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1_CACHE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1_CACHE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L1_CACHE_CTRL` writer"]
pub struct W(crate::W<L1_CACHE_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1_CACHE_CTRL_SPEC>;
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
impl From<crate::W<L1_CACHE_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1_CACHE_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L1_CACHE_SHUT_BUS0` reader - The bit is used to disable core0 dbus access L1-Cache, 0: enable, 1: disable"]
pub type L1_CACHE_SHUT_BUS0_R = crate::BitReader;
#[doc = "Field `L1_CACHE_SHUT_BUS0` writer - The bit is used to disable core0 dbus access L1-Cache, 0: enable, 1: disable"]
pub type L1_CACHE_SHUT_BUS0_W<'a, const O: u8> = crate::BitWriter<'a, L1_CACHE_CTRL_SPEC, O>;
#[doc = "Field `L1_CACHE_SHUT_BUS1` reader - The bit is used to disable core1 dbus access L1-Cache, 0: enable, 1: disable"]
pub type L1_CACHE_SHUT_BUS1_R = crate::BitReader;
#[doc = "Field `L1_CACHE_SHUT_BUS1` writer - The bit is used to disable core1 dbus access L1-Cache, 0: enable, 1: disable"]
pub type L1_CACHE_SHUT_BUS1_W<'a, const O: u8> = crate::BitWriter<'a, L1_CACHE_CTRL_SPEC, O>;
#[doc = "Field `L1_CACHE_SHUT_DBUS2` reader - Reserved"]
pub type L1_CACHE_SHUT_DBUS2_R = crate::BitReader;
#[doc = "Field `L1_CACHE_SHUT_DBUS3` reader - Reserved"]
pub type L1_CACHE_SHUT_DBUS3_R = crate::BitReader;
#[doc = "Field `L1_CACHE_SHUT_DMA` reader - The bit is used to disable DMA access L1-Cache, 0: enable, 1: disable"]
pub type L1_CACHE_SHUT_DMA_R = crate::BitReader;
#[doc = "Field `L1_CACHE_UNDEF_OP` reader - Reserved"]
pub type L1_CACHE_UNDEF_OP_R = crate::FieldReader;
#[doc = "Field `L1_CACHE_UNDEF_OP` writer - Reserved"]
pub type L1_CACHE_UNDEF_OP_W<'a, const O: u8> = crate::FieldWriter<'a, L1_CACHE_CTRL_SPEC, 4, O>;
impl R {
    #[doc = "Bit 0 - The bit is used to disable core0 dbus access L1-Cache, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn l1_cache_shut_bus0(&self) -> L1_CACHE_SHUT_BUS0_R {
        L1_CACHE_SHUT_BUS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to disable core1 dbus access L1-Cache, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn l1_cache_shut_bus1(&self) -> L1_CACHE_SHUT_BUS1_R {
        L1_CACHE_SHUT_BUS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_cache_shut_dbus2(&self) -> L1_CACHE_SHUT_DBUS2_R {
        L1_CACHE_SHUT_DBUS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_cache_shut_dbus3(&self) -> L1_CACHE_SHUT_DBUS3_R {
        L1_CACHE_SHUT_DBUS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to disable DMA access L1-Cache, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn l1_cache_shut_dma(&self) -> L1_CACHE_SHUT_DMA_R {
        L1_CACHE_SHUT_DMA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Reserved"]
    #[inline(always)]
    pub fn l1_cache_undef_op(&self) -> L1_CACHE_UNDEF_OP_R {
        L1_CACHE_UNDEF_OP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_CTRL")
            .field(
                "l1_cache_shut_bus0",
                &format_args!("{}", self.l1_cache_shut_bus0().bit()),
            )
            .field(
                "l1_cache_shut_bus1",
                &format_args!("{}", self.l1_cache_shut_bus1().bit()),
            )
            .field(
                "l1_cache_shut_dbus2",
                &format_args!("{}", self.l1_cache_shut_dbus2().bit()),
            )
            .field(
                "l1_cache_shut_dbus3",
                &format_args!("{}", self.l1_cache_shut_dbus3().bit()),
            )
            .field(
                "l1_cache_shut_dma",
                &format_args!("{}", self.l1_cache_shut_dma().bit()),
            )
            .field(
                "l1_cache_undef_op",
                &format_args!("{}", self.l1_cache_undef_op().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_CACHE_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to disable core0 dbus access L1-Cache, 0: enable, 1: disable"]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_shut_bus0(&mut self) -> L1_CACHE_SHUT_BUS0_W<0> {
        L1_CACHE_SHUT_BUS0_W::new(self)
    }
    #[doc = "Bit 1 - The bit is used to disable core1 dbus access L1-Cache, 0: enable, 1: disable"]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_shut_bus1(&mut self) -> L1_CACHE_SHUT_BUS1_W<1> {
        L1_CACHE_SHUT_BUS1_W::new(self)
    }
    #[doc = "Bits 8:11 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_undef_op(&mut self) -> L1_CACHE_UNDEF_OP_W<8> {
        L1_CACHE_UNDEF_OP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "L1 data Cache(L1-Cache) control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1_cache_ctrl](index.html) module"]
pub struct L1_CACHE_CTRL_SPEC;
impl crate::RegisterSpec for L1_CACHE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1_cache_ctrl::R](R) reader structure"]
impl crate::Readable for L1_CACHE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l1_cache_ctrl::W](W) writer structure"]
impl crate::Writable for L1_CACHE_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L1_CACHE_CTRL to value 0"]
impl crate::Resettable for L1_CACHE_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
