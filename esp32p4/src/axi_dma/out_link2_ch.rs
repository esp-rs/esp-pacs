#[doc = "Register `OUT_LINK2_CH%s` reader"]
pub type R = crate::R<OUT_LINK2_CH_SPEC>;
#[doc = "Register `OUT_LINK2_CH%s` writer"]
pub type W = crate::W<OUT_LINK2_CH_SPEC>;
#[doc = "Field `OUTLINK_ADDR_CH` reader - This register stores the 32 least significant bits of the first outlink descriptor's address."]
pub type OUTLINK_ADDR_CH_R = crate::FieldReader<u32>;
#[doc = "Field `OUTLINK_ADDR_CH` writer - This register stores the 32 least significant bits of the first outlink descriptor's address."]
pub type OUTLINK_ADDR_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the 32 least significant bits of the first outlink descriptor's address."]
    #[inline(always)]
    pub fn outlink_addr_ch(&self) -> OUTLINK_ADDR_CH_R {
        OUTLINK_ADDR_CH_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_LINK2_CH")
            .field(
                "outlink_addr_ch",
                &format_args!("{}", self.outlink_addr_ch().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_LINK2_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register stores the 32 least significant bits of the first outlink descriptor's address."]
    #[inline(always)]
    #[must_use]
    pub fn outlink_addr_ch(&mut self) -> OUTLINK_ADDR_CH_W<OUT_LINK2_CH_SPEC> {
        OUTLINK_ADDR_CH_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Link descriptor configure and control register of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_link2_ch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_link2_ch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_LINK2_CH_SPEC;
impl crate::RegisterSpec for OUT_LINK2_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_link2_ch::R`](R) reader structure"]
impl crate::Readable for OUT_LINK2_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_link2_ch::W`](W) writer structure"]
impl crate::Writable for OUT_LINK2_CH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_LINK2_CH%s to value 0"]
impl crate::Resettable for OUT_LINK2_CH_SPEC {
    const RESET_VALUE: u32 = 0;
}
