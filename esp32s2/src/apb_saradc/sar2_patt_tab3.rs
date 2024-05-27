///Register `SAR2_PATT_TAB3` reader
pub type R = crate::R<SAR2_PATT_TAB3_SPEC>;
///Register `SAR2_PATT_TAB3` writer
pub type W = crate::W<SAR2_PATT_TAB3_SPEC>;
///Field `SAR2_PATT_TAB3` reader - Item 8 ~ 11 for pattern table 2 (each item one byte)
pub type SAR2_PATT_TAB3_R = crate::FieldReader<u32>;
///Field `SAR2_PATT_TAB3` writer - Item 8 ~ 11 for pattern table 2 (each item one byte)
pub type SAR2_PATT_TAB3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Item 8 ~ 11 for pattern table 2 (each item one byte)
    #[inline(always)]
    pub fn sar2_patt_tab3(&self) -> SAR2_PATT_TAB3_R {
        SAR2_PATT_TAB3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR2_PATT_TAB3")
            .field("sar2_patt_tab3", &self.sar2_patt_tab3())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Item 8 ~ 11 for pattern table 2 (each item one byte)
    #[inline(always)]
    #[must_use]
    pub fn sar2_patt_tab3(&mut self) -> SAR2_PATT_TAB3_W<SAR2_PATT_TAB3_SPEC> {
        SAR2_PATT_TAB3_W::new(self, 0)
    }
}
/**Item 8 ~ 11 for pattern table 2 (each item one byte)

You can [`read`](crate::generic::Reg::read) this register and get [`sar2_patt_tab3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar2_patt_tab3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SAR2_PATT_TAB3_SPEC;
impl crate::RegisterSpec for SAR2_PATT_TAB3_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sar2_patt_tab3::R`](R) reader structure
impl crate::Readable for SAR2_PATT_TAB3_SPEC {}
///`write(|w| ..)` method takes [`sar2_patt_tab3::W`](W) writer structure
impl crate::Writable for SAR2_PATT_TAB3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SAR2_PATT_TAB3 to value 0x0f0f_0f0f
impl crate::Resettable for SAR2_PATT_TAB3_SPEC {
    const RESET_VALUE: u32 = 0x0f0f_0f0f;
}
