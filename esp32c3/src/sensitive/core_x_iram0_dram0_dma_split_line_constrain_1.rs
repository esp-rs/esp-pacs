#[doc = "Register `CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1` reader"]
pub struct R(crate::R<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1` writer"]
pub struct W(crate::W<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_SPEC>;
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
impl From<crate::W<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_0` reader - core_x_iram0_dram0_dma_sram_category_0"]
pub type CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_0_R = crate::FieldReader;
#[doc = "Field `CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_0` writer - core_x_iram0_dram0_dma_sram_category_0"]
pub type CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_1` reader - core_x_iram0_dram0_dma_sram_category_1"]
pub type CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_1_R = crate::FieldReader;
#[doc = "Field `CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_1` writer - core_x_iram0_dram0_dma_sram_category_1"]
pub type CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_2` reader - core_x_iram0_dram0_dma_sram_category_2"]
pub type CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_2_R = crate::FieldReader;
#[doc = "Field `CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_2` writer - core_x_iram0_dram0_dma_sram_category_2"]
pub type CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_X_IRAM0_DRAM0_DMA_SRAM_SPLITADDR` reader - core_x_iram0_dram0_dma_sram_splitaddr"]
pub type CORE_X_IRAM0_DRAM0_DMA_SRAM_SPLITADDR_R = crate::FieldReader;
#[doc = "Field `CORE_X_IRAM0_DRAM0_DMA_SRAM_SPLITADDR` writer - core_x_iram0_dram0_dma_sram_splitaddr"]
pub type CORE_X_IRAM0_DRAM0_DMA_SRAM_SPLITADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:1 - core_x_iram0_dram0_dma_sram_category_0"]
    #[inline(always)]
    pub fn core_x_iram0_dram0_dma_sram_category_0(
        &self,
    ) -> CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_0_R {
        CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - core_x_iram0_dram0_dma_sram_category_1"]
    #[inline(always)]
    pub fn core_x_iram0_dram0_dma_sram_category_1(
        &self,
    ) -> CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_1_R {
        CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - core_x_iram0_dram0_dma_sram_category_2"]
    #[inline(always)]
    pub fn core_x_iram0_dram0_dma_sram_category_2(
        &self,
    ) -> CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_2_R {
        CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 14:21 - core_x_iram0_dram0_dma_sram_splitaddr"]
    #[inline(always)]
    pub fn core_x_iram0_dram0_dma_sram_splitaddr(&self) -> CORE_X_IRAM0_DRAM0_DMA_SRAM_SPLITADDR_R {
        CORE_X_IRAM0_DRAM0_DMA_SRAM_SPLITADDR_R::new(((self.bits >> 14) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1")
            .field(
                "core_x_iram0_dram0_dma_sram_category_0",
                &format_args!("{}", self.core_x_iram0_dram0_dma_sram_category_0().bits()),
            )
            .field(
                "core_x_iram0_dram0_dma_sram_category_1",
                &format_args!("{}", self.core_x_iram0_dram0_dma_sram_category_1().bits()),
            )
            .field(
                "core_x_iram0_dram0_dma_sram_category_2",
                &format_args!("{}", self.core_x_iram0_dram0_dma_sram_category_2().bits()),
            )
            .field(
                "core_x_iram0_dram0_dma_sram_splitaddr",
                &format_args!("{}", self.core_x_iram0_dram0_dma_sram_splitaddr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - core_x_iram0_dram0_dma_sram_category_0"]
    #[inline(always)]
    #[must_use]
    pub fn core_x_iram0_dram0_dma_sram_category_0(
        &mut self,
    ) -> CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_0_W<0> {
        CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_0_W::new(self)
    }
    #[doc = "Bits 2:3 - core_x_iram0_dram0_dma_sram_category_1"]
    #[inline(always)]
    #[must_use]
    pub fn core_x_iram0_dram0_dma_sram_category_1(
        &mut self,
    ) -> CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_1_W<2> {
        CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_1_W::new(self)
    }
    #[doc = "Bits 4:5 - core_x_iram0_dram0_dma_sram_category_2"]
    #[inline(always)]
    #[must_use]
    pub fn core_x_iram0_dram0_dma_sram_category_2(
        &mut self,
    ) -> CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_2_W<4> {
        CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_2_W::new(self)
    }
    #[doc = "Bits 14:21 - core_x_iram0_dram0_dma_sram_splitaddr"]
    #[inline(always)]
    #[must_use]
    pub fn core_x_iram0_dram0_dma_sram_splitaddr(
        &mut self,
    ) -> CORE_X_IRAM0_DRAM0_DMA_SRAM_SPLITADDR_W<14> {
        CORE_X_IRAM0_DRAM0_DMA_SRAM_SPLITADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_x_iram0_dram0_dma_split_line_constrain_1](index.html) module"]
pub struct CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_SPEC;
impl crate::RegisterSpec for CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_x_iram0_dram0_dma_split_line_constrain_1::R](R) reader structure"]
impl crate::Readable for CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_x_iram0_dram0_dma_split_line_constrain_1::W](W) writer structure"]
impl crate::Writable for CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1 to value 0"]
impl crate::Resettable for CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
