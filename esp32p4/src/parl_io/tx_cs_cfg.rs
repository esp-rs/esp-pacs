#[doc = "Register `TX_CS_CFG` reader"]
pub type R = crate::R<TX_CS_CFG_SPEC>;
#[doc = "Register `TX_CS_CFG` writer"]
pub type W = crate::W<TX_CS_CFG_SPEC>;
#[doc = "Field `VAL` reader - "]
pub type VAL_R = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - "]
pub type VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CS_CFG")
            .field("val", &self.val())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&mut self) -> VAL_W<'_, TX_CS_CFG_SPEC> {
        VAL_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_cs_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_cs_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_CS_CFG_SPEC;
impl crate::RegisterSpec for TX_CS_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_cs_cfg::R`](R) reader structure"]
impl crate::Readable for TX_CS_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_cs_cfg::W`](W) writer structure"]
impl crate::Writable for TX_CS_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_CS_CFG to value 0"]
impl crate::Resettable for TX_CS_CFG_SPEC {}
