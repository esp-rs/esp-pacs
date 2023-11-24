#[doc = "Register `DPORT_CTL` reader"]
pub type R = crate::R<DPORT_CTL_SPEC>;
#[doc = "Register `DPORT_CTL` writer"]
pub type W = crate::W<DPORT_CTL_SPEC>;
#[doc = "Field `DPORT_CTL_DOUBLE_CLK` reader - "]
pub type DPORT_CTL_DOUBLE_CLK_R = crate::BitReader;
#[doc = "Field `DPORT_CTL_DOUBLE_CLK` writer - "]
pub type DPORT_CTL_DOUBLE_CLK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_ctl_double_clk(&self) -> DPORT_CTL_DOUBLE_CLK_R {
        DPORT_CTL_DOUBLE_CLK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPORT_CTL")
            .field(
                "dport_ctl_double_clk",
                &format_args!("{}", self.dport_ctl_double_clk().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DPORT_CTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dport_ctl_double_clk(&mut self) -> DPORT_CTL_DOUBLE_CLK_W<DPORT_CTL_SPEC> {
        DPORT_CTL_DOUBLE_CLK_W::new(self, 0)
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
#[doc = "DPORT_CTL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dport_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dport_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPORT_CTL_SPEC;
impl crate::RegisterSpec for DPORT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dport_ctl::R`](R) reader structure"]
impl crate::Readable for DPORT_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dport_ctl::W`](W) writer structure"]
impl crate::Writable for DPORT_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPORT_CTL to value 0"]
impl crate::Resettable for DPORT_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
