#[doc = "Register `ECO_CONF` reader"]
pub type R = crate::R<ECO_CONF_SPEC>;
#[doc = "Register `ECO_CONF` writer"]
pub type W = crate::W<ECO_CONF_SPEC>;
#[doc = "Field `RDN_ENA` reader - enable rdn counter bit"]
pub type RDN_ENA_R = crate::BitReader;
#[doc = "Field `RDN_ENA` writer - enable rdn counter bit"]
pub type RDN_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDN_RESULT` reader - rdn result"]
pub type RDN_RESULT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - enable rdn counter bit"]
    #[inline(always)]
    pub fn rdn_ena(&self) -> RDN_ENA_R {
        RDN_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - rdn result"]
    #[inline(always)]
    pub fn rdn_result(&self) -> RDN_RESULT_R {
        RDN_RESULT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECO_CONF")
            .field("rdn_ena", &self.rdn_ena())
            .field("rdn_result", &self.rdn_result())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - enable rdn counter bit"]
    #[inline(always)]
    pub fn rdn_ena(&mut self) -> RDN_ENA_W<ECO_CONF_SPEC> {
        RDN_ENA_W::new(self, 0)
    }
}
#[doc = "I2S ECO register\n\nYou can [`read`](crate::Reg::read) this register and get [`eco_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eco_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECO_CONF_SPEC;
impl crate::RegisterSpec for ECO_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eco_conf::R`](R) reader structure"]
impl crate::Readable for ECO_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eco_conf::W`](W) writer structure"]
impl crate::Writable for ECO_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECO_CONF to value 0"]
impl crate::Resettable for ECO_CONF_SPEC {}
