///Register `XTL_EXT_CTR` reader
pub type R = crate::R<XTL_EXT_CTR_SPEC>;
///Register `XTL_EXT_CTR` writer
pub type W = crate::W<XTL_EXT_CTR_SPEC>;
///Field `SEL` reader - select RTC GPIO 0 ~ 17 to control XTAL
pub type SEL_R = crate::FieldReader;
///Field `SEL` writer - select RTC GPIO 0 ~ 17 to control XTAL
pub type SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 27:31 - select RTC GPIO 0 ~ 17 to control XTAL
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XTL_EXT_CTR")
            .field("sel", &self.sel())
            .finish()
    }
}
impl W {
    ///Bits 27:31 - select RTC GPIO 0 ~ 17 to control XTAL
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<XTL_EXT_CTR_SPEC> {
        SEL_W::new(self, 27)
    }
}
/**configure gpio pd XTAL

You can [`read`](crate::generic::Reg::read) this register and get [`xtl_ext_ctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtl_ext_ctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct XTL_EXT_CTR_SPEC;
impl crate::RegisterSpec for XTL_EXT_CTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`xtl_ext_ctr::R`](R) reader structure
impl crate::Readable for XTL_EXT_CTR_SPEC {}
///`write(|w| ..)` method takes [`xtl_ext_ctr::W`](W) writer structure
impl crate::Writable for XTL_EXT_CTR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets XTL_EXT_CTR to value 0
impl crate::Resettable for XTL_EXT_CTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
