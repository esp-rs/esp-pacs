#[doc = "Register `PDM_FREQ_CONF` reader"]
pub type R = crate::R<PDM_FREQ_CONF_SPEC>;
#[doc = "Register `PDM_FREQ_CONF` writer"]
pub type W = crate::W<PDM_FREQ_CONF_SPEC>;
#[doc = "Field `TX_PDM_FS` reader - "]
pub type TX_PDM_FS_R = crate::FieldReader<u16>;
#[doc = "Field `TX_PDM_FS` writer - "]
pub type TX_PDM_FS_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TX_PDM_FP` reader - "]
pub type TX_PDM_FP_R = crate::FieldReader<u16>;
#[doc = "Field `TX_PDM_FP` writer - "]
pub type TX_PDM_FP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_pdm_fs(&self) -> TX_PDM_FS_R {
        TX_PDM_FS_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn tx_pdm_fp(&self) -> TX_PDM_FP_R {
        TX_PDM_FP_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDM_FREQ_CONF")
            .field("tx_pdm_fs", &self.tx_pdm_fs())
            .field("tx_pdm_fp", &self.tx_pdm_fp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_fs(&mut self) -> TX_PDM_FS_W<PDM_FREQ_CONF_SPEC> {
        TX_PDM_FS_W::new(self, 0)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_fp(&mut self) -> TX_PDM_FP_W<PDM_FREQ_CONF_SPEC> {
        TX_PDM_FP_W::new(self, 10)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdm_freq_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdm_freq_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDM_FREQ_CONF_SPEC;
impl crate::RegisterSpec for PDM_FREQ_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdm_freq_conf::R`](R) reader structure"]
impl crate::Readable for PDM_FREQ_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pdm_freq_conf::W`](W) writer structure"]
impl crate::Writable for PDM_FREQ_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDM_FREQ_CONF to value 0x000f_01e0"]
impl crate::Resettable for PDM_FREQ_CONF_SPEC {
    const RESET_VALUE: u32 = 0x000f_01e0;
}
