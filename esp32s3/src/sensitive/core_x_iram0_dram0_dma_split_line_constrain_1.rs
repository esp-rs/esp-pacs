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
#[doc = "Field `CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_0` reader - category0 of core_x_iram0_dram_dma_line, if the splitaddress in block0 of SRAM, configured as 0x10, else if the splitaddress below block0 of SRAM, configured as 0x11, else if splitaddress higher than block0 of SRAM, configured as 0x00"]
pub type CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_0_R = crate::FieldReader;
#[doc = "Field `CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_0` writer - category0 of core_x_iram0_dram_dma_line, if the splitaddress in block0 of SRAM, configured as 0x10, else if the splitaddress below block0 of SRAM, configured as 0x11, else if splitaddress higher than block0 of SRAM, configured as 0x00"]
pub type CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_1` reader - category1 of core_x_iram0_dram_dma_line, if the splitaddress in block1 of SRAM, configured as 0x10, else if the splitaddress below block1 of SRAM, configured as 0x11, else if splitaddress higher than block1 of SRAM, configured as 0x00"]
pub type CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_1_R = crate::FieldReader;
#[doc = "Field `CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_1` writer - category1 of core_x_iram0_dram_dma_line, if the splitaddress in block1 of SRAM, configured as 0x10, else if the splitaddress below block1 of SRAM, configured as 0x11, else if splitaddress higher than block1 of SRAM, configured as 0x00"]
pub type CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_2` reader - category2 of core_x_iram0_dram_dma_line, if the splitaddress in block2 of SRAM, configured as 0x10, else if the splitaddress below block2 of SRAM, configured as 0x11, else if splitaddress higher than block2 of SRAM, configured as 0x00"]
pub type CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_2_R = crate::FieldReader;
#[doc = "Field `CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_2` writer - category2 of core_x_iram0_dram_dma_line, if the splitaddress in block2 of SRAM, configured as 0x10, else if the splitaddress below block2 of SRAM, configured as 0x11, else if splitaddress higher than block2 of SRAM, configured as 0x00"]
pub type CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_3` reader - category3 of core_x_iram0_dram_dma_line, if the splitaddress in block3 of SRAM, configured as 0x10, else if the splitaddress below block3 of SRAM, configured as 0x11, else if splitaddress higher than block3 of SRAM, configured as 0x00"]
pub type CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_3_R = crate::FieldReader;
#[doc = "Field `CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_3` writer - category3 of core_x_iram0_dram_dma_line, if the splitaddress in block3 of SRAM, configured as 0x10, else if the splitaddress below block3 of SRAM, configured as 0x11, else if splitaddress higher than block3 of SRAM, configured as 0x00"]
pub type CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_3_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_4` reader - category4 of core_x_iram0_dram_dma_line, if the splitaddress in block4 of SRAM, configured as 0x10, else if the splitaddress below block4 of SRAM, configured as 0x11, else if splitaddress higher than block4 of SRAM, configured as 0x00"]
pub type CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_4_R = crate::FieldReader;
#[doc = "Field `CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_4` writer - category4 of core_x_iram0_dram_dma_line, if the splitaddress in block4 of SRAM, configured as 0x10, else if the splitaddress below block4 of SRAM, configured as 0x11, else if splitaddress higher than block4 of SRAM, configured as 0x00"]
pub type CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_4_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_5` reader - category5 of core_x_iram0_dram_dma_line, if the splitaddress in block5 of SRAM, configured as 0x10, else if the splitaddress below block5 of SRAM, configured as 0x11, else if splitaddress higher than block5 of SRAM, configured as 0x00"]
pub type CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_5_R = crate::FieldReader;
#[doc = "Field `CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_5` writer - category5 of core_x_iram0_dram_dma_line, if the splitaddress in block5 of SRAM, configured as 0x10, else if the splitaddress below block5 of SRAM, configured as 0x11, else if splitaddress higher than block5 of SRAM, configured as 0x00"]
pub type CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_5_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_6` reader - category6 of core_x_iram0_dram_dma_line, if the splitaddress in block6 of SRAM, configured as 0x10, else if the splitaddress below block6 of SRAM, configured as 0x11, else if splitaddress higher than block6 of SRAM, configured as 0x00"]
pub type CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_6_R = crate::FieldReader;
#[doc = "Field `CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_6` writer - category6 of core_x_iram0_dram_dma_line, if the splitaddress in block6 of SRAM, configured as 0x10, else if the splitaddress below block6 of SRAM, configured as 0x11, else if splitaddress higher than block6 of SRAM, configured as 0x00"]
pub type CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_6_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_X_IRAM0_DRAM0_DMA_SRAM_SPLITADDR` reader - splitaddr of core_x_iram0_dram_dma_line, configured as \\[15:8\\]bit of actual address"]
pub type CORE_X_IRAM0_DRAM0_DMA_SRAM_SPLITADDR_R = crate::FieldReader;
#[doc = "Field `CORE_X_IRAM0_DRAM0_DMA_SRAM_SPLITADDR` writer - splitaddr of core_x_iram0_dram_dma_line, configured as \\[15:8\\]bit of actual address"]
pub type CORE_X_IRAM0_DRAM0_DMA_SRAM_SPLITADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:1 - category0 of core_x_iram0_dram_dma_line, if the splitaddress in block0 of SRAM, configured as 0x10, else if the splitaddress below block0 of SRAM, configured as 0x11, else if splitaddress higher than block0 of SRAM, configured as 0x00"]
    #[inline(always)]
    pub fn core_x_iram0_dram0_dma_sram_category_0(
        &self,
    ) -> CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_0_R {
        CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - category1 of core_x_iram0_dram_dma_line, if the splitaddress in block1 of SRAM, configured as 0x10, else if the splitaddress below block1 of SRAM, configured as 0x11, else if splitaddress higher than block1 of SRAM, configured as 0x00"]
    #[inline(always)]
    pub fn core_x_iram0_dram0_dma_sram_category_1(
        &self,
    ) -> CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_1_R {
        CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - category2 of core_x_iram0_dram_dma_line, if the splitaddress in block2 of SRAM, configured as 0x10, else if the splitaddress below block2 of SRAM, configured as 0x11, else if splitaddress higher than block2 of SRAM, configured as 0x00"]
    #[inline(always)]
    pub fn core_x_iram0_dram0_dma_sram_category_2(
        &self,
    ) -> CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_2_R {
        CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - category3 of core_x_iram0_dram_dma_line, if the splitaddress in block3 of SRAM, configured as 0x10, else if the splitaddress below block3 of SRAM, configured as 0x11, else if splitaddress higher than block3 of SRAM, configured as 0x00"]
    #[inline(always)]
    pub fn core_x_iram0_dram0_dma_sram_category_3(
        &self,
    ) -> CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_3_R {
        CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - category4 of core_x_iram0_dram_dma_line, if the splitaddress in block4 of SRAM, configured as 0x10, else if the splitaddress below block4 of SRAM, configured as 0x11, else if splitaddress higher than block4 of SRAM, configured as 0x00"]
    #[inline(always)]
    pub fn core_x_iram0_dram0_dma_sram_category_4(
        &self,
    ) -> CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_4_R {
        CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - category5 of core_x_iram0_dram_dma_line, if the splitaddress in block5 of SRAM, configured as 0x10, else if the splitaddress below block5 of SRAM, configured as 0x11, else if splitaddress higher than block5 of SRAM, configured as 0x00"]
    #[inline(always)]
    pub fn core_x_iram0_dram0_dma_sram_category_5(
        &self,
    ) -> CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_5_R {
        CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - category6 of core_x_iram0_dram_dma_line, if the splitaddress in block6 of SRAM, configured as 0x10, else if the splitaddress below block6 of SRAM, configured as 0x11, else if splitaddress higher than block6 of SRAM, configured as 0x00"]
    #[inline(always)]
    pub fn core_x_iram0_dram0_dma_sram_category_6(
        &self,
    ) -> CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_6_R {
        CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:21 - splitaddr of core_x_iram0_dram_dma_line, configured as \\[15:8\\]bit of actual address"]
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
                "core_x_iram0_dram0_dma_sram_category_3",
                &format_args!("{}", self.core_x_iram0_dram0_dma_sram_category_3().bits()),
            )
            .field(
                "core_x_iram0_dram0_dma_sram_category_4",
                &format_args!("{}", self.core_x_iram0_dram0_dma_sram_category_4().bits()),
            )
            .field(
                "core_x_iram0_dram0_dma_sram_category_5",
                &format_args!("{}", self.core_x_iram0_dram0_dma_sram_category_5().bits()),
            )
            .field(
                "core_x_iram0_dram0_dma_sram_category_6",
                &format_args!("{}", self.core_x_iram0_dram0_dma_sram_category_6().bits()),
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
    #[doc = "Bits 0:1 - category0 of core_x_iram0_dram_dma_line, if the splitaddress in block0 of SRAM, configured as 0x10, else if the splitaddress below block0 of SRAM, configured as 0x11, else if splitaddress higher than block0 of SRAM, configured as 0x00"]
    #[inline(always)]
    #[must_use]
    pub fn core_x_iram0_dram0_dma_sram_category_0(
        &mut self,
    ) -> CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_0_W<0> {
        CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_0_W::new(self)
    }
    #[doc = "Bits 2:3 - category1 of core_x_iram0_dram_dma_line, if the splitaddress in block1 of SRAM, configured as 0x10, else if the splitaddress below block1 of SRAM, configured as 0x11, else if splitaddress higher than block1 of SRAM, configured as 0x00"]
    #[inline(always)]
    #[must_use]
    pub fn core_x_iram0_dram0_dma_sram_category_1(
        &mut self,
    ) -> CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_1_W<2> {
        CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_1_W::new(self)
    }
    #[doc = "Bits 4:5 - category2 of core_x_iram0_dram_dma_line, if the splitaddress in block2 of SRAM, configured as 0x10, else if the splitaddress below block2 of SRAM, configured as 0x11, else if splitaddress higher than block2 of SRAM, configured as 0x00"]
    #[inline(always)]
    #[must_use]
    pub fn core_x_iram0_dram0_dma_sram_category_2(
        &mut self,
    ) -> CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_2_W<4> {
        CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_2_W::new(self)
    }
    #[doc = "Bits 6:7 - category3 of core_x_iram0_dram_dma_line, if the splitaddress in block3 of SRAM, configured as 0x10, else if the splitaddress below block3 of SRAM, configured as 0x11, else if splitaddress higher than block3 of SRAM, configured as 0x00"]
    #[inline(always)]
    #[must_use]
    pub fn core_x_iram0_dram0_dma_sram_category_3(
        &mut self,
    ) -> CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_3_W<6> {
        CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_3_W::new(self)
    }
    #[doc = "Bits 8:9 - category4 of core_x_iram0_dram_dma_line, if the splitaddress in block4 of SRAM, configured as 0x10, else if the splitaddress below block4 of SRAM, configured as 0x11, else if splitaddress higher than block4 of SRAM, configured as 0x00"]
    #[inline(always)]
    #[must_use]
    pub fn core_x_iram0_dram0_dma_sram_category_4(
        &mut self,
    ) -> CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_4_W<8> {
        CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_4_W::new(self)
    }
    #[doc = "Bits 10:11 - category5 of core_x_iram0_dram_dma_line, if the splitaddress in block5 of SRAM, configured as 0x10, else if the splitaddress below block5 of SRAM, configured as 0x11, else if splitaddress higher than block5 of SRAM, configured as 0x00"]
    #[inline(always)]
    #[must_use]
    pub fn core_x_iram0_dram0_dma_sram_category_5(
        &mut self,
    ) -> CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_5_W<10> {
        CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_5_W::new(self)
    }
    #[doc = "Bits 12:13 - category6 of core_x_iram0_dram_dma_line, if the splitaddress in block6 of SRAM, configured as 0x10, else if the splitaddress below block6 of SRAM, configured as 0x11, else if splitaddress higher than block6 of SRAM, configured as 0x00"]
    #[inline(always)]
    #[must_use]
    pub fn core_x_iram0_dram0_dma_sram_category_6(
        &mut self,
    ) -> CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_6_W<12> {
        CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_6_W::new(self)
    }
    #[doc = "Bits 14:21 - splitaddr of core_x_iram0_dram_dma_line, configured as \\[15:8\\]bit of actual address"]
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
#[doc = "sram split line configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_x_iram0_dram0_dma_split_line_constrain_1](index.html) module"]
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
