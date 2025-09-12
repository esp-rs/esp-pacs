#[doc = "Register `OUT_LINK_ADDR_CH1` reader"]
pub type R = crate::R<OUT_LINK_ADDR_CH1_SPEC>;
#[doc = "Register `OUT_LINK_ADDR_CH1` writer"]
pub type W = crate::W<OUT_LINK_ADDR_CH1_SPEC>;
#[doc = "Field `OUTLINK_ADDR_CH1` reader - Configures the 32 bits of the first receive descriptor's address."]
pub type OUTLINK_ADDR_CH1_R = crate::FieldReader<u32>;
#[doc = "Field `OUTLINK_ADDR_CH1` writer - Configures the 32 bits of the first receive descriptor's address."]
pub type OUTLINK_ADDR_CH1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures the 32 bits of the first receive descriptor's address."]
    #[inline(always)]
    pub fn outlink_addr_ch1(&self) -> OUTLINK_ADDR_CH1_R {
        OUTLINK_ADDR_CH1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_LINK_ADDR_CH1")
            .field("outlink_addr_ch1", &self.outlink_addr_ch1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures the 32 bits of the first receive descriptor's address."]
    #[inline(always)]
    pub fn outlink_addr_ch1(&mut self) -> OUTLINK_ADDR_CH1_W<'_, OUT_LINK_ADDR_CH1_SPEC> {
        OUTLINK_ADDR_CH1_W::new(self, 0)
    }
}
#[doc = "Link list descriptor address configuration of TX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`out_link_addr_ch1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_link_addr_ch1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_LINK_ADDR_CH1_SPEC;
impl crate::RegisterSpec for OUT_LINK_ADDR_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_link_addr_ch1::R`](R) reader structure"]
impl crate::Readable for OUT_LINK_ADDR_CH1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_link_addr_ch1::W`](W) writer structure"]
impl crate::Writable for OUT_LINK_ADDR_CH1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_LINK_ADDR_CH1 to value 0"]
impl crate::Resettable for OUT_LINK_ADDR_CH1_SPEC {}
