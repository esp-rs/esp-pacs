#[doc = "Register `CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4` reader"]
pub type R = crate::R<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC>;
#[doc = "Register `CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4` writer"]
pub type W = crate::W<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC>;
#[doc = "Field `CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_0` reader - category0 of core_x_iram0_dram_dma_line, if the splitaddress in block0 of SRAM, configured as 0x10, else if the splitaddress below block0 of SRAM, configured as 0x11, else if splitaddress higher than block0 of SRAM, configured as 0x00"]
pub type CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_0_R = crate::FieldReader;
#[doc = "Field `CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_0` writer - category0 of core_x_iram0_dram_dma_line, if the splitaddress in block0 of SRAM, configured as 0x10, else if the splitaddress below block0 of SRAM, configured as 0x11, else if splitaddress higher than block0 of SRAM, configured as 0x00"]
pub type CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_1` reader - category1 of core_x_iram0_dram_dma_line, if the splitaddress in block1 of SRAM, configured as 0x10, else if the splitaddress below block1 of SRAM, configured as 0x11, else if splitaddress higher than block1 of SRAM, configured as 0x00"]
pub type CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_1_R = crate::FieldReader;
#[doc = "Field `CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_1` writer - category1 of core_x_iram0_dram_dma_line, if the splitaddress in block1 of SRAM, configured as 0x10, else if the splitaddress below block1 of SRAM, configured as 0x11, else if splitaddress higher than block1 of SRAM, configured as 0x00"]
pub type CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_2` reader - category2 of core_x_iram0_dram_dma_line, if the splitaddress in block2 of SRAM, configured as 0x10, else if the splitaddress below block2 of SRAM, configured as 0x11, else if splitaddress higher than block2 of SRAM, configured as 0x00"]
pub type CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_2_R = crate::FieldReader;
#[doc = "Field `CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_2` writer - category2 of core_x_iram0_dram_dma_line, if the splitaddress in block2 of SRAM, configured as 0x10, else if the splitaddress below block2 of SRAM, configured as 0x11, else if splitaddress higher than block2 of SRAM, configured as 0x00"]
pub type CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_3` reader - category3 of core_x_iram0_dram_dma_line, if the splitaddress in block3 of SRAM, configured as 0x10, else if the splitaddress below block3 of SRAM, configured as 0x11, else if splitaddress higher than block3 of SRAM, configured as 0x00"]
pub type CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_3_R = crate::FieldReader;
#[doc = "Field `CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_3` writer - category3 of core_x_iram0_dram_dma_line, if the splitaddress in block3 of SRAM, configured as 0x10, else if the splitaddress below block3 of SRAM, configured as 0x11, else if splitaddress higher than block3 of SRAM, configured as 0x00"]
pub type CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_4` reader - category4 of core_x_iram0_dram_dma_line, if the splitaddress in block4 of SRAM, configured as 0x10, else if the splitaddress below block4 of SRAM, configured as 0x11, else if splitaddress higher than block4 of SRAM, configured as 0x00"]
pub type CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_4_R = crate::FieldReader;
#[doc = "Field `CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_4` writer - category4 of core_x_iram0_dram_dma_line, if the splitaddress in block4 of SRAM, configured as 0x10, else if the splitaddress below block4 of SRAM, configured as 0x11, else if splitaddress higher than block4 of SRAM, configured as 0x00"]
pub type CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_5` reader - category5 of core_x_iram0_dram_dma_line, if the splitaddress in block5 of SRAM, configured as 0x10, else if the splitaddress below block5 of SRAM, configured as 0x11, else if splitaddress higher than block5 of SRAM, configured as 0x00"]
pub type CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_5_R = crate::FieldReader;
#[doc = "Field `CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_5` writer - category5 of core_x_iram0_dram_dma_line, if the splitaddress in block5 of SRAM, configured as 0x10, else if the splitaddress below block5 of SRAM, configured as 0x11, else if splitaddress higher than block5 of SRAM, configured as 0x00"]
pub type CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_6` reader - category6 of core_x_iram0_dram_dma_line, if the splitaddress in block6 of SRAM, configured as 0x10, else if the splitaddress below block6 of SRAM, configured as 0x11, else if splitaddress higher than block6 of SRAM, configured as 0x00"]
pub type CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_6_R = crate::FieldReader;
#[doc = "Field `CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_6` writer - category6 of core_x_iram0_dram_dma_line, if the splitaddress in block6 of SRAM, configured as 0x10, else if the splitaddress below block6 of SRAM, configured as 0x11, else if splitaddress higher than block6 of SRAM, configured as 0x00"]
pub type CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CORE_X_DRAM0_DMA_SRAM_LINE_0_SPLITADDR` reader - splitaddr of core_x_iram0_dram_dma_line, configured as \\[15:8\\]bit of actual address"]
pub type CORE_X_DRAM0_DMA_SRAM_LINE_0_SPLITADDR_R = crate::FieldReader;
#[doc = "Field `CORE_X_DRAM0_DMA_SRAM_LINE_0_SPLITADDR` writer - splitaddr of core_x_iram0_dram_dma_line, configured as \\[15:8\\]bit of actual address"]
pub type CORE_X_DRAM0_DMA_SRAM_LINE_0_SPLITADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - category0 of core_x_iram0_dram_dma_line, if the splitaddress in block0 of SRAM, configured as 0x10, else if the splitaddress below block0 of SRAM, configured as 0x11, else if splitaddress higher than block0 of SRAM, configured as 0x00"]
    #[inline(always)]
    pub fn core_x_dram0_dma_sram_line_0_category_0(
        &self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_0_R {
        CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - category1 of core_x_iram0_dram_dma_line, if the splitaddress in block1 of SRAM, configured as 0x10, else if the splitaddress below block1 of SRAM, configured as 0x11, else if splitaddress higher than block1 of SRAM, configured as 0x00"]
    #[inline(always)]
    pub fn core_x_dram0_dma_sram_line_0_category_1(
        &self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_1_R {
        CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - category2 of core_x_iram0_dram_dma_line, if the splitaddress in block2 of SRAM, configured as 0x10, else if the splitaddress below block2 of SRAM, configured as 0x11, else if splitaddress higher than block2 of SRAM, configured as 0x00"]
    #[inline(always)]
    pub fn core_x_dram0_dma_sram_line_0_category_2(
        &self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_2_R {
        CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - category3 of core_x_iram0_dram_dma_line, if the splitaddress in block3 of SRAM, configured as 0x10, else if the splitaddress below block3 of SRAM, configured as 0x11, else if splitaddress higher than block3 of SRAM, configured as 0x00"]
    #[inline(always)]
    pub fn core_x_dram0_dma_sram_line_0_category_3(
        &self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_3_R {
        CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - category4 of core_x_iram0_dram_dma_line, if the splitaddress in block4 of SRAM, configured as 0x10, else if the splitaddress below block4 of SRAM, configured as 0x11, else if splitaddress higher than block4 of SRAM, configured as 0x00"]
    #[inline(always)]
    pub fn core_x_dram0_dma_sram_line_0_category_4(
        &self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_4_R {
        CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - category5 of core_x_iram0_dram_dma_line, if the splitaddress in block5 of SRAM, configured as 0x10, else if the splitaddress below block5 of SRAM, configured as 0x11, else if splitaddress higher than block5 of SRAM, configured as 0x00"]
    #[inline(always)]
    pub fn core_x_dram0_dma_sram_line_0_category_5(
        &self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_5_R {
        CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - category6 of core_x_iram0_dram_dma_line, if the splitaddress in block6 of SRAM, configured as 0x10, else if the splitaddress below block6 of SRAM, configured as 0x11, else if splitaddress higher than block6 of SRAM, configured as 0x00"]
    #[inline(always)]
    pub fn core_x_dram0_dma_sram_line_0_category_6(
        &self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_6_R {
        CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:21 - splitaddr of core_x_iram0_dram_dma_line, configured as \\[15:8\\]bit of actual address"]
    #[inline(always)]
    pub fn core_x_dram0_dma_sram_line_0_splitaddr(
        &self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_0_SPLITADDR_R {
        CORE_X_DRAM0_DMA_SRAM_LINE_0_SPLITADDR_R::new(((self.bits >> 14) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4")
            .field(
                "core_x_dram0_dma_sram_line_0_category_0",
                &self.core_x_dram0_dma_sram_line_0_category_0(),
            )
            .field(
                "core_x_dram0_dma_sram_line_0_category_1",
                &self.core_x_dram0_dma_sram_line_0_category_1(),
            )
            .field(
                "core_x_dram0_dma_sram_line_0_category_2",
                &self.core_x_dram0_dma_sram_line_0_category_2(),
            )
            .field(
                "core_x_dram0_dma_sram_line_0_category_3",
                &self.core_x_dram0_dma_sram_line_0_category_3(),
            )
            .field(
                "core_x_dram0_dma_sram_line_0_category_4",
                &self.core_x_dram0_dma_sram_line_0_category_4(),
            )
            .field(
                "core_x_dram0_dma_sram_line_0_category_5",
                &self.core_x_dram0_dma_sram_line_0_category_5(),
            )
            .field(
                "core_x_dram0_dma_sram_line_0_category_6",
                &self.core_x_dram0_dma_sram_line_0_category_6(),
            )
            .field(
                "core_x_dram0_dma_sram_line_0_splitaddr",
                &self.core_x_dram0_dma_sram_line_0_splitaddr(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - category0 of core_x_iram0_dram_dma_line, if the splitaddress in block0 of SRAM, configured as 0x10, else if the splitaddress below block0 of SRAM, configured as 0x11, else if splitaddress higher than block0 of SRAM, configured as 0x00"]
    #[inline(always)]
    pub fn core_x_dram0_dma_sram_line_0_category_0(
        &mut self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_0_W<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC>
    {
        CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - category1 of core_x_iram0_dram_dma_line, if the splitaddress in block1 of SRAM, configured as 0x10, else if the splitaddress below block1 of SRAM, configured as 0x11, else if splitaddress higher than block1 of SRAM, configured as 0x00"]
    #[inline(always)]
    pub fn core_x_dram0_dma_sram_line_0_category_1(
        &mut self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_1_W<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC>
    {
        CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - category2 of core_x_iram0_dram_dma_line, if the splitaddress in block2 of SRAM, configured as 0x10, else if the splitaddress below block2 of SRAM, configured as 0x11, else if splitaddress higher than block2 of SRAM, configured as 0x00"]
    #[inline(always)]
    pub fn core_x_dram0_dma_sram_line_0_category_2(
        &mut self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_2_W<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC>
    {
        CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - category3 of core_x_iram0_dram_dma_line, if the splitaddress in block3 of SRAM, configured as 0x10, else if the splitaddress below block3 of SRAM, configured as 0x11, else if splitaddress higher than block3 of SRAM, configured as 0x00"]
    #[inline(always)]
    pub fn core_x_dram0_dma_sram_line_0_category_3(
        &mut self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_3_W<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC>
    {
        CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_3_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - category4 of core_x_iram0_dram_dma_line, if the splitaddress in block4 of SRAM, configured as 0x10, else if the splitaddress below block4 of SRAM, configured as 0x11, else if splitaddress higher than block4 of SRAM, configured as 0x00"]
    #[inline(always)]
    pub fn core_x_dram0_dma_sram_line_0_category_4(
        &mut self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_4_W<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC>
    {
        CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_4_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - category5 of core_x_iram0_dram_dma_line, if the splitaddress in block5 of SRAM, configured as 0x10, else if the splitaddress below block5 of SRAM, configured as 0x11, else if splitaddress higher than block5 of SRAM, configured as 0x00"]
    #[inline(always)]
    pub fn core_x_dram0_dma_sram_line_0_category_5(
        &mut self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_5_W<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC>
    {
        CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_5_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - category6 of core_x_iram0_dram_dma_line, if the splitaddress in block6 of SRAM, configured as 0x10, else if the splitaddress below block6 of SRAM, configured as 0x11, else if splitaddress higher than block6 of SRAM, configured as 0x00"]
    #[inline(always)]
    pub fn core_x_dram0_dma_sram_line_0_category_6(
        &mut self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_6_W<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC>
    {
        CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_6_W::new(self, 12)
    }
    #[doc = "Bits 14:21 - splitaddr of core_x_iram0_dram_dma_line, configured as \\[15:8\\]bit of actual address"]
    #[inline(always)]
    pub fn core_x_dram0_dma_sram_line_0_splitaddr(
        &mut self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_0_SPLITADDR_W<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC>
    {
        CORE_X_DRAM0_DMA_SRAM_LINE_0_SPLITADDR_W::new(self, 14)
    }
}
#[doc = "sram split line configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`core_x_iram0_dram0_dma_split_line_constrain_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_x_iram0_dram0_dma_split_line_constrain_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC;
impl crate::RegisterSpec for CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_x_iram0_dram0_dma_split_line_constrain_4::R`](R) reader structure"]
impl crate::Readable for CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_x_iram0_dram0_dma_split_line_constrain_4::W`](W) writer structure"]
impl crate::Writable for CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4 to value 0"]
impl crate::Resettable for CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC {}
