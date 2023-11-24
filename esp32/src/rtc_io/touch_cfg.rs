#[doc = "Register `TOUCH_CFG` reader"]
pub type R = crate::R<TOUCH_CFG_SPEC>;
#[doc = "Register `TOUCH_CFG` writer"]
pub type W = crate::W<TOUCH_CFG_SPEC>;
#[doc = "Field `TOUCH_DCUR` reader - touch sensor bias current. Should have option to tie with BIAS_SLEEP(When BIAS_SLEEP this setting is available"]
pub type TOUCH_DCUR_R = crate::FieldReader;
#[doc = "Field `TOUCH_DCUR` writer - touch sensor bias current. Should have option to tie with BIAS_SLEEP(When BIAS_SLEEP this setting is available"]
pub type TOUCH_DCUR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_DRANGE` reader - touch sensor saw wave voltage range."]
pub type TOUCH_DRANGE_R = crate::FieldReader;
#[doc = "Field `TOUCH_DRANGE` writer - touch sensor saw wave voltage range."]
pub type TOUCH_DRANGE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_DREFL` reader - touch sensor saw wave bottom voltage."]
pub type TOUCH_DREFL_R = crate::FieldReader;
#[doc = "Field `TOUCH_DREFL` writer - touch sensor saw wave bottom voltage."]
pub type TOUCH_DREFL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_DREFH` reader - touch sensor saw wave top voltage."]
pub type TOUCH_DREFH_R = crate::FieldReader;
#[doc = "Field `TOUCH_DREFH` writer - touch sensor saw wave top voltage."]
pub type TOUCH_DREFH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_XPD_BIAS` reader - touch sensor bias power on."]
pub type TOUCH_XPD_BIAS_R = crate::BitReader;
#[doc = "Field `TOUCH_XPD_BIAS` writer - touch sensor bias power on."]
pub type TOUCH_XPD_BIAS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 23:24 - touch sensor bias current. Should have option to tie with BIAS_SLEEP(When BIAS_SLEEP this setting is available"]
    #[inline(always)]
    pub fn touch_dcur(&self) -> TOUCH_DCUR_R {
        TOUCH_DCUR_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:26 - touch sensor saw wave voltage range."]
    #[inline(always)]
    pub fn touch_drange(&self) -> TOUCH_DRANGE_R {
        TOUCH_DRANGE_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:28 - touch sensor saw wave bottom voltage."]
    #[inline(always)]
    pub fn touch_drefl(&self) -> TOUCH_DREFL_R {
        TOUCH_DREFL_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bits 29:30 - touch sensor saw wave top voltage."]
    #[inline(always)]
    pub fn touch_drefh(&self) -> TOUCH_DREFH_R {
        TOUCH_DREFH_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - touch sensor bias power on."]
    #[inline(always)]
    pub fn touch_xpd_bias(&self) -> TOUCH_XPD_BIAS_R {
        TOUCH_XPD_BIAS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_CFG")
            .field("touch_dcur", &format_args!("{}", self.touch_dcur().bits()))
            .field(
                "touch_drange",
                &format_args!("{}", self.touch_drange().bits()),
            )
            .field(
                "touch_drefl",
                &format_args!("{}", self.touch_drefl().bits()),
            )
            .field(
                "touch_drefh",
                &format_args!("{}", self.touch_drefh().bits()),
            )
            .field(
                "touch_xpd_bias",
                &format_args!("{}", self.touch_xpd_bias().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TOUCH_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 23:24 - touch sensor bias current. Should have option to tie with BIAS_SLEEP(When BIAS_SLEEP this setting is available"]
    #[inline(always)]
    #[must_use]
    pub fn touch_dcur(&mut self) -> TOUCH_DCUR_W<TOUCH_CFG_SPEC> {
        TOUCH_DCUR_W::new(self, 23)
    }
    #[doc = "Bits 25:26 - touch sensor saw wave voltage range."]
    #[inline(always)]
    #[must_use]
    pub fn touch_drange(&mut self) -> TOUCH_DRANGE_W<TOUCH_CFG_SPEC> {
        TOUCH_DRANGE_W::new(self, 25)
    }
    #[doc = "Bits 27:28 - touch sensor saw wave bottom voltage."]
    #[inline(always)]
    #[must_use]
    pub fn touch_drefl(&mut self) -> TOUCH_DREFL_W<TOUCH_CFG_SPEC> {
        TOUCH_DREFL_W::new(self, 27)
    }
    #[doc = "Bits 29:30 - touch sensor saw wave top voltage."]
    #[inline(always)]
    #[must_use]
    pub fn touch_drefh(&mut self) -> TOUCH_DREFH_W<TOUCH_CFG_SPEC> {
        TOUCH_DREFH_W::new(self, 29)
    }
    #[doc = "Bit 31 - touch sensor bias power on."]
    #[inline(always)]
    #[must_use]
    pub fn touch_xpd_bias(&mut self) -> TOUCH_XPD_BIAS_W<TOUCH_CFG_SPEC> {
        TOUCH_XPD_BIAS_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_CFG_SPEC;
impl crate::RegisterSpec for TOUCH_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_cfg::R`](R) reader structure"]
impl crate::Readable for TOUCH_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`touch_cfg::W`](W) writer structure"]
impl crate::Writable for TOUCH_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOUCH_CFG to value 0x6600_0000"]
impl crate::Resettable for TOUCH_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x6600_0000;
}
