#[doc = "Register `BLC_CTRL0` reader"]
pub type R = crate::R<BLC_CTRL0_SPEC>;
#[doc = "Register `BLC_CTRL0` writer"]
pub type W = crate::W<BLC_CTRL0_SPEC>;
#[doc = "Field `BLC_R3_STRETCH` reader - this bit configures the stretch feature of bottom right channel. 0: stretch disable, 1: stretch enable"]
pub type BLC_R3_STRETCH_R = crate::BitReader;
#[doc = "Field `BLC_R3_STRETCH` writer - this bit configures the stretch feature of bottom right channel. 0: stretch disable, 1: stretch enable"]
pub type BLC_R3_STRETCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLC_R2_STRETCH` reader - this bit configures the stretch feature of bottom left channel. 0: stretch disable, 1: stretch enable"]
pub type BLC_R2_STRETCH_R = crate::BitReader;
#[doc = "Field `BLC_R2_STRETCH` writer - this bit configures the stretch feature of bottom left channel. 0: stretch disable, 1: stretch enable"]
pub type BLC_R2_STRETCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLC_R1_STRETCH` reader - this bit configures the stretch feature of top right channel. 0: stretch disable, 1: stretch enable"]
pub type BLC_R1_STRETCH_R = crate::BitReader;
#[doc = "Field `BLC_R1_STRETCH` writer - this bit configures the stretch feature of top right channel. 0: stretch disable, 1: stretch enable"]
pub type BLC_R1_STRETCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLC_R0_STRETCH` reader - this bit configures the stretch feature of top left channel. 0: stretch disable, 1: stretch enable"]
pub type BLC_R0_STRETCH_R = crate::BitReader;
#[doc = "Field `BLC_R0_STRETCH` writer - this bit configures the stretch feature of top left channel. 0: stretch disable, 1: stretch enable"]
pub type BLC_R0_STRETCH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - this bit configures the stretch feature of bottom right channel. 0: stretch disable, 1: stretch enable"]
    #[inline(always)]
    pub fn blc_r3_stretch(&self) -> BLC_R3_STRETCH_R {
        BLC_R3_STRETCH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - this bit configures the stretch feature of bottom left channel. 0: stretch disable, 1: stretch enable"]
    #[inline(always)]
    pub fn blc_r2_stretch(&self) -> BLC_R2_STRETCH_R {
        BLC_R2_STRETCH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - this bit configures the stretch feature of top right channel. 0: stretch disable, 1: stretch enable"]
    #[inline(always)]
    pub fn blc_r1_stretch(&self) -> BLC_R1_STRETCH_R {
        BLC_R1_STRETCH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - this bit configures the stretch feature of top left channel. 0: stretch disable, 1: stretch enable"]
    #[inline(always)]
    pub fn blc_r0_stretch(&self) -> BLC_R0_STRETCH_R {
        BLC_R0_STRETCH_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLC_CTRL0")
            .field("blc_r3_stretch", &self.blc_r3_stretch())
            .field("blc_r2_stretch", &self.blc_r2_stretch())
            .field("blc_r1_stretch", &self.blc_r1_stretch())
            .field("blc_r0_stretch", &self.blc_r0_stretch())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - this bit configures the stretch feature of bottom right channel. 0: stretch disable, 1: stretch enable"]
    #[inline(always)]
    pub fn blc_r3_stretch(&mut self) -> BLC_R3_STRETCH_W<BLC_CTRL0_SPEC> {
        BLC_R3_STRETCH_W::new(self, 0)
    }
    #[doc = "Bit 1 - this bit configures the stretch feature of bottom left channel. 0: stretch disable, 1: stretch enable"]
    #[inline(always)]
    pub fn blc_r2_stretch(&mut self) -> BLC_R2_STRETCH_W<BLC_CTRL0_SPEC> {
        BLC_R2_STRETCH_W::new(self, 1)
    }
    #[doc = "Bit 2 - this bit configures the stretch feature of top right channel. 0: stretch disable, 1: stretch enable"]
    #[inline(always)]
    pub fn blc_r1_stretch(&mut self) -> BLC_R1_STRETCH_W<BLC_CTRL0_SPEC> {
        BLC_R1_STRETCH_W::new(self, 2)
    }
    #[doc = "Bit 3 - this bit configures the stretch feature of top left channel. 0: stretch disable, 1: stretch enable"]
    #[inline(always)]
    pub fn blc_r0_stretch(&mut self) -> BLC_R0_STRETCH_W<BLC_CTRL0_SPEC> {
        BLC_R0_STRETCH_W::new(self, 3)
    }
}
#[doc = "blc stretch control register\n\nYou can [`read`](crate::Reg::read) this register and get [`blc_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blc_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLC_CTRL0_SPEC;
impl crate::RegisterSpec for BLC_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blc_ctrl0::R`](R) reader structure"]
impl crate::Readable for BLC_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blc_ctrl0::W`](W) writer structure"]
impl crate::Writable for BLC_CTRL0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLC_CTRL0 to value 0"]
impl crate::Resettable for BLC_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0;
}
