#[doc = "Register `CH2_AXI_ID0` reader"]
pub type R = crate::R<CH2_AXI_ID0_SPEC>;
#[doc = "Register `CH2_AXI_ID0` writer"]
pub type W = crate::W<CH2_AXI_ID0_SPEC>;
#[doc = "Field `CH2_AXI_READ_ID_SUFFIX` reader - NA"]
pub type CH2_AXI_READ_ID_SUFFIX_R = crate::BitReader;
#[doc = "Field `CH2_AXI_READ_ID_SUFFIX` writer - NA"]
pub type CH2_AXI_READ_ID_SUFFIX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_AXI_WRITE_ID_SUFFIX` reader - NA"]
pub type CH2_AXI_WRITE_ID_SUFFIX_R = crate::BitReader;
#[doc = "Field `CH2_AXI_WRITE_ID_SUFFIX` writer - NA"]
pub type CH2_AXI_WRITE_ID_SUFFIX_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn ch2_axi_read_id_suffix(&self) -> CH2_AXI_READ_ID_SUFFIX_R {
        CH2_AXI_READ_ID_SUFFIX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn ch2_axi_write_id_suffix(&self) -> CH2_AXI_WRITE_ID_SUFFIX_R {
        CH2_AXI_WRITE_ID_SUFFIX_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH2_AXI_ID0")
            .field(
                "ch2_axi_read_id_suffix",
                &format_args!("{}", self.ch2_axi_read_id_suffix().bit()),
            )
            .field(
                "ch2_axi_write_id_suffix",
                &format_args!("{}", self.ch2_axi_write_id_suffix().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH2_AXI_ID0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_axi_read_id_suffix(&mut self) -> CH2_AXI_READ_ID_SUFFIX_W<CH2_AXI_ID0_SPEC> {
        CH2_AXI_READ_ID_SUFFIX_W::new(self, 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_axi_write_id_suffix(&mut self) -> CH2_AXI_WRITE_ID_SUFFIX_W<CH2_AXI_ID0_SPEC> {
        CH2_AXI_WRITE_ID_SUFFIX_W::new(self, 16)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_axi_id0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_axi_id0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH2_AXI_ID0_SPEC;
impl crate::RegisterSpec for CH2_AXI_ID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2_axi_id0::R`](R) reader structure"]
impl crate::Readable for CH2_AXI_ID0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch2_axi_id0::W`](W) writer structure"]
impl crate::Writable for CH2_AXI_ID0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH2_AXI_ID0 to value 0"]
impl crate::Resettable for CH2_AXI_ID0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
