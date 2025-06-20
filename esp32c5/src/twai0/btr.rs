#[doc = "Register `BTR` reader"]
pub type R = crate::R<BTR_SPEC>;
#[doc = "Register `BTR` writer"]
pub type W = crate::W<BTR_SPEC>;
#[doc = "Field `PROP` reader - Configures the propagation segment of nominal bit rate. Measurement unit: time quanta"]
pub type PROP_R = crate::FieldReader;
#[doc = "Field `PROP` writer - Configures the propagation segment of nominal bit rate. Measurement unit: time quanta"]
pub type PROP_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PH1` reader - Configures the phase 1 segment of nominal bit rate. Measurement unit: time quanta"]
pub type PH1_R = crate::FieldReader;
#[doc = "Field `PH1` writer - Configures the phase 1 segment of nominal bit rate. Measurement unit: time quanta"]
pub type PH1_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PH2` reader - Configures the phase 2 segment of nominal bit rate. Measurement unit: time quanta"]
pub type PH2_R = crate::FieldReader;
#[doc = "Field `PH2` writer - Configures the phase 2 segment of nominal bit rate. Measurement unit: time quanta"]
pub type PH2_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `BRP` reader - Configures the baud-rate prescaler of nominal bit rate. Measurement unit: cycle of core clock."]
pub type BRP_R = crate::FieldReader;
#[doc = "Field `BRP` writer - Configures the baud-rate prescaler of nominal bit rate. Measurement unit: cycle of core clock."]
pub type BRP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SJW` reader - Represents the synchronization jump width in nominal bit time. Measurement unit: time quanta"]
pub type SJW_R = crate::FieldReader;
#[doc = "Field `SJW` writer - Represents the synchronization jump width in nominal bit time. Measurement unit: time quanta"]
pub type SJW_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:6 - Configures the propagation segment of nominal bit rate. Measurement unit: time quanta"]
    #[inline(always)]
    pub fn prop(&self) -> PROP_R {
        PROP_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:12 - Configures the phase 1 segment of nominal bit rate. Measurement unit: time quanta"]
    #[inline(always)]
    pub fn ph1(&self) -> PH1_R {
        PH1_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bits 13:18 - Configures the phase 2 segment of nominal bit rate. Measurement unit: time quanta"]
    #[inline(always)]
    pub fn ph2(&self) -> PH2_R {
        PH2_R::new(((self.bits >> 13) & 0x3f) as u8)
    }
    #[doc = "Bits 19:26 - Configures the baud-rate prescaler of nominal bit rate. Measurement unit: cycle of core clock."]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new(((self.bits >> 19) & 0xff) as u8)
    }
    #[doc = "Bits 27:31 - Represents the synchronization jump width in nominal bit time. Measurement unit: time quanta"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BTR")
            .field("prop", &self.prop())
            .field("ph1", &self.ph1())
            .field("ph2", &self.ph2())
            .field("brp", &self.brp())
            .field("sjw", &self.sjw())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Configures the propagation segment of nominal bit rate. Measurement unit: time quanta"]
    #[inline(always)]
    pub fn prop(&mut self) -> PROP_W<BTR_SPEC> {
        PROP_W::new(self, 0)
    }
    #[doc = "Bits 7:12 - Configures the phase 1 segment of nominal bit rate. Measurement unit: time quanta"]
    #[inline(always)]
    pub fn ph1(&mut self) -> PH1_W<BTR_SPEC> {
        PH1_W::new(self, 7)
    }
    #[doc = "Bits 13:18 - Configures the phase 2 segment of nominal bit rate. Measurement unit: time quanta"]
    #[inline(always)]
    pub fn ph2(&mut self) -> PH2_W<BTR_SPEC> {
        PH2_W::new(self, 13)
    }
    #[doc = "Bits 19:26 - Configures the baud-rate prescaler of nominal bit rate. Measurement unit: cycle of core clock."]
    #[inline(always)]
    pub fn brp(&mut self) -> BRP_W<BTR_SPEC> {
        BRP_W::new(self, 19)
    }
    #[doc = "Bits 27:31 - Represents the synchronization jump width in nominal bit time. Measurement unit: time quanta"]
    #[inline(always)]
    pub fn sjw(&mut self) -> SJW_W<BTR_SPEC> {
        SJW_W::new(self, 27)
    }
}
#[doc = "TWAI FD bit-timing register\n\nYou can [`read`](crate::Reg::read) this register and get [`btr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BTR_SPEC;
impl crate::RegisterSpec for BTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`btr::R`](R) reader structure"]
impl crate::Readable for BTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`btr::W`](W) writer structure"]
impl crate::Writable for BTR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BTR to value 0x1050_a185"]
impl crate::Resettable for BTR_SPEC {
    const RESET_VALUE: u32 = 0x1050_a185;
}
