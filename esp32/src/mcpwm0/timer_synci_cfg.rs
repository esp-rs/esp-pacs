#[doc = "Register `TIMER_SYNCI_CFG` reader"]
pub type R = crate::R<TIMER_SYNCI_CFG_SPEC>;
#[doc = "Register `TIMER_SYNCI_CFG` writer"]
pub type W = crate::W<TIMER_SYNCI_CFG_SPEC>;
#[doc = "Field `TIMER0_SYNCISEL` reader - "]
pub type TIMER0_SYNCISEL_R = crate::FieldReader;
#[doc = "Field `TIMER0_SYNCISEL` writer - "]
pub type TIMER0_SYNCISEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TIMER1_SYNCISEL` reader - "]
pub type TIMER1_SYNCISEL_R = crate::FieldReader;
#[doc = "Field `TIMER1_SYNCISEL` writer - "]
pub type TIMER1_SYNCISEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TIMER2_SYNCISEL` reader - "]
pub type TIMER2_SYNCISEL_R = crate::FieldReader;
#[doc = "Field `TIMER2_SYNCISEL` writer - "]
pub type TIMER2_SYNCISEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EXTERNAL_SYNCI0_INVERT` reader - "]
pub type EXTERNAL_SYNCI0_INVERT_R = crate::BitReader;
#[doc = "Field `EXTERNAL_SYNCI0_INVERT` writer - "]
pub type EXTERNAL_SYNCI0_INVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTERNAL_SYNCI1_INVERT` reader - "]
pub type EXTERNAL_SYNCI1_INVERT_R = crate::BitReader;
#[doc = "Field `EXTERNAL_SYNCI1_INVERT` writer - "]
pub type EXTERNAL_SYNCI1_INVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTERNAL_SYNCI2_INVERT` reader - "]
pub type EXTERNAL_SYNCI2_INVERT_R = crate::BitReader;
#[doc = "Field `EXTERNAL_SYNCI2_INVERT` writer - "]
pub type EXTERNAL_SYNCI2_INVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn timer0_syncisel(&self) -> TIMER0_SYNCISEL_R {
        TIMER0_SYNCISEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn timer1_syncisel(&self) -> TIMER1_SYNCISEL_R {
        TIMER1_SYNCISEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn timer2_syncisel(&self) -> TIMER2_SYNCISEL_R {
        TIMER2_SYNCISEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn external_synci0_invert(&self) -> EXTERNAL_SYNCI0_INVERT_R {
        EXTERNAL_SYNCI0_INVERT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn external_synci1_invert(&self) -> EXTERNAL_SYNCI1_INVERT_R {
        EXTERNAL_SYNCI1_INVERT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn external_synci2_invert(&self) -> EXTERNAL_SYNCI2_INVERT_R {
        EXTERNAL_SYNCI2_INVERT_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_SYNCI_CFG")
            .field("timer0_syncisel", &self.timer0_syncisel())
            .field("timer1_syncisel", &self.timer1_syncisel())
            .field("timer2_syncisel", &self.timer2_syncisel())
            .field("external_synci0_invert", &self.external_synci0_invert())
            .field("external_synci1_invert", &self.external_synci1_invert())
            .field("external_synci2_invert", &self.external_synci2_invert())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_syncisel(&mut self) -> TIMER0_SYNCISEL_W<TIMER_SYNCI_CFG_SPEC> {
        TIMER0_SYNCISEL_W::new(self, 0)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_syncisel(&mut self) -> TIMER1_SYNCISEL_W<TIMER_SYNCI_CFG_SPEC> {
        TIMER1_SYNCISEL_W::new(self, 3)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_syncisel(&mut self) -> TIMER2_SYNCISEL_W<TIMER_SYNCI_CFG_SPEC> {
        TIMER2_SYNCISEL_W::new(self, 6)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn external_synci0_invert(&mut self) -> EXTERNAL_SYNCI0_INVERT_W<TIMER_SYNCI_CFG_SPEC> {
        EXTERNAL_SYNCI0_INVERT_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn external_synci1_invert(&mut self) -> EXTERNAL_SYNCI1_INVERT_W<TIMER_SYNCI_CFG_SPEC> {
        EXTERNAL_SYNCI1_INVERT_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn external_synci2_invert(&mut self) -> EXTERNAL_SYNCI2_INVERT_W<TIMER_SYNCI_CFG_SPEC> {
        EXTERNAL_SYNCI2_INVERT_W::new(self, 11)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_synci_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_synci_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_SYNCI_CFG_SPEC;
impl crate::RegisterSpec for TIMER_SYNCI_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_synci_cfg::R`](R) reader structure"]
impl crate::Readable for TIMER_SYNCI_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer_synci_cfg::W`](W) writer structure"]
impl crate::Writable for TIMER_SYNCI_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER_SYNCI_CFG to value 0"]
impl crate::Resettable for TIMER_SYNCI_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
