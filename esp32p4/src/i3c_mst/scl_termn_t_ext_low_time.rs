#[doc = "Register `SCL_TERMN_T_EXT_LOW_TIME` reader"]
pub type R = crate::R<SCL_TERMN_T_EXT_LOW_TIME_SPEC>;
#[doc = "Register `SCL_TERMN_T_EXT_LOW_TIME` writer"]
pub type W = crate::W<SCL_TERMN_T_EXT_LOW_TIME_SPEC>;
#[doc = "Field `REG_I3C_MST_TERMN_T_EXT_LOW_TIME` reader - NA"]
pub type REG_I3C_MST_TERMN_T_EXT_LOW_TIME_R = crate::FieldReader;
#[doc = "Field `REG_I3C_MST_TERMN_T_EXT_LOW_TIME` writer - NA"]
pub type REG_I3C_MST_TERMN_T_EXT_LOW_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    pub fn reg_i3c_mst_termn_t_ext_low_time(&self) -> REG_I3C_MST_TERMN_T_EXT_LOW_TIME_R {
        REG_I3C_MST_TERMN_T_EXT_LOW_TIME_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCL_TERMN_T_EXT_LOW_TIME")
            .field(
                "reg_i3c_mst_termn_t_ext_low_time",
                &format_args!("{}", self.reg_i3c_mst_termn_t_ext_low_time().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SCL_TERMN_T_EXT_LOW_TIME_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_i3c_mst_termn_t_ext_low_time(
        &mut self,
    ) -> REG_I3C_MST_TERMN_T_EXT_LOW_TIME_W<SCL_TERMN_T_EXT_LOW_TIME_SPEC> {
        REG_I3C_MST_TERMN_T_EXT_LOW_TIME_W::new(self, 0)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_termn_t_ext_low_time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_termn_t_ext_low_time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCL_TERMN_T_EXT_LOW_TIME_SPEC;
impl crate::RegisterSpec for SCL_TERMN_T_EXT_LOW_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl_termn_t_ext_low_time::R`](R) reader structure"]
impl crate::Readable for SCL_TERMN_T_EXT_LOW_TIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scl_termn_t_ext_low_time::W`](W) writer structure"]
impl crate::Writable for SCL_TERMN_T_EXT_LOW_TIME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCL_TERMN_T_EXT_LOW_TIME to value 0x02"]
impl crate::Resettable for SCL_TERMN_T_EXT_LOW_TIME_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
