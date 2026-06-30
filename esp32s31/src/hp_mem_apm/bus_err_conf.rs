#[doc = "Register `BUS_ERR_CONF` reader"]
pub type R = crate::R<BUS_ERR_CONF_SPEC>;
#[doc = "Register `BUS_ERR_CONF` writer"]
pub type W = crate::W<BUS_ERR_CONF_SPEC>;
#[doc = "Field `BUS_ERR_RESP_EN` reader - Configures whether return error response to cpu when access blocked 0: disable error response \\\\ 1: enable error response \\\\"]
pub type BUS_ERR_RESP_EN_R = crate::BitReader;
#[doc = "Field `BUS_ERR_RESP_EN` writer - Configures whether return error response to cpu when access blocked 0: disable error response \\\\ 1: enable error response \\\\"]
pub type BUS_ERR_RESP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether return error response to cpu when access blocked 0: disable error response \\\\ 1: enable error response \\\\"]
    #[inline(always)]
    pub fn bus_err_resp_en(&self) -> BUS_ERR_RESP_EN_R {
        BUS_ERR_RESP_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUS_ERR_CONF")
            .field("bus_err_resp_en", &self.bus_err_resp_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether return error response to cpu when access blocked 0: disable error response \\\\ 1: enable error response \\\\"]
    #[inline(always)]
    pub fn bus_err_resp_en(&mut self) -> BUS_ERR_RESP_EN_W<'_, BUS_ERR_CONF_SPEC> {
        BUS_ERR_RESP_EN_W::new(self, 0)
    }
}
#[doc = "APM interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_err_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_err_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUS_ERR_CONF_SPEC;
impl crate::RegisterSpec for BUS_ERR_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_err_conf::R`](R) reader structure"]
impl crate::Readable for BUS_ERR_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bus_err_conf::W`](W) writer structure"]
impl crate::Writable for BUS_ERR_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUS_ERR_CONF to value 0"]
impl crate::Resettable for BUS_ERR_CONF_SPEC {}
