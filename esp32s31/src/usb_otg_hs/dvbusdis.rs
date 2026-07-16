#[doc = "Register `DVBUSDIS` reader"]
pub type R = crate::R<DVBUSDIS_SPEC>;
#[doc = "Register `DVBUSDIS` writer"]
pub type W = crate::W<DVBUSDIS_SPEC>;
#[doc = "Field `DVBUSDIS` reader - Device VBUS Discharge Time (DVBUSDis) Specifies the VBUS discharge time after VBUS pulsing during SRP. This value equals (VBUS discharge time in PHY clocks) / 1, 024. The value you use depends whether the PHY is operating at 30MHz (16-bit data width) or 60 MHz (8-bit data width). Depending on your VBUS load, this value can need adjustment."]
pub type DVBUSDIS_R = crate::FieldReader<u16>;
#[doc = "Field `DVBUSDIS` writer - Device VBUS Discharge Time (DVBUSDis) Specifies the VBUS discharge time after VBUS pulsing during SRP. This value equals (VBUS discharge time in PHY clocks) / 1, 024. The value you use depends whether the PHY is operating at 30MHz (16-bit data width) or 60 MHz (8-bit data width). Depending on your VBUS load, this value can need adjustment."]
pub type DVBUSDIS_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Device VBUS Discharge Time (DVBUSDis) Specifies the VBUS discharge time after VBUS pulsing during SRP. This value equals (VBUS discharge time in PHY clocks) / 1, 024. The value you use depends whether the PHY is operating at 30MHz (16-bit data width) or 60 MHz (8-bit data width). Depending on your VBUS load, this value can need adjustment."]
    #[inline(always)]
    pub fn dvbusdis(&self) -> DVBUSDIS_R {
        DVBUSDIS_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DVBUSDIS")
            .field("dvbusdis", &self.dvbusdis())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Device VBUS Discharge Time (DVBUSDis) Specifies the VBUS discharge time after VBUS pulsing during SRP. This value equals (VBUS discharge time in PHY clocks) / 1, 024. The value you use depends whether the PHY is operating at 30MHz (16-bit data width) or 60 MHz (8-bit data width). Depending on your VBUS load, this value can need adjustment."]
    #[inline(always)]
    pub fn dvbusdis(&mut self) -> DVBUSDIS_W<'_, DVBUSDIS_SPEC> {
        DVBUSDIS_W::new(self, 0)
    }
}
#[doc = "This register specifies the VBUS discharge time after VBUS pulsing during SRP.\n\nYou can [`read`](crate::Reg::read) this register and get [`dvbusdis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvbusdis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DVBUSDIS_SPEC;
impl crate::RegisterSpec for DVBUSDIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dvbusdis::R`](R) reader structure"]
impl crate::Readable for DVBUSDIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dvbusdis::W`](W) writer structure"]
impl crate::Writable for DVBUSDIS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DVBUSDIS to value 0x17d7"]
impl crate::Resettable for DVBUSDIS_SPEC {
    const RESET_VALUE: u32 = 0x17d7;
}
