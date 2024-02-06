#[doc = "Register `GEN0_STMP_CFG` reader"]
pub type R = crate::R<GEN0_STMP_CFG_SPEC>;
#[doc = "Register `GEN0_STMP_CFG` writer"]
pub type W = crate::W<GEN0_STMP_CFG_SPEC>;
#[doc = "Field `GEN0_A_UPMETHOD` reader - "]
pub type GEN0_A_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `GEN0_A_UPMETHOD` writer - "]
pub type GEN0_A_UPMETHOD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GEN0_B_UPMETHOD` reader - "]
pub type GEN0_B_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `GEN0_B_UPMETHOD` writer - "]
pub type GEN0_B_UPMETHOD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GEN0_A_SHDW_FULL` reader - "]
pub type GEN0_A_SHDW_FULL_R = crate::BitReader;
#[doc = "Field `GEN0_A_SHDW_FULL` writer - "]
pub type GEN0_A_SHDW_FULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN0_B_SHDW_FULL` reader - "]
pub type GEN0_B_SHDW_FULL_R = crate::BitReader;
#[doc = "Field `GEN0_B_SHDW_FULL` writer - "]
pub type GEN0_B_SHDW_FULL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn gen0_a_upmethod(&self) -> GEN0_A_UPMETHOD_R {
        GEN0_A_UPMETHOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn gen0_b_upmethod(&self) -> GEN0_B_UPMETHOD_R {
        GEN0_B_UPMETHOD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gen0_a_shdw_full(&self) -> GEN0_A_SHDW_FULL_R {
        GEN0_A_SHDW_FULL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gen0_b_shdw_full(&self) -> GEN0_B_SHDW_FULL_R {
        GEN0_B_SHDW_FULL_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEN0_STMP_CFG")
            .field(
                "gen0_a_upmethod",
                &format_args!("{}", self.gen0_a_upmethod().bits()),
            )
            .field(
                "gen0_b_upmethod",
                &format_args!("{}", self.gen0_b_upmethod().bits()),
            )
            .field(
                "gen0_a_shdw_full",
                &format_args!("{}", self.gen0_a_shdw_full().bit()),
            )
            .field(
                "gen0_b_shdw_full",
                &format_args!("{}", self.gen0_b_shdw_full().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GEN0_STMP_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn gen0_a_upmethod(&mut self) -> GEN0_A_UPMETHOD_W<GEN0_STMP_CFG_SPEC> {
        GEN0_A_UPMETHOD_W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn gen0_b_upmethod(&mut self) -> GEN0_B_UPMETHOD_W<GEN0_STMP_CFG_SPEC> {
        GEN0_B_UPMETHOD_W::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn gen0_a_shdw_full(&mut self) -> GEN0_A_SHDW_FULL_W<GEN0_STMP_CFG_SPEC> {
        GEN0_A_SHDW_FULL_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn gen0_b_shdw_full(&mut self) -> GEN0_B_SHDW_FULL_W<GEN0_STMP_CFG_SPEC> {
        GEN0_B_SHDW_FULL_W::new(self, 9)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen0_stmp_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen0_stmp_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GEN0_STMP_CFG_SPEC;
impl crate::RegisterSpec for GEN0_STMP_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gen0_stmp_cfg::R`](R) reader structure"]
impl crate::Readable for GEN0_STMP_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gen0_stmp_cfg::W`](W) writer structure"]
impl crate::Writable for GEN0_STMP_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GEN0_STMP_CFG to value 0"]
impl crate::Resettable for GEN0_STMP_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
