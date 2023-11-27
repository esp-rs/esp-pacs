#[doc = "Register `DPA_CTRL0` reader"]
pub type R = crate::R<DPA_CTRL0_SPEC>;
#[doc = "Register `DPA_CTRL0` writer"]
pub type W = crate::W<DPA_CTRL0_SPEC>;
#[doc = "Field `REG_SEC_DPA_LEVEL` reader - Reserved"]
pub type REG_SEC_DPA_LEVEL_R = crate::FieldReader;
#[doc = "Field `REG_SEC_DPA_LEVEL` writer - Reserved"]
pub type REG_SEC_DPA_LEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_SEC_DPA_CFG_SEL` reader - Reserved"]
pub type REG_SEC_DPA_CFG_SEL_R = crate::BitReader;
#[doc = "Field `REG_SEC_DPA_CFG_SEL` writer - Reserved"]
pub type REG_SEC_DPA_CFG_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn reg_sec_dpa_level(&self) -> REG_SEC_DPA_LEVEL_R {
        REG_SEC_DPA_LEVEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_sec_dpa_cfg_sel(&self) -> REG_SEC_DPA_CFG_SEL_R {
        REG_SEC_DPA_CFG_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPA_CTRL0")
            .field(
                "reg_sec_dpa_level",
                &format_args!("{}", self.reg_sec_dpa_level().bits()),
            )
            .field(
                "reg_sec_dpa_cfg_sel",
                &format_args!("{}", self.reg_sec_dpa_cfg_sel().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DPA_CTRL0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_sec_dpa_level(&mut self) -> REG_SEC_DPA_LEVEL_W<DPA_CTRL0_SPEC> {
        REG_SEC_DPA_LEVEL_W::new(self, 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_sec_dpa_cfg_sel(&mut self) -> REG_SEC_DPA_CFG_SEL_W<DPA_CTRL0_SPEC> {
        REG_SEC_DPA_CFG_SEL_W::new(self, 2)
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
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpa_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpa_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPA_CTRL0_SPEC;
impl crate::RegisterSpec for DPA_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpa_ctrl0::R`](R) reader structure"]
impl crate::Readable for DPA_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dpa_ctrl0::W`](W) writer structure"]
impl crate::Writable for DPA_CTRL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPA_CTRL0 to value 0"]
impl crate::Resettable for DPA_CTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
