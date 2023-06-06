#[doc = "Register `PRO_ICACHE_MEM_SYNC0` reader"]
pub struct R(crate::R<PRO_ICACHE_MEM_SYNC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_ICACHE_MEM_SYNC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_ICACHE_MEM_SYNC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_ICACHE_MEM_SYNC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_ICACHE_MEM_SYNC0` writer"]
pub struct W(crate::W<PRO_ICACHE_MEM_SYNC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_ICACHE_MEM_SYNC0_SPEC>;
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
impl From<crate::W<PRO_ICACHE_MEM_SYNC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_ICACHE_MEM_SYNC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_ICACHE_MEMSYNC_ADDR` reader - The bits are used to configure the start virtual address for invalidate, flush, clean, lock and unlock operations. The manual operations will be issued if the address is validate. The auto operations will be issued if the address is invalidate. It should be combined with PRO_ICACHE_MEM_SYNC1."]
pub type PRO_ICACHE_MEMSYNC_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `PRO_ICACHE_MEMSYNC_ADDR` writer - The bits are used to configure the start virtual address for invalidate, flush, clean, lock and unlock operations. The manual operations will be issued if the address is validate. The auto operations will be issued if the address is invalidate. It should be combined with PRO_ICACHE_MEM_SYNC1."]
pub type PRO_ICACHE_MEMSYNC_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, PRO_ICACHE_MEM_SYNC0_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address for invalidate, flush, clean, lock and unlock operations. The manual operations will be issued if the address is validate. The auto operations will be issued if the address is invalidate. It should be combined with PRO_ICACHE_MEM_SYNC1."]
    #[inline(always)]
    pub fn pro_icache_memsync_addr(&self) -> PRO_ICACHE_MEMSYNC_ADDR_R {
        PRO_ICACHE_MEMSYNC_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_ICACHE_MEM_SYNC0")
            .field(
                "pro_icache_memsync_addr",
                &format_args!("{}", self.pro_icache_memsync_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_ICACHE_MEM_SYNC0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address for invalidate, flush, clean, lock and unlock operations. The manual operations will be issued if the address is validate. The auto operations will be issued if the address is invalidate. It should be combined with PRO_ICACHE_MEM_SYNC1."]
    #[inline(always)]
    #[must_use]
    pub fn pro_icache_memsync_addr(&mut self) -> PRO_ICACHE_MEMSYNC_ADDR_W<0> {
        PRO_ICACHE_MEMSYNC_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_icache_mem_sync0](index.html) module"]
pub struct PRO_ICACHE_MEM_SYNC0_SPEC;
impl crate::RegisterSpec for PRO_ICACHE_MEM_SYNC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_icache_mem_sync0::R](R) reader structure"]
impl crate::Readable for PRO_ICACHE_MEM_SYNC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_icache_mem_sync0::W](W) writer structure"]
impl crate::Writable for PRO_ICACHE_MEM_SYNC0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_ICACHE_MEM_SYNC0 to value 0"]
impl crate::Resettable for PRO_ICACHE_MEM_SYNC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
