#[doc = "Register `RST_EVENT_BYPASS` reader"]
pub type R = crate::R<RST_EVENT_BYPASS_SPEC>;
#[doc = "Register `RST_EVENT_BYPASS` writer"]
pub type W = crate::W<RST_EVENT_BYPASS_SPEC>;
#[doc = "Field `ICMAPB` reader - Set 1 to bypass none-power_on reset source"]
pub type ICMAPB_R = crate::BitReader;
#[doc = "Field `ICMAPB` writer - Set 1 to bypass none-power_on reset source"]
pub type ICMAPB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICMSYS` reader - Set 1 to bypass none-power_on reset source"]
pub type ICMSYS_R = crate::BitReader;
#[doc = "Field `ICMSYS` writer - Set 1 to bypass none-power_on reset source"]
pub type ICMSYS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICMCPU` reader - Set 1 to bypass none-power_on reset source"]
pub type ICMCPU_R = crate::BitReader;
#[doc = "Field `ICMCPU` writer - Set 1 to bypass none-power_on reset source"]
pub type ICMCPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKRST` reader - Set 1 to bypass none-power_on reset source"]
pub type CLKRST_R = crate::BitReader;
#[doc = "Field `CLKRST` writer - Set 1 to bypass none-power_on reset source"]
pub type CLKRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKRST_REG` reader - Set 1 to bypass none-power_on reset source"]
pub type CLKRST_REG_R = crate::BitReader;
#[doc = "Field `CLKRST_REG` writer - Set 1 to bypass none-power_on reset source"]
pub type CLKRST_REG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOMUX` reader - Set 1 to bypass none-power_on reset source"]
pub type IOMUX_R = crate::BitReader;
#[doc = "Field `IOMUX` writer - Set 1 to bypass none-power_on reset source"]
pub type IOMUX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_BYPASS_LOCK` reader - Set 1 to lock reset event bypass configuration"]
pub type RST_BYPASS_LOCK_R = crate::BitReader;
#[doc = "Field `RST_BYPASS_LOCK` writer - Set 1 to lock reset event bypass configuration"]
pub type RST_BYPASS_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to bypass none-power_on reset source"]
    #[inline(always)]
    pub fn icmapb(&self) -> ICMAPB_R {
        ICMAPB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to bypass none-power_on reset source"]
    #[inline(always)]
    pub fn icmsys(&self) -> ICMSYS_R {
        ICMSYS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set 1 to bypass none-power_on reset source"]
    #[inline(always)]
    pub fn icmcpu(&self) -> ICMCPU_R {
        ICMCPU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set 1 to bypass none-power_on reset source"]
    #[inline(always)]
    pub fn clkrst(&self) -> CLKRST_R {
        CLKRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set 1 to bypass none-power_on reset source"]
    #[inline(always)]
    pub fn clkrst_reg(&self) -> CLKRST_REG_R {
        CLKRST_REG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set 1 to bypass none-power_on reset source"]
    #[inline(always)]
    pub fn iomux(&self) -> IOMUX_R {
        IOMUX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set 1 to lock reset event bypass configuration"]
    #[inline(always)]
    pub fn rst_bypass_lock(&self) -> RST_BYPASS_LOCK_R {
        RST_BYPASS_LOCK_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RST_EVENT_BYPASS")
            .field("icmapb", &self.icmapb())
            .field("icmsys", &self.icmsys())
            .field("icmcpu", &self.icmcpu())
            .field("clkrst", &self.clkrst())
            .field("clkrst_reg", &self.clkrst_reg())
            .field("iomux", &self.iomux())
            .field("rst_bypass_lock", &self.rst_bypass_lock())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to bypass none-power_on reset source"]
    #[inline(always)]
    pub fn icmapb(&mut self) -> ICMAPB_W<'_, RST_EVENT_BYPASS_SPEC> {
        ICMAPB_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 1 to bypass none-power_on reset source"]
    #[inline(always)]
    pub fn icmsys(&mut self) -> ICMSYS_W<'_, RST_EVENT_BYPASS_SPEC> {
        ICMSYS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set 1 to bypass none-power_on reset source"]
    #[inline(always)]
    pub fn icmcpu(&mut self) -> ICMCPU_W<'_, RST_EVENT_BYPASS_SPEC> {
        ICMCPU_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set 1 to bypass none-power_on reset source"]
    #[inline(always)]
    pub fn clkrst(&mut self) -> CLKRST_W<'_, RST_EVENT_BYPASS_SPEC> {
        CLKRST_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set 1 to bypass none-power_on reset source"]
    #[inline(always)]
    pub fn clkrst_reg(&mut self) -> CLKRST_REG_W<'_, RST_EVENT_BYPASS_SPEC> {
        CLKRST_REG_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set 1 to bypass none-power_on reset source"]
    #[inline(always)]
    pub fn iomux(&mut self) -> IOMUX_W<'_, RST_EVENT_BYPASS_SPEC> {
        IOMUX_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set 1 to lock reset event bypass configuration"]
    #[inline(always)]
    pub fn rst_bypass_lock(&mut self) -> RST_BYPASS_LOCK_W<'_, RST_EVENT_BYPASS_SPEC> {
        RST_BYPASS_LOCK_W::new(self, 6)
    }
}
#[doc = "bus clock gating bypass configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_event_bypass::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_event_bypass::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RST_EVENT_BYPASS_SPEC;
impl crate::RegisterSpec for RST_EVENT_BYPASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst_event_bypass::R`](R) reader structure"]
impl crate::Readable for RST_EVENT_BYPASS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rst_event_bypass::W`](W) writer structure"]
impl crate::Writable for RST_EVENT_BYPASS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RST_EVENT_BYPASS to value 0x08"]
impl crate::Resettable for RST_EVENT_BYPASS_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
