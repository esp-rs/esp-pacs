#[doc = "Register `IN_CRC_INIT_DATA` reader"]
pub type R = crate::R<IN_CRC_INIT_DATA_SPEC>;
#[doc = "Register `IN_CRC_INIT_DATA` writer"]
pub type W = crate::W<IN_CRC_INIT_DATA_SPEC>;
#[doc = "Field `IN_CRC_INIT_DATA` reader - This register is used to config ch0 of rx crc initial value"]
pub type IN_CRC_INIT_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `IN_CRC_INIT_DATA` writer - This register is used to config ch0 of rx crc initial value"]
pub type IN_CRC_INIT_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register is used to config ch0 of rx crc initial value"]
    #[inline(always)]
    pub fn in_crc_init_data(&self) -> IN_CRC_INIT_DATA_R {
        IN_CRC_INIT_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_CRC_INIT_DATA")
            .field("in_crc_init_data", &self.in_crc_init_data().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_CRC_INIT_DATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register is used to config ch0 of rx crc initial value"]
    #[inline(always)]
    #[must_use]
    pub fn in_crc_init_data(&mut self) -> IN_CRC_INIT_DATA_W<IN_CRC_INIT_DATA_SPEC> {
        IN_CRC_INIT_DATA_W::new(self, 0)
    }
}
#[doc = "This register is used to config ch0 crc initial data(max 32 bit)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_crc_init_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_crc_init_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_CRC_INIT_DATA_SPEC;
impl crate::RegisterSpec for IN_CRC_INIT_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_crc_init_data::R`](R) reader structure"]
impl crate::Readable for IN_CRC_INIT_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_crc_init_data::W`](W) writer structure"]
impl crate::Writable for IN_CRC_INIT_DATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IN_CRC_INIT_DATA to value 0xffff_ffff"]
impl crate::Resettable for IN_CRC_INIT_DATA_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
