#[doc = "Register `IN_CRC_CLEAR` reader"]
pub type R = crate::R<IN_CRC_CLEAR_SPEC>;
#[doc = "Register `IN_CRC_CLEAR` writer"]
pub type W = crate::W<IN_CRC_CLEAR_SPEC>;
#[doc = "Field `IN_CRC_CLEAR` reader - This register is used to clear ch0 of rx crc result"]
pub type IN_CRC_CLEAR_R = crate::BitReader;
#[doc = "Field `IN_CRC_CLEAR` writer - This register is used to clear ch0 of rx crc result"]
pub type IN_CRC_CLEAR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This register is used to clear ch0 of rx crc result"]
    #[inline(always)]
    pub fn in_crc_clear(&self) -> IN_CRC_CLEAR_R {
        IN_CRC_CLEAR_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_CRC_CLEAR")
            .field("in_crc_clear", &self.in_crc_clear())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - This register is used to clear ch0 of rx crc result"]
    #[inline(always)]
    #[must_use]
    pub fn in_crc_clear(&mut self) -> IN_CRC_CLEAR_W<IN_CRC_CLEAR_SPEC> {
        IN_CRC_CLEAR_W::new(self, 0)
    }
}
#[doc = "This register is used to clear ch0 crc result\n\nYou can [`read`](crate::Reg::read) this register and get [`in_crc_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_crc_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_CRC_CLEAR_SPEC;
impl crate::RegisterSpec for IN_CRC_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_crc_clear::R`](R) reader structure"]
impl crate::Readable for IN_CRC_CLEAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_crc_clear::W`](W) writer structure"]
impl crate::Writable for IN_CRC_CLEAR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IN_CRC_CLEAR to value 0"]
impl crate::Resettable for IN_CRC_CLEAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
