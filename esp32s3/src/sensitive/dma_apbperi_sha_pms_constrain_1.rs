#[doc = "Register `DMA_APBPERI_SHA_PMS_CONSTRAIN_1` reader"]
pub type R = crate::R<DMA_APBPERI_SHA_PMS_CONSTRAIN_1_SPEC>;
#[doc = "Register `DMA_APBPERI_SHA_PMS_CONSTRAIN_1` writer"]
pub type W = crate::W<DMA_APBPERI_SHA_PMS_CONSTRAIN_1_SPEC>;
#[doc = "Field `DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_0` reader - sha's permission(store,load) in data region0 of SRAM"]
pub type DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_0_R = crate::FieldReader;
#[doc = "Field `DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_0` writer - sha's permission(store,load) in data region0 of SRAM"]
pub type DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_0_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_1` reader - sha's permission(store,load) in data region1 of SRAM"]
pub type DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_1_R = crate::FieldReader;
#[doc = "Field `DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_1` writer - sha's permission(store,load) in data region1 of SRAM"]
pub type DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_1_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_2` reader - sha's permission(store,load) in data region2 of SRAM"]
pub type DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_2_R = crate::FieldReader;
#[doc = "Field `DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_2` writer - sha's permission(store,load) in data region2 of SRAM"]
pub type DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_2_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_3` reader - sha's permission(store,load) in data region3 of SRAM"]
pub type DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_3_R = crate::FieldReader;
#[doc = "Field `DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_3` writer - sha's permission(store,load) in data region3 of SRAM"]
pub type DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_3_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0` reader - sha's permission(store,load) in dcache data sram block0"]
pub type DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0_R = crate::FieldReader;
#[doc = "Field `DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0` writer - sha's permission(store,load) in dcache data sram block0"]
pub type DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1` reader - sha's permission(store,load) in dcache data sram block1"]
pub type DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1_R = crate::FieldReader;
#[doc = "Field `DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1` writer - sha's permission(store,load) in dcache data sram block1"]
pub type DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - sha's permission(store,load) in data region0 of SRAM"]
    #[inline(always)]
    pub fn dma_apbperi_sha_pms_constrain_sram_pms_0(
        &self,
    ) -> DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_0_R {
        DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - sha's permission(store,load) in data region1 of SRAM"]
    #[inline(always)]
    pub fn dma_apbperi_sha_pms_constrain_sram_pms_1(
        &self,
    ) -> DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_1_R {
        DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - sha's permission(store,load) in data region2 of SRAM"]
    #[inline(always)]
    pub fn dma_apbperi_sha_pms_constrain_sram_pms_2(
        &self,
    ) -> DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_2_R {
        DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - sha's permission(store,load) in data region3 of SRAM"]
    #[inline(always)]
    pub fn dma_apbperi_sha_pms_constrain_sram_pms_3(
        &self,
    ) -> DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_3_R {
        DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - sha's permission(store,load) in dcache data sram block0"]
    #[inline(always)]
    pub fn dma_apbperi_sha_pms_constrain_sram_cachedataarray_pms_0(
        &self,
    ) -> DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0_R {
        DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - sha's permission(store,load) in dcache data sram block1"]
    #[inline(always)]
    pub fn dma_apbperi_sha_pms_constrain_sram_cachedataarray_pms_1(
        &self,
    ) -> DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1_R {
        DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1_R::new(
            ((self.bits >> 10) & 3) as u8,
        )
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_APBPERI_SHA_PMS_CONSTRAIN_1")
            .field(
                "dma_apbperi_sha_pms_constrain_sram_pms_0",
                &format_args!("{}", self.dma_apbperi_sha_pms_constrain_sram_pms_0().bits()),
            )
            .field(
                "dma_apbperi_sha_pms_constrain_sram_pms_1",
                &format_args!("{}", self.dma_apbperi_sha_pms_constrain_sram_pms_1().bits()),
            )
            .field(
                "dma_apbperi_sha_pms_constrain_sram_pms_2",
                &format_args!("{}", self.dma_apbperi_sha_pms_constrain_sram_pms_2().bits()),
            )
            .field(
                "dma_apbperi_sha_pms_constrain_sram_pms_3",
                &format_args!("{}", self.dma_apbperi_sha_pms_constrain_sram_pms_3().bits()),
            )
            .field(
                "dma_apbperi_sha_pms_constrain_sram_cachedataarray_pms_0",
                &format_args!(
                    "{}",
                    self.dma_apbperi_sha_pms_constrain_sram_cachedataarray_pms_0()
                        .bits()
                ),
            )
            .field(
                "dma_apbperi_sha_pms_constrain_sram_cachedataarray_pms_1",
                &format_args!(
                    "{}",
                    self.dma_apbperi_sha_pms_constrain_sram_cachedataarray_pms_1()
                        .bits()
                ),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_APBPERI_SHA_PMS_CONSTRAIN_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - sha's permission(store,load) in data region0 of SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn dma_apbperi_sha_pms_constrain_sram_pms_0(
        &mut self,
    ) -> DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_0_W<DMA_APBPERI_SHA_PMS_CONSTRAIN_1_SPEC, 0> {
        DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_0_W::new(self)
    }
    #[doc = "Bits 2:3 - sha's permission(store,load) in data region1 of SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn dma_apbperi_sha_pms_constrain_sram_pms_1(
        &mut self,
    ) -> DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_1_W<DMA_APBPERI_SHA_PMS_CONSTRAIN_1_SPEC, 2> {
        DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_1_W::new(self)
    }
    #[doc = "Bits 4:5 - sha's permission(store,load) in data region2 of SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn dma_apbperi_sha_pms_constrain_sram_pms_2(
        &mut self,
    ) -> DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_2_W<DMA_APBPERI_SHA_PMS_CONSTRAIN_1_SPEC, 4> {
        DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_2_W::new(self)
    }
    #[doc = "Bits 6:7 - sha's permission(store,load) in data region3 of SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn dma_apbperi_sha_pms_constrain_sram_pms_3(
        &mut self,
    ) -> DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_3_W<DMA_APBPERI_SHA_PMS_CONSTRAIN_1_SPEC, 6> {
        DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_PMS_3_W::new(self)
    }
    #[doc = "Bits 8:9 - sha's permission(store,load) in dcache data sram block0"]
    #[inline(always)]
    #[must_use]
    pub fn dma_apbperi_sha_pms_constrain_sram_cachedataarray_pms_0(
        &mut self,
    ) -> DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0_W<
        DMA_APBPERI_SHA_PMS_CONSTRAIN_1_SPEC,
        8,
    > {
        DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0_W::new(self)
    }
    #[doc = "Bits 10:11 - sha's permission(store,load) in dcache data sram block1"]
    #[inline(always)]
    #[must_use]
    pub fn dma_apbperi_sha_pms_constrain_sram_cachedataarray_pms_1(
        &mut self,
    ) -> DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1_W<
        DMA_APBPERI_SHA_PMS_CONSTRAIN_1_SPEC,
        10,
    > {
        DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "sha dma permission configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_sha_pms_constrain_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_sha_pms_constrain_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_APBPERI_SHA_PMS_CONSTRAIN_1_SPEC;
impl crate::RegisterSpec for DMA_APBPERI_SHA_PMS_CONSTRAIN_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_apbperi_sha_pms_constrain_1::R`](R) reader structure"]
impl crate::Readable for DMA_APBPERI_SHA_PMS_CONSTRAIN_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_apbperi_sha_pms_constrain_1::W`](W) writer structure"]
impl crate::Writable for DMA_APBPERI_SHA_PMS_CONSTRAIN_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_APBPERI_SHA_PMS_CONSTRAIN_1 to value 0x0fff"]
impl crate::Resettable for DMA_APBPERI_SHA_PMS_CONSTRAIN_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff;
}
