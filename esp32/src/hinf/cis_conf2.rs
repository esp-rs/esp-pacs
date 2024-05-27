#[doc = "Register `CIS_CONF2` reader"]
pub type R = crate::R<CIS_CONF2_SPEC>;
#[doc = "Register `CIS_CONF2` writer"]
pub type W = crate::W<CIS_CONF2_SPEC>;
#[doc = "Field `CIS_CONF_W2` reader - "]
pub type CIS_CONF_W2_R = crate::FieldReader<u32>;
#[doc = "Field `CIS_CONF_W2` writer - "]
pub type CIS_CONF_W2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cis_conf_w2(&self) -> CIS_CONF_W2_R {
        CIS_CONF_W2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIS_CONF2")
            .field("cis_conf_w2", &self.cis_conf_w2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn cis_conf_w2(&mut self) -> CIS_CONF_W2_W<CIS_CONF2_SPEC> {
        CIS_CONF_W2_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cis_conf2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cis_conf2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIS_CONF2_SPEC;
impl crate::RegisterSpec for CIS_CONF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cis_conf2::R`](R) reader structure"]
impl crate::Readable for CIS_CONF2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cis_conf2::W`](W) writer structure"]
impl crate::Writable for CIS_CONF2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CIS_CONF2 to value 0xffff_ffff"]
impl crate::Resettable for CIS_CONF2_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
