#[doc = "Register `EXT_LDO_P0_0P1A_ANA` reader"]
pub type R = crate::R<EXT_LDO_P0_0P1A_ANA_SPEC>;
#[doc = "Register `EXT_LDO_P0_0P1A_ANA` writer"]
pub type W = crate::W<EXT_LDO_P0_0P1A_ANA_SPEC>;
#[doc = "Field `ANA_0P1A_MUL_0` reader - need_des"]
pub type ANA_0P1A_MUL_0_R = crate::FieldReader;
#[doc = "Field `ANA_0P1A_MUL_0` writer - need_des"]
pub type ANA_0P1A_MUL_0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ANA_0P1A_EN_VDET_0` reader - need_des"]
pub type ANA_0P1A_EN_VDET_0_R = crate::BitReader;
#[doc = "Field `ANA_0P1A_EN_VDET_0` writer - need_des"]
pub type ANA_0P1A_EN_VDET_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANA_0P1A_EN_CUR_LIM_0` reader - need_des"]
pub type ANA_0P1A_EN_CUR_LIM_0_R = crate::BitReader;
#[doc = "Field `ANA_0P1A_EN_CUR_LIM_0` writer - need_des"]
pub type ANA_0P1A_EN_CUR_LIM_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANA_0P1A_DREF_0` reader - need_des"]
pub type ANA_0P1A_DREF_0_R = crate::FieldReader;
#[doc = "Field `ANA_0P1A_DREF_0` writer - need_des"]
pub type ANA_0P1A_DREF_0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 23:25 - need_des"]
    #[inline(always)]
    pub fn ana_0p1a_mul_0(&self) -> ANA_0P1A_MUL_0_R {
        ANA_0P1A_MUL_0_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn ana_0p1a_en_vdet_0(&self) -> ANA_0P1A_EN_VDET_0_R {
        ANA_0P1A_EN_VDET_0_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn ana_0p1a_en_cur_lim_0(&self) -> ANA_0P1A_EN_CUR_LIM_0_R {
        ANA_0P1A_EN_CUR_LIM_0_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - need_des"]
    #[inline(always)]
    pub fn ana_0p1a_dref_0(&self) -> ANA_0P1A_DREF_0_R {
        ANA_0P1A_DREF_0_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_LDO_P0_0P1A_ANA")
            .field(
                "ana_0p1a_mul_0",
                &format_args!("{}", self.ana_0p1a_mul_0().bits()),
            )
            .field(
                "ana_0p1a_en_vdet_0",
                &format_args!("{}", self.ana_0p1a_en_vdet_0().bit()),
            )
            .field(
                "ana_0p1a_en_cur_lim_0",
                &format_args!("{}", self.ana_0p1a_en_cur_lim_0().bit()),
            )
            .field(
                "ana_0p1a_dref_0",
                &format_args!("{}", self.ana_0p1a_dref_0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EXT_LDO_P0_0P1A_ANA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 23:25 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ana_0p1a_mul_0(&mut self) -> ANA_0P1A_MUL_0_W<EXT_LDO_P0_0P1A_ANA_SPEC> {
        ANA_0P1A_MUL_0_W::new(self, 23)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ana_0p1a_en_vdet_0(&mut self) -> ANA_0P1A_EN_VDET_0_W<EXT_LDO_P0_0P1A_ANA_SPEC> {
        ANA_0P1A_EN_VDET_0_W::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ana_0p1a_en_cur_lim_0(&mut self) -> ANA_0P1A_EN_CUR_LIM_0_W<EXT_LDO_P0_0P1A_ANA_SPEC> {
        ANA_0P1A_EN_CUR_LIM_0_W::new(self, 27)
    }
    #[doc = "Bits 28:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ana_0p1a_dref_0(&mut self) -> ANA_0P1A_DREF_0_W<EXT_LDO_P0_0P1A_ANA_SPEC> {
        ANA_0P1A_DREF_0_W::new(self, 28)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_ldo_p0_0p1a_ana::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_ldo_p0_0p1a_ana::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT_LDO_P0_0P1A_ANA_SPEC;
impl crate::RegisterSpec for EXT_LDO_P0_0P1A_ANA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_ldo_p0_0p1a_ana::R`](R) reader structure"]
impl crate::Readable for EXT_LDO_P0_0P1A_ANA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext_ldo_p0_0p1a_ana::W`](W) writer structure"]
impl crate::Writable for EXT_LDO_P0_0P1A_ANA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXT_LDO_P0_0P1A_ANA to value 0xb100_0000"]
impl crate::Resettable for EXT_LDO_P0_0P1A_ANA_SPEC {
    const RESET_VALUE: u32 = 0xb100_0000;
}
