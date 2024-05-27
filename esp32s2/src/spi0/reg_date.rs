///Register `REG_DATE` reader
pub type R = crate::R<REG_DATE_SPEC>;
///Register `REG_DATE` writer
pub type W = crate::W<REG_DATE_SPEC>;
///Field `DATE` reader - SPI register version.
pub type DATE_R = crate::FieldReader<u32>;
///Field `DATE` writer - SPI register version.
pub type DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    ///Bits 0:27 - SPI register version.
    #[inline(always)]
    pub fn date(&self) -> DATE_R {
        DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG_DATE")
            .field("date", &self.date())
            .finish()
    }
}
impl W {
    ///Bits 0:27 - SPI register version.
    #[inline(always)]
    #[must_use]
    pub fn date(&mut self) -> DATE_W<REG_DATE_SPEC> {
        DATE_W::new(self, 0)
    }
}
/**SPI version control

You can [`read`](crate::generic::Reg::read) this register and get [`reg_date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct REG_DATE_SPEC;
impl crate::RegisterSpec for REG_DATE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`reg_date::R`](R) reader structure
impl crate::Readable for REG_DATE_SPEC {}
///`write(|w| ..)` method takes [`reg_date::W`](W) writer structure
impl crate::Writable for REG_DATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets REG_DATE to value 0x0190_7240
impl crate::Resettable for REG_DATE_SPEC {
    const RESET_VALUE: u32 = 0x0190_7240;
}
