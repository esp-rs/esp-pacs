#[doc = "Register `LP_AONCLKRST_HPCPU_RESET_CTRL1` reader"]
pub type R = crate::R<LP_AONCLKRST_HPCPU_RESET_CTRL1_SPEC>;
#[doc = "Register `LP_AONCLKRST_HPCPU_RESET_CTRL1` writer"]
pub type W = crate::W<LP_AONCLKRST_HPCPU_RESET_CTRL1_SPEC>;
#[doc = "Field `LP_AONCLKRST_HPCORE0_SW_STALL_CODE` reader - HP core0 software stall when set to 8'h86"]
pub type LP_AONCLKRST_HPCORE0_SW_STALL_CODE_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_HPCORE0_SW_STALL_CODE` writer - HP core0 software stall when set to 8'h86"]
pub type LP_AONCLKRST_HPCORE0_SW_STALL_CODE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LP_AONCLKRST_HPCORE1_SW_STALL_CODE` reader - HP core1 software stall when set to 8'h86"]
pub type LP_AONCLKRST_HPCORE1_SW_STALL_CODE_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_HPCORE1_SW_STALL_CODE` writer - HP core1 software stall when set to 8'h86"]
pub type LP_AONCLKRST_HPCORE1_SW_STALL_CODE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 16:23 - HP core0 software stall when set to 8'h86"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_sw_stall_code(&self) -> LP_AONCLKRST_HPCORE0_SW_STALL_CODE_R {
        LP_AONCLKRST_HPCORE0_SW_STALL_CODE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - HP core1 software stall when set to 8'h86"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore1_sw_stall_code(&self) -> LP_AONCLKRST_HPCORE1_SW_STALL_CODE_R {
        LP_AONCLKRST_HPCORE1_SW_STALL_CODE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_AONCLKRST_HPCPU_RESET_CTRL1")
            .field(
                "lp_aonclkrst_hpcore0_sw_stall_code",
                &format_args!("{}", self.lp_aonclkrst_hpcore0_sw_stall_code().bits()),
            )
            .field(
                "lp_aonclkrst_hpcore1_sw_stall_code",
                &format_args!("{}", self.lp_aonclkrst_hpcore1_sw_stall_code().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_AONCLKRST_HPCPU_RESET_CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 16:23 - HP core0 software stall when set to 8'h86"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_hpcore0_sw_stall_code(
        &mut self,
    ) -> LP_AONCLKRST_HPCORE0_SW_STALL_CODE_W<LP_AONCLKRST_HPCPU_RESET_CTRL1_SPEC> {
        LP_AONCLKRST_HPCORE0_SW_STALL_CODE_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - HP core1 software stall when set to 8'h86"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_hpcore1_sw_stall_code(
        &mut self,
    ) -> LP_AONCLKRST_HPCORE1_SW_STALL_CODE_W<LP_AONCLKRST_HPCPU_RESET_CTRL1_SPEC> {
        LP_AONCLKRST_HPCORE1_SW_STALL_CODE_W::new(self, 24)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_hpcpu_reset_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_hpcpu_reset_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_AONCLKRST_HPCPU_RESET_CTRL1_SPEC;
impl crate::RegisterSpec for LP_AONCLKRST_HPCPU_RESET_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_hpcpu_reset_ctrl1::R`](R) reader structure"]
impl crate::Readable for LP_AONCLKRST_HPCPU_RESET_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_hpcpu_reset_ctrl1::W`](W) writer structure"]
impl crate::Writable for LP_AONCLKRST_HPCPU_RESET_CTRL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_AONCLKRST_HPCPU_RESET_CTRL1 to value 0"]
impl crate::Resettable for LP_AONCLKRST_HPCPU_RESET_CTRL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
