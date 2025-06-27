#[doc = "Register `IN_SRAM_SIZE_CH%s` reader"]
pub type R = crate::R<IN_SRAM_SIZE_CH_SPEC>;
#[doc = "Register `IN_SRAM_SIZE_CH%s` writer"]
pub type W = crate::W<IN_SRAM_SIZE_CH_SPEC>;
#[doc = "Field `IN_SIZE` reader - This register is used to configure the size of L2 Tx FIFO for Rx channel 0. 0:16 bytes. 1:24 bytes. 2:32 bytes. 3: 40 bytes. 4: 48 bytes. 5:56 bytes. 6: 64 bytes. 7: 72 bytes. 8: 80 bytes."]
pub type IN_SIZE_R = crate::FieldReader;
#[doc = "Field `IN_SIZE` writer - This register is used to configure the size of L2 Tx FIFO for Rx channel 0. 0:16 bytes. 1:24 bytes. 2:32 bytes. 3: 40 bytes. 4: 48 bytes. 5:56 bytes. 6: 64 bytes. 7: 72 bytes. 8: 80 bytes."]
pub type IN_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - This register is used to configure the size of L2 Tx FIFO for Rx channel 0. 0:16 bytes. 1:24 bytes. 2:32 bytes. 3: 40 bytes. 4: 48 bytes. 5:56 bytes. 6: 64 bytes. 7: 72 bytes. 8: 80 bytes."]
    #[inline(always)]
    pub fn in_size(&self) -> IN_SIZE_R {
        IN_SIZE_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_SRAM_SIZE_CH")
            .field("in_size", &self.in_size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - This register is used to configure the size of L2 Tx FIFO for Rx channel 0. 0:16 bytes. 1:24 bytes. 2:32 bytes. 3: 40 bytes. 4: 48 bytes. 5:56 bytes. 6: 64 bytes. 7: 72 bytes. 8: 80 bytes."]
    #[inline(always)]
    pub fn in_size(&mut self) -> IN_SIZE_W<IN_SRAM_SIZE_CH_SPEC> {
        IN_SIZE_W::new(self, 0)
    }
}
#[doc = "Receive L2 FIFO depth of Rx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_sram_size_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_sram_size_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_SRAM_SIZE_CH_SPEC;
impl crate::RegisterSpec for IN_SRAM_SIZE_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_sram_size_ch::R`](R) reader structure"]
impl crate::Readable for IN_SRAM_SIZE_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_sram_size_ch::W`](W) writer structure"]
impl crate::Writable for IN_SRAM_SIZE_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_SRAM_SIZE_CH%s to value 0x0e"]
impl crate::Resettable for IN_SRAM_SIZE_CH_SPEC {
    const RESET_VALUE: u32 = 0x0e;
}
