#[doc = "Register `HALL_SENS` reader"]
pub type R = crate::R<HALL_SENS_SPEC>;
#[doc = "Register `HALL_SENS` writer"]
pub type W = crate::W<HALL_SENS_SPEC>;
#[doc = "Field `HALL_PHASE` reader - Reverse phase of hall sensor"]
pub type HALL_PHASE_R = crate::BitReader;
#[doc = "Field `HALL_PHASE` writer - Reverse phase of hall sensor"]
pub type HALL_PHASE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD_HALL` reader - Power on hall sensor and connect to VP and VN"]
pub type XPD_HALL_R = crate::BitReader;
#[doc = "Field `XPD_HALL` writer - Power on hall sensor and connect to VP and VN"]
pub type XPD_HALL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - Reverse phase of hall sensor"]
    #[inline(always)]
    pub fn hall_phase(&self) -> HALL_PHASE_R {
        HALL_PHASE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Power on hall sensor and connect to VP and VN"]
    #[inline(always)]
    pub fn xpd_hall(&self) -> XPD_HALL_R {
        XPD_HALL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HALL_SENS")
            .field("hall_phase", &self.hall_phase())
            .field("xpd_hall", &self.xpd_hall())
            .finish()
    }
}
impl W {
    #[doc = "Bit 30 - Reverse phase of hall sensor"]
    #[inline(always)]
    pub fn hall_phase(&mut self) -> HALL_PHASE_W<HALL_SENS_SPEC> {
        HALL_PHASE_W::new(self, 30)
    }
    #[doc = "Bit 31 - Power on hall sensor and connect to VP and VN"]
    #[inline(always)]
    pub fn xpd_hall(&mut self) -> XPD_HALL_W<HALL_SENS_SPEC> {
        XPD_HALL_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`hall_sens::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hall_sens::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HALL_SENS_SPEC;
impl crate::RegisterSpec for HALL_SENS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hall_sens::R`](R) reader structure"]
impl crate::Readable for HALL_SENS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hall_sens::W`](W) writer structure"]
impl crate::Writable for HALL_SENS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HALL_SENS to value 0"]
impl crate::Resettable for HALL_SENS_SPEC {}
