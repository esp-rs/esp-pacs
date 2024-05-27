#[doc = "Register `CONF_CHAN` reader"]
pub type R = crate::R<CONF_CHAN_SPEC>;
#[doc = "Register `CONF_CHAN` writer"]
pub type W = crate::W<CONF_CHAN_SPEC>;
#[doc = "Field `TX_CHAN_MOD` reader - "]
pub type TX_CHAN_MOD_R = crate::FieldReader;
#[doc = "Field `TX_CHAN_MOD` writer - "]
pub type TX_CHAN_MOD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RX_CHAN_MOD` reader - "]
pub type RX_CHAN_MOD_R = crate::FieldReader;
#[doc = "Field `RX_CHAN_MOD` writer - "]
pub type RX_CHAN_MOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tx_chan_mod(&self) -> TX_CHAN_MOD_R {
        TX_CHAN_MOD_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn rx_chan_mod(&self) -> RX_CHAN_MOD_R {
        RX_CHAN_MOD_R::new(((self.bits >> 3) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_CHAN")
            .field("tx_chan_mod", &self.tx_chan_mod())
            .field("rx_chan_mod", &self.rx_chan_mod())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn tx_chan_mod(&mut self) -> TX_CHAN_MOD_W<CONF_CHAN_SPEC> {
        TX_CHAN_MOD_W::new(self, 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    #[must_use]
    pub fn rx_chan_mod(&mut self) -> RX_CHAN_MOD_W<CONF_CHAN_SPEC> {
        RX_CHAN_MOD_W::new(self, 3)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf_chan::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf_chan::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_CHAN_SPEC;
impl crate::RegisterSpec for CONF_CHAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf_chan::R`](R) reader structure"]
impl crate::Readable for CONF_CHAN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf_chan::W`](W) writer structure"]
impl crate::Writable for CONF_CHAN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF_CHAN to value 0"]
impl crate::Resettable for CONF_CHAN_SPEC {
    const RESET_VALUE: u32 = 0;
}
