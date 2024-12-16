#[doc = "Register `RX_CTRL` reader"]
pub type R = crate::R<RX_CTRL_SPEC>;
#[doc = "Register `RX_CTRL` writer"]
pub type W = crate::W<RX_CTRL_SPEC>;
#[doc = "Field `RX_DESCR_RELOAD` reader - Instruct the hardware to reload the RX descriptors"]
pub type RX_DESCR_RELOAD_R = crate::BitReader;
#[doc = "Field `RX_DESCR_RELOAD` writer - Instruct the hardware to reload the RX descriptors"]
pub type RX_DESCR_RELOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_ENABLE` reader - Enable frame reception"]
pub type RX_ENABLE_R = crate::BitReader;
#[doc = "Field `RX_ENABLE` writer - Enable frame reception"]
pub type RX_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Instruct the hardware to reload the RX descriptors"]
    #[inline(always)]
    pub fn rx_descr_reload(&self) -> RX_DESCR_RELOAD_R {
        RX_DESCR_RELOAD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 31 - Enable frame reception"]
    #[inline(always)]
    pub fn rx_enable(&self) -> RX_ENABLE_R {
        RX_ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CTRL")
            .field("rx_descr_reload", &self.rx_descr_reload())
            .field("rx_enable", &self.rx_enable())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Instruct the hardware to reload the RX descriptors"]
    #[inline(always)]
    pub fn rx_descr_reload(&mut self) -> RX_DESCR_RELOAD_W<RX_CTRL_SPEC> {
        RX_DESCR_RELOAD_W::new(self, 0)
    }
    #[doc = "Bit 31 - Enable frame reception"]
    #[inline(always)]
    pub fn rx_enable(&mut self) -> RX_ENABLE_W<RX_CTRL_SPEC> {
        RX_ENABLE_W::new(self, 31)
    }
}
#[doc = "Controls the reception of frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_CTRL_SPEC;
impl crate::RegisterSpec for RX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_ctrl::R`](R) reader structure"]
impl crate::Readable for RX_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_ctrl::W`](W) writer structure"]
impl crate::Writable for RX_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_CTRL to value 0"]
impl crate::Resettable for RX_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
