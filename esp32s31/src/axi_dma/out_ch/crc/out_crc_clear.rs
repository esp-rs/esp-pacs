#[doc = "Register `OUT_CRC_CLEAR` reader"]
pub type R = crate::R<OUT_CRC_CLEAR_SPEC>;
#[doc = "Register `OUT_CRC_CLEAR` writer"]
pub type W = crate::W<OUT_CRC_CLEAR_SPEC>;
#[doc = "Field `OUT_CRC_CLEAR` reader - This register is used to clear ch0 of tx crc result"]
pub type OUT_CRC_CLEAR_R = crate::BitReader;
#[doc = "Field `OUT_CRC_CLEAR` writer - This register is used to clear ch0 of tx crc result"]
pub type OUT_CRC_CLEAR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This register is used to clear ch0 of tx crc result"]
    #[inline(always)]
    pub fn out_crc_clear(&self) -> OUT_CRC_CLEAR_R {
        OUT_CRC_CLEAR_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_CRC_CLEAR")
            .field("out_crc_clear", &self.out_crc_clear())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - This register is used to clear ch0 of tx crc result"]
    #[inline(always)]
    pub fn out_crc_clear(&mut self) -> OUT_CRC_CLEAR_W<'_, OUT_CRC_CLEAR_SPEC> {
        OUT_CRC_CLEAR_W::new(self, 0)
    }
}
#[doc = "This register is used to clear ch0 crc result\n\nYou can [`read`](crate::Reg::read) this register and get [`out_crc_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_crc_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_CRC_CLEAR_SPEC;
impl crate::RegisterSpec for OUT_CRC_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_crc_clear::R`](R) reader structure"]
impl crate::Readable for OUT_CRC_CLEAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_crc_clear::W`](W) writer structure"]
impl crate::Writable for OUT_CRC_CLEAR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_CRC_CLEAR to value 0"]
impl crate::Resettable for OUT_CRC_CLEAR_SPEC {}
