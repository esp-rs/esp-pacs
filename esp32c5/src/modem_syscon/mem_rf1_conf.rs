#[doc = "Register `MEM_RF1_CONF` reader"]
pub type R = crate::R<MEM_RF1_CONF_SPEC>;
#[doc = "Register `MEM_RF1_CONF` writer"]
pub type W = crate::W<MEM_RF1_CONF_SPEC>;
#[doc = "Field `MODEM_RF1_MEM_AUX_CTRL` reader - "]
pub type MODEM_RF1_MEM_AUX_CTRL_R = crate::FieldReader<u32>;
#[doc = "Field `MODEM_RF1_MEM_AUX_CTRL` writer - "]
pub type MODEM_RF1_MEM_AUX_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn modem_rf1_mem_aux_ctrl(&self) -> MODEM_RF1_MEM_AUX_CTRL_R {
        MODEM_RF1_MEM_AUX_CTRL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_RF1_CONF")
            .field("modem_rf1_mem_aux_ctrl", &self.modem_rf1_mem_aux_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn modem_rf1_mem_aux_ctrl(&mut self) -> MODEM_RF1_MEM_AUX_CTRL_W<'_, MEM_RF1_CONF_SPEC> {
        MODEM_RF1_MEM_AUX_CTRL_W::new(self, 0)
    }
}
#[doc = "MEM_RF1_CONF\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_rf1_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_rf1_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_RF1_CONF_SPEC;
impl crate::RegisterSpec for MEM_RF1_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_rf1_conf::R`](R) reader structure"]
impl crate::Readable for MEM_RF1_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_rf1_conf::W`](W) writer structure"]
impl crate::Writable for MEM_RF1_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_RF1_CONF to value 0x2070"]
impl crate::Resettable for MEM_RF1_CONF_SPEC {
    const RESET_VALUE: u32 = 0x2070;
}
