#[doc = "Register `BTR_FD` reader"]
pub type R = crate::R<BTR_FD_SPEC>;
#[doc = "Register `BTR_FD` writer"]
pub type W = crate::W<BTR_FD_SPEC>;
#[doc = "Field `PROP_FD` reader - Configures the propagation segment of data bit rate. Measurement unit: time quanta"]
pub type PROP_FD_R = crate::FieldReader;
#[doc = "Field `PROP_FD` writer - Configures the propagation segment of data bit rate. Measurement unit: time quanta"]
pub type PROP_FD_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PH1_FD` reader - Configures the phase 1 segment of data bit rate. Measurement unit: time quanta"]
pub type PH1_FD_R = crate::FieldReader;
#[doc = "Field `PH1_FD` writer - Configures the phase 1 segment of data bit rate. Measurement unit: time quanta"]
pub type PH1_FD_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PH2_FD` reader - Configures the phase 2 segment of data bit rate. Measurement unit: time quanta"]
pub type PH2_FD_R = crate::FieldReader;
#[doc = "Field `PH2_FD` writer - Configures the phase 2 segment of data bit rate. Measurement unit: time quanta"]
pub type PH2_FD_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BRP_FD` reader - Configures the baud-rate prescaler of data bit rate. Measurement unit: cycle of core clock."]
pub type BRP_FD_R = crate::FieldReader;
#[doc = "Field `BRP_FD` writer - Configures the baud-rate prescaler of data bit rate. Measurement unit: cycle of core clock."]
pub type BRP_FD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SJW_FD` reader - Represents the synchronization jump width in data bit time. Measurement unit: time quanta"]
pub type SJW_FD_R = crate::FieldReader;
#[doc = "Field `SJW_FD` writer - Represents the synchronization jump width in data bit time. Measurement unit: time quanta"]
pub type SJW_FD_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:5 - Configures the propagation segment of data bit rate. Measurement unit: time quanta"]
    #[inline(always)]
    pub fn prop_fd(&self) -> PROP_FD_R {
        PROP_FD_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 7:11 - Configures the phase 1 segment of data bit rate. Measurement unit: time quanta"]
    #[inline(always)]
    pub fn ph1_fd(&self) -> PH1_FD_R {
        PH1_FD_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 13:17 - Configures the phase 2 segment of data bit rate. Measurement unit: time quanta"]
    #[inline(always)]
    pub fn ph2_fd(&self) -> PH2_FD_R {
        PH2_FD_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bits 19:26 - Configures the baud-rate prescaler of data bit rate. Measurement unit: cycle of core clock."]
    #[inline(always)]
    pub fn brp_fd(&self) -> BRP_FD_R {
        BRP_FD_R::new(((self.bits >> 19) & 0xff) as u8)
    }
    #[doc = "Bits 27:31 - Represents the synchronization jump width in data bit time. Measurement unit: time quanta"]
    #[inline(always)]
    pub fn sjw_fd(&self) -> SJW_FD_R {
        SJW_FD_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BTR_FD")
            .field("prop_fd", &self.prop_fd())
            .field("ph1_fd", &self.ph1_fd())
            .field("ph2_fd", &self.ph2_fd())
            .field("brp_fd", &self.brp_fd())
            .field("sjw_fd", &self.sjw_fd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Configures the propagation segment of data bit rate. Measurement unit: time quanta"]
    #[inline(always)]
    pub fn prop_fd(&mut self) -> PROP_FD_W<BTR_FD_SPEC> {
        PROP_FD_W::new(self, 0)
    }
    #[doc = "Bits 7:11 - Configures the phase 1 segment of data bit rate. Measurement unit: time quanta"]
    #[inline(always)]
    pub fn ph1_fd(&mut self) -> PH1_FD_W<BTR_FD_SPEC> {
        PH1_FD_W::new(self, 7)
    }
    #[doc = "Bits 13:17 - Configures the phase 2 segment of data bit rate. Measurement unit: time quanta"]
    #[inline(always)]
    pub fn ph2_fd(&mut self) -> PH2_FD_W<BTR_FD_SPEC> {
        PH2_FD_W::new(self, 13)
    }
    #[doc = "Bits 19:26 - Configures the baud-rate prescaler of data bit rate. Measurement unit: cycle of core clock."]
    #[inline(always)]
    pub fn brp_fd(&mut self) -> BRP_FD_W<BTR_FD_SPEC> {
        BRP_FD_W::new(self, 19)
    }
    #[doc = "Bits 27:31 - Represents the synchronization jump width in data bit time. Measurement unit: time quanta"]
    #[inline(always)]
    pub fn sjw_fd(&mut self) -> SJW_FD_W<BTR_FD_SPEC> {
        SJW_FD_W::new(self, 27)
    }
}
#[doc = "TWAI FD bit-timing of FD register\n\nYou can [`read`](crate::Reg::read) this register and get [`btr_fd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr_fd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BTR_FD_SPEC;
impl crate::RegisterSpec for BTR_FD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`btr_fd::R`](R) reader structure"]
impl crate::Readable for BTR_FD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`btr_fd::W`](W) writer structure"]
impl crate::Writable for BTR_FD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BTR_FD to value 0x1020_6183"]
impl crate::Resettable for BTR_FD_SPEC {
    const RESET_VALUE: u32 = 0x1020_6183;
}
