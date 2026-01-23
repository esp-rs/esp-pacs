#[doc = "Register `BITSCRAMBLER_PERI_SEL` reader"]
pub type R = crate::R<BITSCRAMBLER_PERI_SEL_SPEC>;
#[doc = "Register `BITSCRAMBLER_PERI_SEL` writer"]
pub type W = crate::W<BITSCRAMBLER_PERI_SEL_SPEC>;
#[doc = "Field `BITSCRAMBLER_RX_SEL` reader - select peri that will be connected to bitscrambler,dir : receive data from bs"]
pub type BITSCRAMBLER_RX_SEL_R = crate::FieldReader;
#[doc = "Field `BITSCRAMBLER_RX_SEL` writer - select peri that will be connected to bitscrambler,dir : receive data from bs"]
pub type BITSCRAMBLER_RX_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BITSCRAMBLER_TX_SEL` reader - select peri that will be connected to bitscrambler,dir : transfer data to peri"]
pub type BITSCRAMBLER_TX_SEL_R = crate::FieldReader;
#[doc = "Field `BITSCRAMBLER_TX_SEL` writer - select peri that will be connected to bitscrambler,dir : transfer data to peri"]
pub type BITSCRAMBLER_TX_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - select peri that will be connected to bitscrambler,dir : receive data from bs"]
    #[inline(always)]
    pub fn bitscrambler_rx_sel(&self) -> BITSCRAMBLER_RX_SEL_R {
        BITSCRAMBLER_RX_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - select peri that will be connected to bitscrambler,dir : transfer data to peri"]
    #[inline(always)]
    pub fn bitscrambler_tx_sel(&self) -> BITSCRAMBLER_TX_SEL_R {
        BITSCRAMBLER_TX_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BITSCRAMBLER_PERI_SEL")
            .field("bitscrambler_rx_sel", &self.bitscrambler_rx_sel())
            .field("bitscrambler_tx_sel", &self.bitscrambler_tx_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - select peri that will be connected to bitscrambler,dir : receive data from bs"]
    #[inline(always)]
    pub fn bitscrambler_rx_sel(&mut self) -> BITSCRAMBLER_RX_SEL_W<'_, BITSCRAMBLER_PERI_SEL_SPEC> {
        BITSCRAMBLER_RX_SEL_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - select peri that will be connected to bitscrambler,dir : transfer data to peri"]
    #[inline(always)]
    pub fn bitscrambler_tx_sel(&mut self) -> BITSCRAMBLER_TX_SEL_W<'_, BITSCRAMBLER_PERI_SEL_SPEC> {
        BITSCRAMBLER_TX_SEL_W::new(self, 4)
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`bitscrambler_peri_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bitscrambler_peri_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BITSCRAMBLER_PERI_SEL_SPEC;
impl crate::RegisterSpec for BITSCRAMBLER_PERI_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bitscrambler_peri_sel::R`](R) reader structure"]
impl crate::Readable for BITSCRAMBLER_PERI_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bitscrambler_peri_sel::W`](W) writer structure"]
impl crate::Writable for BITSCRAMBLER_PERI_SEL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BITSCRAMBLER_PERI_SEL to value 0"]
impl crate::Resettable for BITSCRAMBLER_PERI_SEL_SPEC {}
