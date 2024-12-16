#[doc = "Register `AMP_CTRL1` reader"]
pub type R = crate::R<AMP_CTRL1_SPEC>;
#[doc = "Register `AMP_CTRL1` writer"]
pub type W = crate::W<AMP_CTRL1_SPEC>;
#[doc = "Field `SAR_AMP_WAIT1` reader - N/A"]
pub type SAR_AMP_WAIT1_R = crate::FieldReader<u16>;
#[doc = "Field `SAR_AMP_WAIT1` writer - N/A"]
pub type SAR_AMP_WAIT1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SAR_AMP_WAIT2` reader - N/A"]
pub type SAR_AMP_WAIT2_R = crate::FieldReader<u16>;
#[doc = "Field `SAR_AMP_WAIT2` writer - N/A"]
pub type SAR_AMP_WAIT2_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn sar_amp_wait1(&self) -> SAR_AMP_WAIT1_R {
        SAR_AMP_WAIT1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - N/A"]
    #[inline(always)]
    pub fn sar_amp_wait2(&self) -> SAR_AMP_WAIT2_R {
        SAR_AMP_WAIT2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AMP_CTRL1")
            .field("sar_amp_wait1", &self.sar_amp_wait1())
            .field("sar_amp_wait2", &self.sar_amp_wait2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn sar_amp_wait1(&mut self) -> SAR_AMP_WAIT1_W<AMP_CTRL1_SPEC> {
        SAR_AMP_WAIT1_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - N/A"]
    #[inline(always)]
    pub fn sar_amp_wait2(&mut self) -> SAR_AMP_WAIT2_W<AMP_CTRL1_SPEC> {
        SAR_AMP_WAIT2_W::new(self, 16)
    }
}
#[doc = "N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`amp_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amp_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AMP_CTRL1_SPEC;
impl crate::RegisterSpec for AMP_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`amp_ctrl1::R`](R) reader structure"]
impl crate::Readable for AMP_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`amp_ctrl1::W`](W) writer structure"]
impl crate::Writable for AMP_CTRL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AMP_CTRL1 to value 0x000a_000a"]
impl crate::Resettable for AMP_CTRL1_SPEC {
    const RESET_VALUE: u32 = 0x000a_000a;
}
