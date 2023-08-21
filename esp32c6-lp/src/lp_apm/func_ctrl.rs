#[doc = "Register `FUNC_CTRL` reader"]
pub type R = crate::R<FUNC_CTRL_SPEC>;
#[doc = "Register `FUNC_CTRL` writer"]
pub type W = crate::W<FUNC_CTRL_SPEC>;
#[doc = "Field `M0_PMS_FUNC_EN` reader - PMS M0 function enable"]
pub type M0_PMS_FUNC_EN_R = crate::BitReader;
#[doc = "Field `M0_PMS_FUNC_EN` writer - PMS M0 function enable"]
pub type M0_PMS_FUNC_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `M1_PMS_FUNC_EN` reader - PMS M1 function enable"]
pub type M1_PMS_FUNC_EN_R = crate::BitReader;
#[doc = "Field `M1_PMS_FUNC_EN` writer - PMS M1 function enable"]
pub type M1_PMS_FUNC_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - PMS M0 function enable"]
    #[inline(always)]
    pub fn m0_pms_func_en(&self) -> M0_PMS_FUNC_EN_R {
        M0_PMS_FUNC_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PMS M1 function enable"]
    #[inline(always)]
    pub fn m1_pms_func_en(&self) -> M1_PMS_FUNC_EN_R {
        M1_PMS_FUNC_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNC_CTRL")
            .field(
                "m0_pms_func_en",
                &format_args!("{}", self.m0_pms_func_en().bit()),
            )
            .field(
                "m1_pms_func_en",
                &format_args!("{}", self.m1_pms_func_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FUNC_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - PMS M0 function enable"]
    #[inline(always)]
    #[must_use]
    pub fn m0_pms_func_en(&mut self) -> M0_PMS_FUNC_EN_W<FUNC_CTRL_SPEC, 0> {
        M0_PMS_FUNC_EN_W::new(self)
    }
    #[doc = "Bit 1 - PMS M1 function enable"]
    #[inline(always)]
    #[must_use]
    pub fn m1_pms_func_en(&mut self) -> M1_PMS_FUNC_EN_W<FUNC_CTRL_SPEC, 1> {
        M1_PMS_FUNC_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PMS function control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FUNC_CTRL_SPEC;
impl crate::RegisterSpec for FUNC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func_ctrl::R`](R) reader structure"]
impl crate::Readable for FUNC_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`func_ctrl::W`](W) writer structure"]
impl crate::Writable for FUNC_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FUNC_CTRL to value 0x03"]
impl crate::Resettable for FUNC_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
