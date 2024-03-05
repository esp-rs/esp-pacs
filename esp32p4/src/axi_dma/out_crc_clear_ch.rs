#[doc = "Register `OUT_CRC_CLEAR_CH%s` reader"]
pub type R = crate::R<OUT_CRC_CLEAR_CH_SPEC>;
#[doc = "Register `OUT_CRC_CLEAR_CH%s` writer"]
pub type W = crate::W<OUT_CRC_CLEAR_CH_SPEC>;
#[doc = "Field `OUT_CRC_CLEAR_CH` reader - This register is used to clear ch0 of tx crc result"]
pub type OUT_CRC_CLEAR_CH_R = crate::BitReader;
#[doc = "Field `OUT_CRC_CLEAR_CH` writer - This register is used to clear ch0 of tx crc result"]
pub type OUT_CRC_CLEAR_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This register is used to clear ch0 of tx crc result"]
    #[inline(always)]
    pub fn out_crc_clear_ch(&self) -> OUT_CRC_CLEAR_CH_R {
        OUT_CRC_CLEAR_CH_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_CRC_CLEAR_CH")
            .field(
                "out_crc_clear_ch",
                &format_args!("{}", self.out_crc_clear_ch().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_CRC_CLEAR_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - This register is used to clear ch0 of tx crc result"]
    #[inline(always)]
    #[must_use]
    pub fn out_crc_clear_ch(&mut self) -> OUT_CRC_CLEAR_CH_W<OUT_CRC_CLEAR_CH_SPEC> {
        OUT_CRC_CLEAR_CH_W::new(self, 0)
    }
}
#[doc = "This register is used to clear ch0 crc result\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_crc_clear_ch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_crc_clear_ch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_CRC_CLEAR_CH_SPEC;
impl crate::RegisterSpec for OUT_CRC_CLEAR_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_crc_clear_ch::R`](R) reader structure"]
impl crate::Readable for OUT_CRC_CLEAR_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_crc_clear_ch::W`](W) writer structure"]
impl crate::Writable for OUT_CRC_CLEAR_CH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_CRC_CLEAR_CH%s to value 0"]
impl crate::Resettable for OUT_CRC_CLEAR_CH_SPEC {
    const RESET_VALUE: u32 = 0;
}
