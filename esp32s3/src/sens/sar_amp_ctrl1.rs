#[doc = "Register `SAR_AMP_CTRL1` reader"]
pub type R = crate::R<SAR_AMP_CTRL1_SPEC>;
#[doc = "Register `SAR_AMP_CTRL1` writer"]
pub type W = crate::W<SAR_AMP_CTRL1_SPEC>;
#[doc = "Field `SAR_AMP_WAIT1` reader - no public"]
pub type SAR_AMP_WAIT1_R = crate::FieldReader<u16>;
#[doc = "Field `SAR_AMP_WAIT1` writer - no public"]
pub type SAR_AMP_WAIT1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SAR_AMP_WAIT2` reader - no public"]
pub type SAR_AMP_WAIT2_R = crate::FieldReader<u16>;
#[doc = "Field `SAR_AMP_WAIT2` writer - no public"]
pub type SAR_AMP_WAIT2_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - no public"]
    #[inline(always)]
    pub fn sar_amp_wait1(&self) -> SAR_AMP_WAIT1_R {
        SAR_AMP_WAIT1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - no public"]
    #[inline(always)]
    pub fn sar_amp_wait2(&self) -> SAR_AMP_WAIT2_R {
        SAR_AMP_WAIT2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_AMP_CTRL1")
            .field("sar_amp_wait1", &self.sar_amp_wait1().bits())
            .field("sar_amp_wait2", &self.sar_amp_wait2().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_AMP_CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - no public"]
    #[inline(always)]
    #[must_use]
    pub fn sar_amp_wait1(&mut self) -> SAR_AMP_WAIT1_W<SAR_AMP_CTRL1_SPEC> {
        SAR_AMP_WAIT1_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - no public"]
    #[inline(always)]
    #[must_use]
    pub fn sar_amp_wait2(&mut self) -> SAR_AMP_WAIT2_W<SAR_AMP_CTRL1_SPEC> {
        SAR_AMP_WAIT2_W::new(self, 16)
    }
}
#[doc = "no public\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_amp_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_amp_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_AMP_CTRL1_SPEC;
impl crate::RegisterSpec for SAR_AMP_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_amp_ctrl1::R`](R) reader structure"]
impl crate::Readable for SAR_AMP_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_amp_ctrl1::W`](W) writer structure"]
impl crate::Writable for SAR_AMP_CTRL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_AMP_CTRL1 to value 0x000a_000a"]
impl crate::Resettable for SAR_AMP_CTRL1_SPEC {
    const RESET_VALUE: u32 = 0x000a_000a;
}
