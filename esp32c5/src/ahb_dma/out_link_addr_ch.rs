#[doc = "Register `OUT_LINK_ADDR_CH%s` reader"]
pub type R = crate::R<OUT_LINK_ADDR_CH_SPEC>;
#[doc = "Register `OUT_LINK_ADDR_CH%s` writer"]
pub type W = crate::W<OUT_LINK_ADDR_CH_SPEC>;
#[doc = "Field `OUTLINK_ADDR_CH` reader - Configures the 32 bits of the first receive descriptor's address."]
pub type OUTLINK_ADDR_CH_R = crate::FieldReader<u32>;
#[doc = "Field `OUTLINK_ADDR_CH` writer - Configures the 32 bits of the first receive descriptor's address."]
pub type OUTLINK_ADDR_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures the 32 bits of the first receive descriptor's address."]
    #[inline(always)]
    pub fn outlink_addr_ch(&self) -> OUTLINK_ADDR_CH_R {
        OUTLINK_ADDR_CH_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_LINK_ADDR_CH")
            .field("outlink_addr_ch", &self.outlink_addr_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures the 32 bits of the first receive descriptor's address."]
    #[inline(always)]
    pub fn outlink_addr_ch(&mut self) -> OUTLINK_ADDR_CH_W<OUT_LINK_ADDR_CH_SPEC> {
        OUTLINK_ADDR_CH_W::new(self, 0)
    }
}
#[doc = "Link list descriptor address configuration of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_link_addr_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_link_addr_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_LINK_ADDR_CH_SPEC;
impl crate::RegisterSpec for OUT_LINK_ADDR_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_link_addr_ch::R`](R) reader structure"]
impl crate::Readable for OUT_LINK_ADDR_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_link_addr_ch::W`](W) writer structure"]
impl crate::Writable for OUT_LINK_ADDR_CH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_LINK_ADDR_CH%s to value 0"]
impl crate::Resettable for OUT_LINK_ADDR_CH_SPEC {
    const RESET_VALUE: u32 = 0;
}
