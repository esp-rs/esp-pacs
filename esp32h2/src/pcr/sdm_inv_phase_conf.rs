#[doc = "Register `SDM_INV_PHASE_CONF` reader"]
pub type R = crate::R<SDM_INV_PHASE_CONF_SPEC>;
#[doc = "Register `SDM_INV_PHASE_CONF` writer"]
pub type W = crate::W<SDM_INV_PHASE_CONF_SPEC>;
#[doc = "Field `CLK_SDM_INV_PHASE_ENA` reader - xxxx"]
pub type CLK_SDM_INV_PHASE_ENA_R = crate::BitReader;
#[doc = "Field `CLK_SDM_INV_PHASE_ENA` writer - xxxx"]
pub type CLK_SDM_INV_PHASE_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SDM_INV_PHASE_SEL` reader - xxxx"]
pub type CLK_SDM_INV_PHASE_SEL_R = crate::FieldReader;
#[doc = "Field `CLK_SDM_INV_PHASE_SEL` writer - xxxx"]
pub type CLK_SDM_INV_PHASE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - xxxx"]
    #[inline(always)]
    pub fn clk_sdm_inv_phase_ena(&self) -> CLK_SDM_INV_PHASE_ENA_R {
        CLK_SDM_INV_PHASE_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - xxxx"]
    #[inline(always)]
    pub fn clk_sdm_inv_phase_sel(&self) -> CLK_SDM_INV_PHASE_SEL_R {
        CLK_SDM_INV_PHASE_SEL_R::new(((self.bits >> 1) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDM_INV_PHASE_CONF")
            .field(
                "clk_sdm_inv_phase_ena",
                &format_args!("{}", self.clk_sdm_inv_phase_ena().bit()),
            )
            .field(
                "clk_sdm_inv_phase_sel",
                &format_args!("{}", self.clk_sdm_inv_phase_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SDM_INV_PHASE_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - xxxx"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sdm_inv_phase_ena(&mut self) -> CLK_SDM_INV_PHASE_ENA_W<SDM_INV_PHASE_CONF_SPEC> {
        CLK_SDM_INV_PHASE_ENA_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - xxxx"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sdm_inv_phase_sel(&mut self) -> CLK_SDM_INV_PHASE_SEL_W<SDM_INV_PHASE_CONF_SPEC> {
        CLK_SDM_INV_PHASE_SEL_W::new(self, 1)
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
#[doc = "xxxx\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdm_inv_phase_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdm_inv_phase_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDM_INV_PHASE_CONF_SPEC;
impl crate::RegisterSpec for SDM_INV_PHASE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdm_inv_phase_conf::R`](R) reader structure"]
impl crate::Readable for SDM_INV_PHASE_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdm_inv_phase_conf::W`](W) writer structure"]
impl crate::Writable for SDM_INV_PHASE_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDM_INV_PHASE_CONF to value 0"]
impl crate::Resettable for SDM_INV_PHASE_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
