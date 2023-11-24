#[doc = "Register `SRAM_PD_CTRL_1` reader"]
pub type R = crate::R<SRAM_PD_CTRL_1_SPEC>;
#[doc = "Register `SRAM_PD_CTRL_1` writer"]
pub type W = crate::W<SRAM_PD_CTRL_1_SPEC>;
#[doc = "Field `SRAM_PD_1` reader - "]
pub type SRAM_PD_1_R = crate::BitReader;
#[doc = "Field `SRAM_PD_1` writer - "]
pub type SRAM_PD_1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sram_pd_1(&self) -> SRAM_PD_1_R {
        SRAM_PD_1_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_PD_CTRL_1")
            .field("sram_pd_1", &format_args!("{}", self.sram_pd_1().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SRAM_PD_CTRL_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sram_pd_1(&mut self) -> SRAM_PD_1_W<SRAM_PD_CTRL_1_SPEC> {
        SRAM_PD_1_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_pd_ctrl_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_pd_ctrl_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_PD_CTRL_1_SPEC;
impl crate::RegisterSpec for SRAM_PD_CTRL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_pd_ctrl_1::R`](R) reader structure"]
impl crate::Readable for SRAM_PD_CTRL_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sram_pd_ctrl_1::W`](W) writer structure"]
impl crate::Writable for SRAM_PD_CTRL_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRAM_PD_CTRL_1 to value 0"]
impl crate::Resettable for SRAM_PD_CTRL_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
