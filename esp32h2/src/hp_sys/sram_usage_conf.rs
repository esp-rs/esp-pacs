#[doc = "Register `SRAM_USAGE_CONF` reader"]
pub struct R(crate::R<SRAM_USAGE_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAM_USAGE_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAM_USAGE_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAM_USAGE_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAM_USAGE_CONF` writer"]
pub struct W(crate::W<SRAM_USAGE_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAM_USAGE_CONF_SPEC>;
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
impl From<crate::W<SRAM_USAGE_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAM_USAGE_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRAM_USAGE` reader - 0: cpu use hp-memory. 1: mac-dump accessing hp-memory."]
pub type SRAM_USAGE_R = crate::FieldReader;
#[doc = "Field `SRAM_USAGE` writer - 0: cpu use hp-memory. 1: mac-dump accessing hp-memory."]
pub type SRAM_USAGE_W<'a, const O: u8> = crate::FieldWriter<'a, SRAM_USAGE_CONF_SPEC, 5, O>;
#[doc = "Field `MAC_DUMP_ALLOC` reader - reserved."]
pub type MAC_DUMP_ALLOC_R = crate::FieldReader;
#[doc = "Field `MAC_DUMP_ALLOC` writer - reserved."]
pub type MAC_DUMP_ALLOC_W<'a, const O: u8> = crate::FieldWriter<'a, SRAM_USAGE_CONF_SPEC, 5, O>;
#[doc = "Field `CACHE_USAGE` reader - reserved"]
pub type CACHE_USAGE_R = crate::BitReader;
impl R {
    #[doc = "Bits 10:14 - 0: cpu use hp-memory. 1: mac-dump accessing hp-memory."]
    #[inline(always)]
    pub fn sram_usage(&self) -> SRAM_USAGE_R {
        SRAM_USAGE_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - reserved."]
    #[inline(always)]
    pub fn mac_dump_alloc(&self) -> MAC_DUMP_ALLOC_R {
        MAC_DUMP_ALLOC_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - reserved"]
    #[inline(always)]
    pub fn cache_usage(&self) -> CACHE_USAGE_R {
        CACHE_USAGE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_USAGE_CONF")
            .field("sram_usage", &format_args!("{}", self.sram_usage().bits()))
            .field(
                "mac_dump_alloc",
                &format_args!("{}", self.mac_dump_alloc().bits()),
            )
            .field("cache_usage", &format_args!("{}", self.cache_usage().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SRAM_USAGE_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 10:14 - 0: cpu use hp-memory. 1: mac-dump accessing hp-memory."]
    #[inline(always)]
    #[must_use]
    pub fn sram_usage(&mut self) -> SRAM_USAGE_W<10> {
        SRAM_USAGE_W::new(self)
    }
    #[doc = "Bits 20:24 - reserved."]
    #[inline(always)]
    #[must_use]
    pub fn mac_dump_alloc(&mut self) -> MAC_DUMP_ALLOC_W<20> {
        MAC_DUMP_ALLOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HP memory usage configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_usage_conf](index.html) module"]
pub struct SRAM_USAGE_CONF_SPEC;
impl crate::RegisterSpec for SRAM_USAGE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sram_usage_conf::R](R) reader structure"]
impl crate::Readable for SRAM_USAGE_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sram_usage_conf::W](W) writer structure"]
impl crate::Writable for SRAM_USAGE_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRAM_USAGE_CONF to value 0"]
impl crate::Resettable for SRAM_USAGE_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
