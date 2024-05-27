#[doc = "Register `UHCI_CONF` reader"]
pub type R = crate::R<UHCI_CONF_SPEC>;
#[doc = "Register `UHCI_CONF` writer"]
pub type W = crate::W<UHCI_CONF_SPEC>;
#[doc = "Field `UHCI_CLK_EN` reader - Set 1 to enable uhci clock"]
pub type UHCI_CLK_EN_R = crate::BitReader;
#[doc = "Field `UHCI_CLK_EN` writer - Set 1 to enable uhci clock"]
pub type UHCI_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UHCI_RST_EN` reader - Set 0 to reset uhci module"]
pub type UHCI_RST_EN_R = crate::BitReader;
#[doc = "Field `UHCI_RST_EN` writer - Set 0 to reset uhci module"]
pub type UHCI_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UHCI_READY` reader - Query this field after reset uhci module"]
pub type UHCI_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set 1 to enable uhci clock"]
    #[inline(always)]
    pub fn uhci_clk_en(&self) -> UHCI_CLK_EN_R {
        UHCI_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset uhci module"]
    #[inline(always)]
    pub fn uhci_rst_en(&self) -> UHCI_RST_EN_R {
        UHCI_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Query this field after reset uhci module"]
    #[inline(always)]
    pub fn uhci_ready(&self) -> UHCI_READY_R {
        UHCI_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UHCI_CONF")
            .field("uhci_clk_en", &self.uhci_clk_en())
            .field("uhci_rst_en", &self.uhci_rst_en())
            .field("uhci_ready", &self.uhci_ready())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable uhci clock"]
    #[inline(always)]
    #[must_use]
    pub fn uhci_clk_en(&mut self) -> UHCI_CLK_EN_W<UHCI_CONF_SPEC> {
        UHCI_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 0 to reset uhci module"]
    #[inline(always)]
    #[must_use]
    pub fn uhci_rst_en(&mut self) -> UHCI_RST_EN_W<UHCI_CONF_SPEC> {
        UHCI_RST_EN_W::new(self, 1)
    }
}
#[doc = "UHCI configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uhci_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uhci_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UHCI_CONF_SPEC;
impl crate::RegisterSpec for UHCI_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhci_conf::R`](R) reader structure"]
impl crate::Readable for UHCI_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uhci_conf::W`](W) writer structure"]
impl crate::Writable for UHCI_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UHCI_CONF to value 0x05"]
impl crate::Resettable for UHCI_CONF_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
