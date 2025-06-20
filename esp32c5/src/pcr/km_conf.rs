#[doc = "Register `KM_CONF` reader"]
pub type R = crate::R<KM_CONF_SPEC>;
#[doc = "Register `KM_CONF` writer"]
pub type W = crate::W<KM_CONF_SPEC>;
#[doc = "Field `KM_CLK_EN` reader - Set 1 to enable km clock"]
pub type KM_CLK_EN_R = crate::BitReader;
#[doc = "Field `KM_CLK_EN` writer - Set 1 to enable km clock"]
pub type KM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KM_RST_EN` reader - Set 0 to reset km module"]
pub type KM_RST_EN_R = crate::BitReader;
#[doc = "Field `KM_RST_EN` writer - Set 0 to reset km module"]
pub type KM_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KM_READY` reader - Query this field after reset km module"]
pub type KM_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set 1 to enable km clock"]
    #[inline(always)]
    pub fn km_clk_en(&self) -> KM_CLK_EN_R {
        KM_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset km module"]
    #[inline(always)]
    pub fn km_rst_en(&self) -> KM_RST_EN_R {
        KM_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Query this field after reset km module"]
    #[inline(always)]
    pub fn km_ready(&self) -> KM_READY_R {
        KM_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KM_CONF")
            .field("km_clk_en", &self.km_clk_en())
            .field("km_rst_en", &self.km_rst_en())
            .field("km_ready", &self.km_ready())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable km clock"]
    #[inline(always)]
    pub fn km_clk_en(&mut self) -> KM_CLK_EN_W<KM_CONF_SPEC> {
        KM_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 0 to reset km module"]
    #[inline(always)]
    pub fn km_rst_en(&mut self) -> KM_RST_EN_W<KM_CONF_SPEC> {
        KM_RST_EN_W::new(self, 1)
    }
}
#[doc = "Key Manager configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`km_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`km_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KM_CONF_SPEC;
impl crate::RegisterSpec for KM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`km_conf::R`](R) reader structure"]
impl crate::Readable for KM_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`km_conf::W`](W) writer structure"]
impl crate::Writable for KM_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KM_CONF to value 0x04"]
impl crate::Resettable for KM_CONF_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
