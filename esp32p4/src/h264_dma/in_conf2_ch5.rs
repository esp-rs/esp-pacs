#[doc = "Register `IN_CONF2_CH5` reader"]
pub type R = crate::R<IN_CONF2_CH5_SPEC>;
#[doc = "Register `IN_CONF2_CH5` writer"]
pub type W = crate::W<IN_CONF2_CH5_SPEC>;
#[doc = "Field `BLOCK_ROW_LENGTH_12LINE_CH5` reader - The number of bytes contained in a row block 12line in RX channel 5"]
pub type BLOCK_ROW_LENGTH_12LINE_CH5_R = crate::FieldReader<u16>;
#[doc = "Field `BLOCK_ROW_LENGTH_12LINE_CH5` writer - The number of bytes contained in a row block 12line in RX channel 5"]
pub type BLOCK_ROW_LENGTH_12LINE_CH5_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BLOCK_ROW_LENGTH_4LINE_CH5` reader - The number of bytes contained in a row block 4line in RX channel 5"]
pub type BLOCK_ROW_LENGTH_4LINE_CH5_R = crate::FieldReader<u16>;
#[doc = "Field `BLOCK_ROW_LENGTH_4LINE_CH5` writer - The number of bytes contained in a row block 4line in RX channel 5"]
pub type BLOCK_ROW_LENGTH_4LINE_CH5_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The number of bytes contained in a row block 12line in RX channel 5"]
    #[inline(always)]
    pub fn block_row_length_12line_ch5(&self) -> BLOCK_ROW_LENGTH_12LINE_CH5_R {
        BLOCK_ROW_LENGTH_12LINE_CH5_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - The number of bytes contained in a row block 4line in RX channel 5"]
    #[inline(always)]
    pub fn block_row_length_4line_ch5(&self) -> BLOCK_ROW_LENGTH_4LINE_CH5_R {
        BLOCK_ROW_LENGTH_4LINE_CH5_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_CONF2_CH5")
            .field(
                "block_row_length_12line_ch5",
                &format_args!("{}", self.block_row_length_12line_ch5().bits()),
            )
            .field(
                "block_row_length_4line_ch5",
                &format_args!("{}", self.block_row_length_4line_ch5().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_CONF2_CH5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - The number of bytes contained in a row block 12line in RX channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn block_row_length_12line_ch5(
        &mut self,
    ) -> BLOCK_ROW_LENGTH_12LINE_CH5_W<IN_CONF2_CH5_SPEC> {
        BLOCK_ROW_LENGTH_12LINE_CH5_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - The number of bytes contained in a row block 4line in RX channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn block_row_length_4line_ch5(
        &mut self,
    ) -> BLOCK_ROW_LENGTH_4LINE_CH5_W<IN_CONF2_CH5_SPEC> {
        BLOCK_ROW_LENGTH_4LINE_CH5_W::new(self, 16)
    }
}
#[doc = "RX CH5 config2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf2_ch5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf2_ch5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_CONF2_CH5_SPEC;
impl crate::RegisterSpec for IN_CONF2_CH5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_conf2_ch5::R`](R) reader structure"]
impl crate::Readable for IN_CONF2_CH5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_conf2_ch5::W`](W) writer structure"]
impl crate::Writable for IN_CONF2_CH5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IN_CONF2_CH5 to value 0x3c00_7800"]
impl crate::Resettable for IN_CONF2_CH5_SPEC {
    const RESET_VALUE: u32 = 0x3c00_7800;
}
