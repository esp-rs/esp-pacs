#[doc = "Register `AXI_ID0` reader"]
pub type R = crate::R<AXI_ID0_SPEC>;
#[doc = "Register `AXI_ID0` writer"]
pub type W = crate::W<AXI_ID0_SPEC>;
#[doc = "Field `CH1_AXI_READ_ID_SUFFIX` reader - NA"]
pub type CH1_AXI_READ_ID_SUFFIX_R = crate::BitReader;
#[doc = "Field `CH1_AXI_READ_ID_SUFFIX` writer - NA"]
pub type CH1_AXI_READ_ID_SUFFIX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_AXI_WRITE_ID_SUFFIX` reader - NA"]
pub type CH1_AXI_WRITE_ID_SUFFIX_R = crate::BitReader;
#[doc = "Field `CH1_AXI_WRITE_ID_SUFFIX` writer - NA"]
pub type CH1_AXI_WRITE_ID_SUFFIX_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn ch1_axi_read_id_suffix(&self) -> CH1_AXI_READ_ID_SUFFIX_R {
        CH1_AXI_READ_ID_SUFFIX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn ch1_axi_write_id_suffix(&self) -> CH1_AXI_WRITE_ID_SUFFIX_R {
        CH1_AXI_WRITE_ID_SUFFIX_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AXI_ID0")
            .field("ch1_axi_read_id_suffix", &self.ch1_axi_read_id_suffix())
            .field("ch1_axi_write_id_suffix", &self.ch1_axi_write_id_suffix())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn ch1_axi_read_id_suffix(&mut self) -> CH1_AXI_READ_ID_SUFFIX_W<'_, AXI_ID0_SPEC> {
        CH1_AXI_READ_ID_SUFFIX_W::new(self, 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn ch1_axi_write_id_suffix(&mut self) -> CH1_AXI_WRITE_ID_SUFFIX_W<'_, AXI_ID0_SPEC> {
        CH1_AXI_WRITE_ID_SUFFIX_W::new(self, 16)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_id0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_id0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXI_ID0_SPEC;
impl crate::RegisterSpec for AXI_ID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_id0::R`](R) reader structure"]
impl crate::Readable for AXI_ID0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`axi_id0::W`](W) writer structure"]
impl crate::Writable for AXI_ID0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AXI_ID0 to value 0"]
impl crate::Resettable for AXI_ID0_SPEC {}
