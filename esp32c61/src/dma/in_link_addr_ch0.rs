#[doc = "Register `IN_LINK_ADDR_CH0` reader"]
pub type R = crate::R<IN_LINK_ADDR_CH0_SPEC>;
#[doc = "Register `IN_LINK_ADDR_CH0` writer"]
pub type W = crate::W<IN_LINK_ADDR_CH0_SPEC>;
#[doc = "Field `INLINK_ADDR_CH0` reader - Configures the 32 bits of the first receive descriptor's address"]
pub type INLINK_ADDR_CH0_R = crate::FieldReader<u32>;
#[doc = "Field `INLINK_ADDR_CH0` writer - Configures the 32 bits of the first receive descriptor's address"]
pub type INLINK_ADDR_CH0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures the 32 bits of the first receive descriptor's address"]
    #[inline(always)]
    pub fn inlink_addr_ch0(&self) -> INLINK_ADDR_CH0_R {
        INLINK_ADDR_CH0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_LINK_ADDR_CH0")
            .field("inlink_addr_ch0", &self.inlink_addr_ch0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures the 32 bits of the first receive descriptor's address"]
    #[inline(always)]
    pub fn inlink_addr_ch0(&mut self) -> INLINK_ADDR_CH0_W<'_, IN_LINK_ADDR_CH0_SPEC> {
        INLINK_ADDR_CH0_W::new(self, 0)
    }
}
#[doc = "Link list descriptor address configuration of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_link_addr_ch0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_link_addr_ch0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_LINK_ADDR_CH0_SPEC;
impl crate::RegisterSpec for IN_LINK_ADDR_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_link_addr_ch0::R`](R) reader structure"]
impl crate::Readable for IN_LINK_ADDR_CH0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_link_addr_ch0::W`](W) writer structure"]
impl crate::Writable for IN_LINK_ADDR_CH0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_LINK_ADDR_CH0 to value 0"]
impl crate::Resettable for IN_LINK_ADDR_CH0_SPEC {}
