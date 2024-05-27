#[doc = "Register `PAD_COMP_CONFIG` reader"]
pub type R = crate::R<PAD_COMP_CONFIG_SPEC>;
#[doc = "Register `PAD_COMP_CONFIG` writer"]
pub type W = crate::W<PAD_COMP_CONFIG_SPEC>;
#[doc = "Field `XPD_COMP` reader - Pad compare enable bit."]
pub type XPD_COMP_R = crate::BitReader;
#[doc = "Field `XPD_COMP` writer - Pad compare enable bit."]
pub type XPD_COMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE_COMP` reader - 1 to enable external reference from PAD\\[0\\]. 0 to enable internal reference, meanwhile PAD\\[0\\] can be used as a regular GPIO."]
pub type MODE_COMP_R = crate::BitReader;
#[doc = "Field `MODE_COMP` writer - 1 to enable external reference from PAD\\[0\\]. 0 to enable internal reference, meanwhile PAD\\[0\\] can be used as a regular GPIO."]
pub type MODE_COMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DREF_COMP` reader - internal reference voltage tuning bit. 0V to 0.7*VDDPST step 0.1*VDDPST."]
pub type DREF_COMP_R = crate::FieldReader;
#[doc = "Field `DREF_COMP` writer - internal reference voltage tuning bit. 0V to 0.7*VDDPST step 0.1*VDDPST."]
pub type DREF_COMP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ZERO_DET_MODE` reader - Zero Detect mode select."]
pub type ZERO_DET_MODE_R = crate::FieldReader;
#[doc = "Field `ZERO_DET_MODE` writer - Zero Detect mode select."]
pub type ZERO_DET_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Pad compare enable bit."]
    #[inline(always)]
    pub fn xpd_comp(&self) -> XPD_COMP_R {
        XPD_COMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1 to enable external reference from PAD\\[0\\]. 0 to enable internal reference, meanwhile PAD\\[0\\] can be used as a regular GPIO."]
    #[inline(always)]
    pub fn mode_comp(&self) -> MODE_COMP_R {
        MODE_COMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - internal reference voltage tuning bit. 0V to 0.7*VDDPST step 0.1*VDDPST."]
    #[inline(always)]
    pub fn dref_comp(&self) -> DREF_COMP_R {
        DREF_COMP_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:6 - Zero Detect mode select."]
    #[inline(always)]
    pub fn zero_det_mode(&self) -> ZERO_DET_MODE_R {
        ZERO_DET_MODE_R::new(((self.bits >> 5) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_COMP_CONFIG")
            .field("xpd_comp", &self.xpd_comp())
            .field("mode_comp", &self.mode_comp())
            .field("dref_comp", &self.dref_comp())
            .field("zero_det_mode", &self.zero_det_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Pad compare enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn xpd_comp(&mut self) -> XPD_COMP_W<PAD_COMP_CONFIG_SPEC> {
        XPD_COMP_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1 to enable external reference from PAD\\[0\\]. 0 to enable internal reference, meanwhile PAD\\[0\\] can be used as a regular GPIO."]
    #[inline(always)]
    #[must_use]
    pub fn mode_comp(&mut self) -> MODE_COMP_W<PAD_COMP_CONFIG_SPEC> {
        MODE_COMP_W::new(self, 1)
    }
    #[doc = "Bits 2:4 - internal reference voltage tuning bit. 0V to 0.7*VDDPST step 0.1*VDDPST."]
    #[inline(always)]
    #[must_use]
    pub fn dref_comp(&mut self) -> DREF_COMP_W<PAD_COMP_CONFIG_SPEC> {
        DREF_COMP_W::new(self, 2)
    }
    #[doc = "Bits 5:6 - Zero Detect mode select."]
    #[inline(always)]
    #[must_use]
    pub fn zero_det_mode(&mut self) -> ZERO_DET_MODE_W<PAD_COMP_CONFIG_SPEC> {
        ZERO_DET_MODE_W::new(self, 5)
    }
}
#[doc = "PAD Compare configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_comp_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_comp_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD_COMP_CONFIG_SPEC;
impl crate::RegisterSpec for PAD_COMP_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_comp_config::R`](R) reader structure"]
impl crate::Readable for PAD_COMP_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad_comp_config::W`](W) writer structure"]
impl crate::Writable for PAD_COMP_CONFIG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PAD_COMP_CONFIG to value 0"]
impl crate::Resettable for PAD_COMP_CONFIG_SPEC {
    const RESET_VALUE: u32 = 0;
}
