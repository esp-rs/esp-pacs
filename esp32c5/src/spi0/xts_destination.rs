#[doc = "Register `XTS_DESTINATION` reader"]
pub type R = crate::R<XTS_DESTINATION_SPEC>;
#[doc = "Register `XTS_DESTINATION` writer"]
pub type W = crate::W<XTS_DESTINATION_SPEC>;
#[doc = "Field `XTS_DESTINATION` reader - This bit stores the destination parameter which will be used in manual encryption calculation. 0: flash(default), 1: psram(reserved). Only default value can be used."]
pub type XTS_DESTINATION_R = crate::BitReader;
#[doc = "Field `XTS_DESTINATION` writer - This bit stores the destination parameter which will be used in manual encryption calculation. 0: flash(default), 1: psram(reserved). Only default value can be used."]
pub type XTS_DESTINATION_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit stores the destination parameter which will be used in manual encryption calculation. 0: flash(default), 1: psram(reserved). Only default value can be used."]
    #[inline(always)]
    pub fn xts_destination(&self) -> XTS_DESTINATION_R {
        XTS_DESTINATION_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XTS_DESTINATION")
            .field("xts_destination", &self.xts_destination())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - This bit stores the destination parameter which will be used in manual encryption calculation. 0: flash(default), 1: psram(reserved). Only default value can be used."]
    #[inline(always)]
    pub fn xts_destination(&mut self) -> XTS_DESTINATION_W<XTS_DESTINATION_SPEC> {
        XTS_DESTINATION_W::new(self, 0)
    }
}
#[doc = "Manual Encryption destination register\n\nYou can [`read`](crate::Reg::read) this register and get [`xts_destination::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_destination::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XTS_DESTINATION_SPEC;
impl crate::RegisterSpec for XTS_DESTINATION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xts_destination::R`](R) reader structure"]
impl crate::Readable for XTS_DESTINATION_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xts_destination::W`](W) writer structure"]
impl crate::Writable for XTS_DESTINATION_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XTS_DESTINATION to value 0"]
impl crate::Resettable for XTS_DESTINATION_SPEC {
    const RESET_VALUE: u32 = 0;
}
