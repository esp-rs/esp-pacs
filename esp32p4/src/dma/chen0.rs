#[doc = "Register `CHEN0` reader"]
pub type R = crate::R<CHEN0_SPEC>;
#[doc = "Register `CHEN0` writer"]
pub type W = crate::W<CHEN0_SPEC>;
#[doc = "Field `CH1_EN` reader - NA"]
pub type CH1_EN_R = crate::BitReader;
#[doc = "Field `CH1_EN` writer - NA"]
pub type CH1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_EN` reader - NA"]
pub type CH2_EN_R = crate::BitReader;
#[doc = "Field `CH2_EN` writer - NA"]
pub type CH2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_EN` reader - NA"]
pub type CH3_EN_R = crate::BitReader;
#[doc = "Field `CH3_EN` writer - NA"]
pub type CH3_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_EN` reader - NA"]
pub type CH4_EN_R = crate::BitReader;
#[doc = "Field `CH4_EN` writer - NA"]
pub type CH4_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_EN_WE` writer - NA"]
pub type CH1_EN_WE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_EN_WE` writer - NA"]
pub type CH2_EN_WE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_EN_WE` writer - NA"]
pub type CH3_EN_WE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_EN_WE` writer - NA"]
pub type CH4_EN_WE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_SUSP` reader - NA"]
pub type CH1_SUSP_R = crate::BitReader;
#[doc = "Field `CH1_SUSP` writer - NA"]
pub type CH1_SUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_SUSP` reader - NA"]
pub type CH2_SUSP_R = crate::BitReader;
#[doc = "Field `CH2_SUSP` writer - NA"]
pub type CH2_SUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_SUSP` reader - NA"]
pub type CH3_SUSP_R = crate::BitReader;
#[doc = "Field `CH3_SUSP` writer - NA"]
pub type CH3_SUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_SUSP` reader - NA"]
pub type CH4_SUSP_R = crate::BitReader;
#[doc = "Field `CH4_SUSP` writer - NA"]
pub type CH4_SUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_SUSP_WE` writer - NA"]
pub type CH1_SUSP_WE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_SUSP_WE` writer - NA"]
pub type CH2_SUSP_WE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_SUSP_WE` writer - NA"]
pub type CH3_SUSP_WE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_SUSP_WE` writer - NA"]
pub type CH4_SUSP_WE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn ch1_en(&self) -> CH1_EN_R {
        CH1_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn ch2_en(&self) -> CH2_EN_R {
        CH2_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn ch3_en(&self) -> CH3_EN_R {
        CH3_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn ch4_en(&self) -> CH4_EN_R {
        CH4_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn ch1_susp(&self) -> CH1_SUSP_R {
        CH1_SUSP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NA"]
    #[inline(always)]
    pub fn ch2_susp(&self) -> CH2_SUSP_R {
        CH2_SUSP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - NA"]
    #[inline(always)]
    pub fn ch3_susp(&self) -> CH3_SUSP_R {
        CH3_SUSP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - NA"]
    #[inline(always)]
    pub fn ch4_susp(&self) -> CH4_SUSP_R {
        CH4_SUSP_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHEN0")
            .field("ch1_en", &self.ch1_en().bit())
            .field("ch2_en", &self.ch2_en().bit())
            .field("ch3_en", &self.ch3_en().bit())
            .field("ch4_en", &self.ch4_en().bit())
            .field("ch1_susp", &self.ch1_susp().bit())
            .field("ch2_susp", &self.ch2_susp().bit())
            .field("ch3_susp", &self.ch3_susp().bit())
            .field("ch4_susp", &self.ch4_susp().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CHEN0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_en(&mut self) -> CH1_EN_W<CHEN0_SPEC> {
        CH1_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_en(&mut self) -> CH2_EN_W<CHEN0_SPEC> {
        CH2_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_en(&mut self) -> CH3_EN_W<CHEN0_SPEC> {
        CH3_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_en(&mut self) -> CH4_EN_W<CHEN0_SPEC> {
        CH4_EN_W::new(self, 3)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_en_we(&mut self) -> CH1_EN_WE_W<CHEN0_SPEC> {
        CH1_EN_WE_W::new(self, 8)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_en_we(&mut self) -> CH2_EN_WE_W<CHEN0_SPEC> {
        CH2_EN_WE_W::new(self, 9)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_en_we(&mut self) -> CH3_EN_WE_W<CHEN0_SPEC> {
        CH3_EN_WE_W::new(self, 10)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_en_we(&mut self) -> CH4_EN_WE_W<CHEN0_SPEC> {
        CH4_EN_WE_W::new(self, 11)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_susp(&mut self) -> CH1_SUSP_W<CHEN0_SPEC> {
        CH1_SUSP_W::new(self, 16)
    }
    #[doc = "Bit 17 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_susp(&mut self) -> CH2_SUSP_W<CHEN0_SPEC> {
        CH2_SUSP_W::new(self, 17)
    }
    #[doc = "Bit 18 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_susp(&mut self) -> CH3_SUSP_W<CHEN0_SPEC> {
        CH3_SUSP_W::new(self, 18)
    }
    #[doc = "Bit 19 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_susp(&mut self) -> CH4_SUSP_W<CHEN0_SPEC> {
        CH4_SUSP_W::new(self, 19)
    }
    #[doc = "Bit 24 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_susp_we(&mut self) -> CH1_SUSP_WE_W<CHEN0_SPEC> {
        CH1_SUSP_WE_W::new(self, 24)
    }
    #[doc = "Bit 25 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_susp_we(&mut self) -> CH2_SUSP_WE_W<CHEN0_SPEC> {
        CH2_SUSP_WE_W::new(self, 25)
    }
    #[doc = "Bit 26 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_susp_we(&mut self) -> CH3_SUSP_WE_W<CHEN0_SPEC> {
        CH3_SUSP_WE_W::new(self, 26)
    }
    #[doc = "Bit 27 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_susp_we(&mut self) -> CH4_SUSP_WE_W<CHEN0_SPEC> {
        CH4_SUSP_WE_W::new(self, 27)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chen0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chen0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHEN0_SPEC;
impl crate::RegisterSpec for CHEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chen0::R`](R) reader structure"]
impl crate::Readable for CHEN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chen0::W`](W) writer structure"]
impl crate::Writable for CHEN0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHEN0 to value 0"]
impl crate::Resettable for CHEN0_SPEC {
    const RESET_VALUE: u32 = 0;
}
