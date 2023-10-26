#[doc = "Register `LPMEM_FORCE` reader"]
pub type R = crate::R<LPMEM_FORCE_SPEC>;
#[doc = "Register `LPMEM_FORCE` writer"]
pub type W = crate::W<LPMEM_FORCE_SPEC>;
#[doc = "Field `LPMEM_CLK_FORCE_ON` reader - need_des"]
pub type LPMEM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `LPMEM_CLK_FORCE_ON` writer - need_des"]
pub type LPMEM_CLK_FORCE_ON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lpmem_clk_force_on(&self) -> LPMEM_CLK_FORCE_ON_R {
        LPMEM_CLK_FORCE_ON_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPMEM_FORCE")
            .field(
                "lpmem_clk_force_on",
                &format_args!("{}", self.lpmem_clk_force_on().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LPMEM_FORCE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lpmem_clk_force_on(&mut self) -> LPMEM_CLK_FORCE_ON_W<LPMEM_FORCE_SPEC, 31> {
        LPMEM_CLK_FORCE_ON_W::new(self)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpmem_force::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpmem_force::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPMEM_FORCE_SPEC;
impl crate::RegisterSpec for LPMEM_FORCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpmem_force::R`](R) reader structure"]
impl crate::Readable for LPMEM_FORCE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lpmem_force::W`](W) writer structure"]
impl crate::Writable for LPMEM_FORCE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPMEM_FORCE to value 0"]
impl crate::Resettable for LPMEM_FORCE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
