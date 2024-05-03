#[doc = "Register `PAD_COMP0` reader"]
pub type R = crate::R<PAD_COMP0_SPEC>;
#[doc = "Register `PAD_COMP0` writer"]
pub type W = crate::W<PAD_COMP0_SPEC>;
#[doc = "Field `DREF_COMP0` reader - pad comp dref"]
pub type DREF_COMP0_R = crate::FieldReader;
#[doc = "Field `DREF_COMP0` writer - pad comp dref"]
pub type DREF_COMP0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MODE_COMP0` reader - pad comp mode"]
pub type MODE_COMP0_R = crate::BitReader;
#[doc = "Field `MODE_COMP0` writer - pad comp mode"]
pub type MODE_COMP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD_COMP0` reader - pad comp xpd"]
pub type XPD_COMP0_R = crate::BitReader;
#[doc = "Field `XPD_COMP0` writer - pad comp xpd"]
pub type XPD_COMP0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - pad comp dref"]
    #[inline(always)]
    pub fn dref_comp0(&self) -> DREF_COMP0_R {
        DREF_COMP0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - pad comp mode"]
    #[inline(always)]
    pub fn mode_comp0(&self) -> MODE_COMP0_R {
        MODE_COMP0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - pad comp xpd"]
    #[inline(always)]
    pub fn xpd_comp0(&self) -> XPD_COMP0_R {
        XPD_COMP0_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_COMP0")
            .field("dref_comp0", &self.dref_comp0().bits())
            .field("mode_comp0", &self.mode_comp0().bit())
            .field("xpd_comp0", &self.xpd_comp0().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PAD_COMP0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2 - pad comp dref"]
    #[inline(always)]
    #[must_use]
    pub fn dref_comp0(&mut self) -> DREF_COMP0_W<PAD_COMP0_SPEC> {
        DREF_COMP0_W::new(self, 0)
    }
    #[doc = "Bit 3 - pad comp mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode_comp0(&mut self) -> MODE_COMP0_W<PAD_COMP0_SPEC> {
        MODE_COMP0_W::new(self, 3)
    }
    #[doc = "Bit 4 - pad comp xpd"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_comp0(&mut self) -> XPD_COMP0_W<PAD_COMP0_SPEC> {
        XPD_COMP0_W::new(self, 4)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_comp0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_comp0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD_COMP0_SPEC;
impl crate::RegisterSpec for PAD_COMP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_comp0::R`](R) reader structure"]
impl crate::Readable for PAD_COMP0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad_comp0::W`](W) writer structure"]
impl crate::Writable for PAD_COMP0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PAD_COMP0 to value 0"]
impl crate::Resettable for PAD_COMP0_SPEC {
    const RESET_VALUE: u32 = 0;
}
