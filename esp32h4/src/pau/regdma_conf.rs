#[doc = "Register `REGDMA_CONF` reader"]
pub type R = crate::R<REGDMA_CONF_SPEC>;
#[doc = "Register `REGDMA_CONF` writer"]
pub type W = crate::W<REGDMA_CONF_SPEC>;
#[doc = "Field `START` writer - backup start signal"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TO_MEM` reader - backup direction(reg to mem / mem to reg)"]
pub type TO_MEM_R = crate::BitReader;
#[doc = "Field `TO_MEM` writer - backup direction(reg to mem / mem to reg)"]
pub type TO_MEM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINK_SEL` reader - Link select"]
pub type LINK_SEL_R = crate::FieldReader;
#[doc = "Field `LINK_SEL` writer - Link select"]
pub type LINK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SW_RETRY_EN` reader - sw_retry_en"]
pub type SW_RETRY_EN_R = crate::BitReader;
#[doc = "Field `SW_RETRY_EN` writer - sw_retry_en"]
pub type SW_RETRY_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAUDMA_BUSY` reader - regdma_busy"]
pub type PAUDMA_BUSY_R = crate::BitReader;
#[doc = "Field `FIX_PRI_EN` reader - fix_pri_en"]
pub type FIX_PRI_EN_R = crate::BitReader;
#[doc = "Field `FIX_PRI_EN` writer - fix_pri_en"]
pub type FIX_PRI_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - backup direction(reg to mem / mem to reg)"]
    #[inline(always)]
    pub fn to_mem(&self) -> TO_MEM_R {
        TO_MEM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:8 - Link select"]
    #[inline(always)]
    pub fn link_sel(&self) -> LINK_SEL_R {
        LINK_SEL_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - sw_retry_en"]
    #[inline(always)]
    pub fn sw_retry_en(&self) -> SW_RETRY_EN_R {
        SW_RETRY_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - regdma_busy"]
    #[inline(always)]
    pub fn paudma_busy(&self) -> PAUDMA_BUSY_R {
        PAUDMA_BUSY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - fix_pri_en"]
    #[inline(always)]
    pub fn fix_pri_en(&self) -> FIX_PRI_EN_R {
        FIX_PRI_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGDMA_CONF")
            .field("to_mem", &self.to_mem())
            .field("link_sel", &self.link_sel())
            .field("sw_retry_en", &self.sw_retry_en())
            .field("paudma_busy", &self.paudma_busy())
            .field("fix_pri_en", &self.fix_pri_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - backup start signal"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<'_, REGDMA_CONF_SPEC> {
        START_W::new(self, 3)
    }
    #[doc = "Bit 4 - backup direction(reg to mem / mem to reg)"]
    #[inline(always)]
    pub fn to_mem(&mut self) -> TO_MEM_W<'_, REGDMA_CONF_SPEC> {
        TO_MEM_W::new(self, 4)
    }
    #[doc = "Bits 5:8 - Link select"]
    #[inline(always)]
    pub fn link_sel(&mut self) -> LINK_SEL_W<'_, REGDMA_CONF_SPEC> {
        LINK_SEL_W::new(self, 5)
    }
    #[doc = "Bit 9 - sw_retry_en"]
    #[inline(always)]
    pub fn sw_retry_en(&mut self) -> SW_RETRY_EN_W<'_, REGDMA_CONF_SPEC> {
        SW_RETRY_EN_W::new(self, 9)
    }
    #[doc = "Bit 11 - fix_pri_en"]
    #[inline(always)]
    pub fn fix_pri_en(&mut self) -> FIX_PRI_EN_W<'_, REGDMA_CONF_SPEC> {
        FIX_PRI_EN_W::new(self, 11)
    }
}
#[doc = "Peri backup control register\n\nYou can [`read`](crate::Reg::read) this register and get [`regdma_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regdma_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGDMA_CONF_SPEC;
impl crate::RegisterSpec for REGDMA_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regdma_conf::R`](R) reader structure"]
impl crate::Readable for REGDMA_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`regdma_conf::W`](W) writer structure"]
impl crate::Writable for REGDMA_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGDMA_CONF to value 0"]
impl crate::Resettable for REGDMA_CONF_SPEC {}
