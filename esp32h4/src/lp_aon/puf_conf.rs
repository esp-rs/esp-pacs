#[doc = "Register `PUF_CONF` reader"]
pub type R = crate::R<PUF_CONF_SPEC>;
#[doc = "Register `PUF_CONF` writer"]
pub type W = crate::W<PUF_CONF_SPEC>;
#[doc = "Field `PUF_SW` reader - puf mem power switch control signal"]
pub type PUF_SW_R = crate::BitReader;
#[doc = "Field `PUF_SW` writer - puf mem power switch control signal"]
pub type PUF_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUF_ISO_EN` reader - iso enable signal for puf mem"]
pub type PUF_ISO_EN_R = crate::BitReader;
#[doc = "Field `PUF_ISO_EN` writer - iso enable signal for puf mem"]
pub type PUF_ISO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUF_MEM_DISCHARGE` reader - discharge signal for puf mem"]
pub type PUF_MEM_DISCHARGE_R = crate::BitReader;
#[doc = "Field `PUF_MEM_DISCHARGE` writer - discharge signal for puf mem"]
pub type PUF_MEM_DISCHARGE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - puf mem power switch control signal"]
    #[inline(always)]
    pub fn puf_sw(&self) -> PUF_SW_R {
        PUF_SW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - iso enable signal for puf mem"]
    #[inline(always)]
    pub fn puf_iso_en(&self) -> PUF_ISO_EN_R {
        PUF_ISO_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - discharge signal for puf mem"]
    #[inline(always)]
    pub fn puf_mem_discharge(&self) -> PUF_MEM_DISCHARGE_R {
        PUF_MEM_DISCHARGE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUF_CONF")
            .field("puf_sw", &self.puf_sw())
            .field("puf_iso_en", &self.puf_iso_en())
            .field("puf_mem_discharge", &self.puf_mem_discharge())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - puf mem power switch control signal"]
    #[inline(always)]
    pub fn puf_sw(&mut self) -> PUF_SW_W<'_, PUF_CONF_SPEC> {
        PUF_SW_W::new(self, 0)
    }
    #[doc = "Bit 1 - iso enable signal for puf mem"]
    #[inline(always)]
    pub fn puf_iso_en(&mut self) -> PUF_ISO_EN_W<'_, PUF_CONF_SPEC> {
        PUF_ISO_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - discharge signal for puf mem"]
    #[inline(always)]
    pub fn puf_mem_discharge(&mut self) -> PUF_MEM_DISCHARGE_W<'_, PUF_CONF_SPEC> {
        PUF_MEM_DISCHARGE_W::new(self, 2)
    }
}
#[doc = "PUF mem control config register\n\nYou can [`read`](crate::Reg::read) this register and get [`puf_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`puf_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PUF_CONF_SPEC;
impl crate::RegisterSpec for PUF_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`puf_conf::R`](R) reader structure"]
impl crate::Readable for PUF_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`puf_conf::W`](W) writer structure"]
impl crate::Writable for PUF_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PUF_CONF to value 0x01"]
impl crate::Resettable for PUF_CONF_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
