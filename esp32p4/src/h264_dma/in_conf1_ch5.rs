#[doc = "Register `IN_CONF1_CH5` reader"]
pub type R = crate::R<IN_CONF1_CH5_SPEC>;
#[doc = "Register `IN_CONF1_CH5` writer"]
pub type W = crate::W<IN_CONF1_CH5_SPEC>;
#[doc = "Field `BLOCK_START_ADDR_CH5` reader - RX Channel 5 destination start address"]
pub type BLOCK_START_ADDR_CH5_R = crate::FieldReader<u32>;
#[doc = "Field `BLOCK_START_ADDR_CH5` writer - RX Channel 5 destination start address"]
pub type BLOCK_START_ADDR_CH5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RX Channel 5 destination start address"]
    #[inline(always)]
    pub fn block_start_addr_ch5(&self) -> BLOCK_START_ADDR_CH5_R {
        BLOCK_START_ADDR_CH5_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_CONF1_CH5")
            .field(
                "block_start_addr_ch5",
                &format_args!("{}", self.block_start_addr_ch5().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_CONF1_CH5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - RX Channel 5 destination start address"]
    #[inline(always)]
    #[must_use]
    pub fn block_start_addr_ch5(&mut self) -> BLOCK_START_ADDR_CH5_W<IN_CONF1_CH5_SPEC> {
        BLOCK_START_ADDR_CH5_W::new(self, 0)
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
#[doc = "RX CH5 config1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf1_ch5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf1_ch5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_CONF1_CH5_SPEC;
impl crate::RegisterSpec for IN_CONF1_CH5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_conf1_ch5::R`](R) reader structure"]
impl crate::Readable for IN_CONF1_CH5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_conf1_ch5::W`](W) writer structure"]
impl crate::Writable for IN_CONF1_CH5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IN_CONF1_CH5 to value 0"]
impl crate::Resettable for IN_CONF1_CH5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
