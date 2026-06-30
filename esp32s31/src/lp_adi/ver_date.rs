#[doc = "Register `VER_DATE` reader"]
pub type R = crate::R<VER_DATE_SPEC>;
#[doc = "Register `VER_DATE` writer"]
pub type W = crate::W<VER_DATE_SPEC>;
#[doc = "Field `VER_DATE` reader - need_des"]
pub type VER_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `VER_DATE` writer - need_des"]
pub type VER_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `CLK_EN` reader - need_des"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - need_des"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - need_des"]
    #[inline(always)]
    pub fn ver_date(&self) -> VER_DATE_R {
        VER_DATE_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VER_DATE")
            .field("ver_date", &self.ver_date())
            .field("clk_en", &self.clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:30 - need_des"]
    #[inline(always)]
    pub fn ver_date(&mut self) -> VER_DATE_W<'_, VER_DATE_SPEC> {
        VER_DATE_W::new(self, 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<'_, VER_DATE_SPEC> {
        CLK_EN_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ver_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ver_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VER_DATE_SPEC;
impl crate::RegisterSpec for VER_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ver_date::R`](R) reader structure"]
impl crate::Readable for VER_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ver_date::W`](W) writer structure"]
impl crate::Writable for VER_DATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VER_DATE to value 0x0025_0704"]
impl crate::Resettable for VER_DATE_SPEC {
    const RESET_VALUE: u32 = 0x0025_0704;
}
