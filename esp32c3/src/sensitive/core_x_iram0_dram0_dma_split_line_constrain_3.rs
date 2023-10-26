#[doc = "Register `CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3` reader"]
pub type R = crate::R<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3_SPEC>;
#[doc = "Register `CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3` writer"]
pub type W = crate::W<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3_SPEC>;
#[doc = "Field `CORE_X_IRAM0_SRAM_LINE_1_CATEGORY_0` reader - core_x_iram0_sram_line_1_category_0"]
pub type CORE_X_IRAM0_SRAM_LINE_1_CATEGORY_0_R = crate::FieldReader;
#[doc = "Field `CORE_X_IRAM0_SRAM_LINE_1_CATEGORY_0` writer - core_x_iram0_sram_line_1_category_0"]
pub type CORE_X_IRAM0_SRAM_LINE_1_CATEGORY_0_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CORE_X_IRAM0_SRAM_LINE_1_CATEGORY_1` reader - core_x_iram0_sram_line_1_category_1"]
pub type CORE_X_IRAM0_SRAM_LINE_1_CATEGORY_1_R = crate::FieldReader;
#[doc = "Field `CORE_X_IRAM0_SRAM_LINE_1_CATEGORY_1` writer - core_x_iram0_sram_line_1_category_1"]
pub type CORE_X_IRAM0_SRAM_LINE_1_CATEGORY_1_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CORE_X_IRAM0_SRAM_LINE_1_CATEGORY_2` reader - core_x_iram0_sram_line_1_category_2"]
pub type CORE_X_IRAM0_SRAM_LINE_1_CATEGORY_2_R = crate::FieldReader;
#[doc = "Field `CORE_X_IRAM0_SRAM_LINE_1_CATEGORY_2` writer - core_x_iram0_sram_line_1_category_2"]
pub type CORE_X_IRAM0_SRAM_LINE_1_CATEGORY_2_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CORE_X_IRAM0_SRAM_LINE_1_SPLITADDR` reader - core_x_iram0_sram_line_1_splitaddr"]
pub type CORE_X_IRAM0_SRAM_LINE_1_SPLITADDR_R = crate::FieldReader;
#[doc = "Field `CORE_X_IRAM0_SRAM_LINE_1_SPLITADDR` writer - core_x_iram0_sram_line_1_splitaddr"]
pub type CORE_X_IRAM0_SRAM_LINE_1_SPLITADDR_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:1 - core_x_iram0_sram_line_1_category_0"]
    #[inline(always)]
    pub fn core_x_iram0_sram_line_1_category_0(&self) -> CORE_X_IRAM0_SRAM_LINE_1_CATEGORY_0_R {
        CORE_X_IRAM0_SRAM_LINE_1_CATEGORY_0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - core_x_iram0_sram_line_1_category_1"]
    #[inline(always)]
    pub fn core_x_iram0_sram_line_1_category_1(&self) -> CORE_X_IRAM0_SRAM_LINE_1_CATEGORY_1_R {
        CORE_X_IRAM0_SRAM_LINE_1_CATEGORY_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - core_x_iram0_sram_line_1_category_2"]
    #[inline(always)]
    pub fn core_x_iram0_sram_line_1_category_2(&self) -> CORE_X_IRAM0_SRAM_LINE_1_CATEGORY_2_R {
        CORE_X_IRAM0_SRAM_LINE_1_CATEGORY_2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 14:21 - core_x_iram0_sram_line_1_splitaddr"]
    #[inline(always)]
    pub fn core_x_iram0_sram_line_1_splitaddr(&self) -> CORE_X_IRAM0_SRAM_LINE_1_SPLITADDR_R {
        CORE_X_IRAM0_SRAM_LINE_1_SPLITADDR_R::new(((self.bits >> 14) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3")
            .field(
                "core_x_iram0_sram_line_1_category_0",
                &format_args!("{}", self.core_x_iram0_sram_line_1_category_0().bits()),
            )
            .field(
                "core_x_iram0_sram_line_1_category_1",
                &format_args!("{}", self.core_x_iram0_sram_line_1_category_1().bits()),
            )
            .field(
                "core_x_iram0_sram_line_1_category_2",
                &format_args!("{}", self.core_x_iram0_sram_line_1_category_2().bits()),
            )
            .field(
                "core_x_iram0_sram_line_1_splitaddr",
                &format_args!("{}", self.core_x_iram0_sram_line_1_splitaddr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - core_x_iram0_sram_line_1_category_0"]
    #[inline(always)]
    #[must_use]
    pub fn core_x_iram0_sram_line_1_category_0(
        &mut self,
    ) -> CORE_X_IRAM0_SRAM_LINE_1_CATEGORY_0_W<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3_SPEC, 0>
    {
        CORE_X_IRAM0_SRAM_LINE_1_CATEGORY_0_W::new(self)
    }
    #[doc = "Bits 2:3 - core_x_iram0_sram_line_1_category_1"]
    #[inline(always)]
    #[must_use]
    pub fn core_x_iram0_sram_line_1_category_1(
        &mut self,
    ) -> CORE_X_IRAM0_SRAM_LINE_1_CATEGORY_1_W<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3_SPEC, 2>
    {
        CORE_X_IRAM0_SRAM_LINE_1_CATEGORY_1_W::new(self)
    }
    #[doc = "Bits 4:5 - core_x_iram0_sram_line_1_category_2"]
    #[inline(always)]
    #[must_use]
    pub fn core_x_iram0_sram_line_1_category_2(
        &mut self,
    ) -> CORE_X_IRAM0_SRAM_LINE_1_CATEGORY_2_W<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3_SPEC, 4>
    {
        CORE_X_IRAM0_SRAM_LINE_1_CATEGORY_2_W::new(self)
    }
    #[doc = "Bits 14:21 - core_x_iram0_sram_line_1_splitaddr"]
    #[inline(always)]
    #[must_use]
    pub fn core_x_iram0_sram_line_1_splitaddr(
        &mut self,
    ) -> CORE_X_IRAM0_SRAM_LINE_1_SPLITADDR_W<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3_SPEC, 14>
    {
        CORE_X_IRAM0_SRAM_LINE_1_SPLITADDR_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_x_iram0_dram0_dma_split_line_constrain_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_x_iram0_dram0_dma_split_line_constrain_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3_SPEC;
impl crate::RegisterSpec for CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_x_iram0_dram0_dma_split_line_constrain_3::R`](R) reader structure"]
impl crate::Readable for CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_x_iram0_dram0_dma_split_line_constrain_3::W`](W) writer structure"]
impl crate::Writable for CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3 to value 0"]
impl crate::Resettable for CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
