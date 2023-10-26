#[doc = "Register `ECO_CFG` reader"]
pub type R = crate::R<ECO_CFG_SPEC>;
#[doc = "Register `ECO_CFG` writer"]
pub type W = crate::W<ECO_CFG_SPEC>;
#[doc = "Field `RDN_ENA` reader - Enable eco module."]
pub type RDN_ENA_R = crate::BitReader;
#[doc = "Field `RDN_ENA` writer - Enable eco module."]
pub type RDN_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RDN_RESULT` reader - Output of eco module."]
pub type RDN_RESULT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Enable eco module."]
    #[inline(always)]
    pub fn rdn_ena(&self) -> RDN_ENA_R {
        RDN_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output of eco module."]
    #[inline(always)]
    pub fn rdn_result(&self) -> RDN_RESULT_R {
        RDN_RESULT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECO_CFG")
            .field("rdn_ena", &format_args!("{}", self.rdn_ena().bit()))
            .field("rdn_result", &format_args!("{}", self.rdn_result().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ECO_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Enable eco module."]
    #[inline(always)]
    #[must_use]
    pub fn rdn_ena(&mut self) -> RDN_ENA_W<ECO_CFG_SPEC, 0> {
        RDN_ENA_W::new(self)
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
#[doc = "ECO configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eco_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eco_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECO_CFG_SPEC;
impl crate::RegisterSpec for ECO_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eco_cfg::R`](R) reader structure"]
impl crate::Readable for ECO_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eco_cfg::W`](W) writer structure"]
impl crate::Writable for ECO_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECO_CFG to value 0x02"]
impl crate::Resettable for ECO_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
