#[doc = "Register `PARL_IO_TX_INTR_MAP` reader"]
pub type R = crate::R<PARL_IO_TX_INTR_MAP_SPEC>;
#[doc = "Register `PARL_IO_TX_INTR_MAP` writer"]
pub type W = crate::W<PARL_IO_TX_INTR_MAP_SPEC>;
#[doc = "Field `PARL_IO_TX_INTR_MAP` reader - CORE0_PARL_IO_TX_INTR mapping register"]
pub type PARL_IO_TX_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `PARL_IO_TX_INTR_MAP` writer - CORE0_PARL_IO_TX_INTR mapping register"]
pub type PARL_IO_TX_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - CORE0_PARL_IO_TX_INTR mapping register"]
    #[inline(always)]
    pub fn parl_io_tx_intr_map(&self) -> PARL_IO_TX_INTR_MAP_R {
        PARL_IO_TX_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PARL_IO_TX_INTR_MAP")
            .field("parl_io_tx_intr_map", &self.parl_io_tx_intr_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - CORE0_PARL_IO_TX_INTR mapping register"]
    #[inline(always)]
    #[must_use]
    pub fn parl_io_tx_intr_map(&mut self) -> PARL_IO_TX_INTR_MAP_W<PARL_IO_TX_INTR_MAP_SPEC> {
        PARL_IO_TX_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`parl_io_tx_intr_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`parl_io_tx_intr_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PARL_IO_TX_INTR_MAP_SPEC;
impl crate::RegisterSpec for PARL_IO_TX_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`parl_io_tx_intr_map::R`](R) reader structure"]
impl crate::Readable for PARL_IO_TX_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`parl_io_tx_intr_map::W`](W) writer structure"]
impl crate::Writable for PARL_IO_TX_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PARL_IO_TX_INTR_MAP to value 0"]
impl crate::Resettable for PARL_IO_TX_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
