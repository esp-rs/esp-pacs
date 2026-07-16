#[doc = "Register `MEM_MODE_CONF` reader"]
pub type R = crate::R<MEM_MODE_CONF_SPEC>;
#[doc = "Register `MEM_MODE_CONF` writer"]
pub type W = crate::W<MEM_MODE_CONF_SPEC>;
#[doc = "Field `MODEM_RF1_MEM_SEL` reader - "]
pub type MODEM_RF1_MEM_SEL_R = crate::FieldReader;
#[doc = "Field `MODEM_RF1_MEM_SEL` writer - "]
pub type MODEM_RF1_MEM_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MODEM_RF2_MEM_SEL` reader - "]
pub type MODEM_RF2_MEM_SEL_R = crate::FieldReader;
#[doc = "Field `MODEM_RF2_MEM_SEL` writer - "]
pub type MODEM_RF2_MEM_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn modem_rf1_mem_sel(&self) -> MODEM_RF1_MEM_SEL_R {
        MODEM_RF1_MEM_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn modem_rf2_mem_sel(&self) -> MODEM_RF2_MEM_SEL_R {
        MODEM_RF2_MEM_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_MODE_CONF")
            .field("modem_rf1_mem_sel", &self.modem_rf1_mem_sel())
            .field("modem_rf2_mem_sel", &self.modem_rf2_mem_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn modem_rf1_mem_sel(&mut self) -> MODEM_RF1_MEM_SEL_W<'_, MEM_MODE_CONF_SPEC> {
        MODEM_RF1_MEM_SEL_W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn modem_rf2_mem_sel(&mut self) -> MODEM_RF2_MEM_SEL_W<'_, MEM_MODE_CONF_SPEC> {
        MODEM_RF2_MEM_SEL_W::new(self, 4)
    }
}
#[doc = "MEM_MODE_CONF\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_mode_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_mode_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_MODE_CONF_SPEC;
impl crate::RegisterSpec for MEM_MODE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_mode_conf::R`](R) reader structure"]
impl crate::Readable for MEM_MODE_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_mode_conf::W`](W) writer structure"]
impl crate::Writable for MEM_MODE_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_MODE_CONF to value 0x44"]
impl crate::Resettable for MEM_MODE_CONF_SPEC {
    const RESET_VALUE: u32 = 0x44;
}
