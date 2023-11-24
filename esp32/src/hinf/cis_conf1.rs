#[doc = "Register `CIS_CONF1` reader"]
pub type R = crate::R<CIS_CONF1_SPEC>;
#[doc = "Register `CIS_CONF1` writer"]
pub type W = crate::W<CIS_CONF1_SPEC>;
#[doc = "Field `CIS_CONF_W1` reader - "]
pub type CIS_CONF_W1_R = crate::FieldReader<u32>;
#[doc = "Field `CIS_CONF_W1` writer - "]
pub type CIS_CONF_W1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cis_conf_w1(&self) -> CIS_CONF_W1_R {
        CIS_CONF_W1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIS_CONF1")
            .field(
                "cis_conf_w1",
                &format_args!("{}", self.cis_conf_w1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CIS_CONF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn cis_conf_w1(&mut self) -> CIS_CONF_W1_W<CIS_CONF1_SPEC> {
        CIS_CONF_W1_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cis_conf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cis_conf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIS_CONF1_SPEC;
impl crate::RegisterSpec for CIS_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cis_conf1::R`](R) reader structure"]
impl crate::Readable for CIS_CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cis_conf1::W`](W) writer structure"]
impl crate::Writable for CIS_CONF1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CIS_CONF1 to value 0xffff_ffff"]
impl crate::Resettable for CIS_CONF1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
