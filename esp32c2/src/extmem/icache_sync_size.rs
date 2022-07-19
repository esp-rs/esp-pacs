#[doc = "Register `ICACHE_SYNC_SIZE` reader"]
pub struct R(crate::R<ICACHE_SYNC_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICACHE_SYNC_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICACHE_SYNC_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICACHE_SYNC_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICACHE_SYNC_SIZE` writer"]
pub struct W(crate::W<ICACHE_SYNC_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICACHE_SYNC_SIZE_SPEC>;
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
impl From<crate::W<ICACHE_SYNC_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICACHE_SYNC_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICACHE_SYNC_SIZE` reader - The bits are used to configure the length for sync operations. The bits are the counts of cache block. It should be combined with ICACHE_SYNC_ADDR_REG."]
pub type ICACHE_SYNC_SIZE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ICACHE_SYNC_SIZE` writer - The bits are used to configure the length for sync operations. The bits are the counts of cache block. It should be combined with ICACHE_SYNC_ADDR_REG."]
pub type ICACHE_SYNC_SIZE_W<'a> =
    crate::FieldWriter<'a, u32, ICACHE_SYNC_SIZE_SPEC, u32, u32, 23, 0>;
impl R {
    #[doc = "Bits 0:22 - The bits are used to configure the length for sync operations. The bits are the counts of cache block. It should be combined with ICACHE_SYNC_ADDR_REG."]
    #[inline(always)]
    pub fn icache_sync_size(&self) -> ICACHE_SYNC_SIZE_R {
        ICACHE_SYNC_SIZE_R::new((self.bits & 0x007f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:22 - The bits are used to configure the length for sync operations. The bits are the counts of cache block. It should be combined with ICACHE_SYNC_ADDR_REG."]
    #[inline(always)]
    pub fn icache_sync_size(&mut self) -> ICACHE_SYNC_SIZE_W {
        ICACHE_SYNC_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This description will be updated in the near future.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_sync_size](index.html) module"]
pub struct ICACHE_SYNC_SIZE_SPEC;
impl crate::RegisterSpec for ICACHE_SYNC_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icache_sync_size::R](R) reader structure"]
impl crate::Readable for ICACHE_SYNC_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icache_sync_size::W](W) writer structure"]
impl crate::Writable for ICACHE_SYNC_SIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICACHE_SYNC_SIZE to value 0"]
impl crate::Resettable for ICACHE_SYNC_SIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
