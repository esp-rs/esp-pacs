#[doc = "Register `CARRIER_CFG` reader"]
pub type R = crate::R<CARRIER_CFG_SPEC>;
#[doc = "Register `CARRIER_CFG` writer"]
pub type W = crate::W<CARRIER_CFG_SPEC>;
#[doc = "Field `EN` reader - "]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - "]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESCALE` reader - "]
pub type PRESCALE_R = crate::FieldReader;
#[doc = "Field `PRESCALE` writer - "]
pub type PRESCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DUTY` reader - "]
pub type DUTY_R = crate::FieldReader;
#[doc = "Field `DUTY` writer - "]
pub type DUTY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OSHTWTH` reader - "]
pub type OSHTWTH_R = crate::FieldReader;
#[doc = "Field `OSHTWTH` writer - "]
pub type OSHTWTH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OUT_INVERT` reader - "]
pub type OUT_INVERT_R = crate::BitReader;
#[doc = "Field `OUT_INVERT` writer - "]
pub type OUT_INVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_INVERT` reader - "]
pub type IN_INVERT_R = crate::BitReader;
#[doc = "Field `IN_INVERT` writer - "]
pub type IN_INVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4"]
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn duty(&self) -> DUTY_R {
        DUTY_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn oshtwth(&self) -> OSHTWTH_R {
        OSHTWTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn out_invert(&self) -> OUT_INVERT_R {
        OUT_INVERT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn in_invert(&self) -> IN_INVERT_R {
        IN_INVERT_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CARRIER_CFG")
            .field("en", &self.en())
            .field("prescale", &self.prescale())
            .field("duty", &self.duty())
            .field("oshtwth", &self.oshtwth())
            .field("out_invert", &self.out_invert())
            .field("in_invert", &self.in_invert())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CARRIER_CFG_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bits 1:4"]
    #[inline(always)]
    #[must_use]
    pub fn prescale(&mut self) -> PRESCALE_W<CARRIER_CFG_SPEC> {
        PRESCALE_W::new(self, 1)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    #[must_use]
    pub fn duty(&mut self) -> DUTY_W<CARRIER_CFG_SPEC> {
        DUTY_W::new(self, 5)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn oshtwth(&mut self) -> OSHTWTH_W<CARRIER_CFG_SPEC> {
        OSHTWTH_W::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn out_invert(&mut self) -> OUT_INVERT_W<CARRIER_CFG_SPEC> {
        OUT_INVERT_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn in_invert(&mut self) -> IN_INVERT_W<CARRIER_CFG_SPEC> {
        IN_INVERT_W::new(self, 13)
    }
}
#[doc = "Carrier enable and configuratoin\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`carrier_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`carrier_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CARRIER_CFG_SPEC;
impl crate::RegisterSpec for CARRIER_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`carrier_cfg::R`](R) reader structure"]
impl crate::Readable for CARRIER_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`carrier_cfg::W`](W) writer structure"]
impl crate::Writable for CARRIER_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CARRIER_CFG to value 0"]
impl crate::Resettable for CARRIER_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
