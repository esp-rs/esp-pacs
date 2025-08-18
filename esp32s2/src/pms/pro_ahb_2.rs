#[doc = "Register `PRO_AHB_2` reader"]
pub type R = crate::R<PRO_AHB_2_SPEC>;
#[doc = "Register `PRO_AHB_2` writer"]
pub type W = crate::W<PRO_AHB_2_SPEC>;
#[doc = "Field `PRO_AHB_RTCSLOW_1_SPLTADDR` reader - Configure the split address of RTCSlow_1 for PeriBus2 access."]
pub type PRO_AHB_RTCSLOW_1_SPLTADDR_R = crate::FieldReader<u16>;
#[doc = "Field `PRO_AHB_RTCSLOW_1_SPLTADDR` writer - Configure the split address of RTCSlow_1 for PeriBus2 access."]
pub type PRO_AHB_RTCSLOW_1_SPLTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PRO_AHB_RTCSLOW_1_L_F` reader - Setting to 1 grants PeriBus2 permission to fetch RTCSlow_1 low address region."]
pub type PRO_AHB_RTCSLOW_1_L_F_R = crate::BitReader;
#[doc = "Field `PRO_AHB_RTCSLOW_1_L_F` writer - Setting to 1 grants PeriBus2 permission to fetch RTCSlow_1 low address region."]
pub type PRO_AHB_RTCSLOW_1_L_F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_AHB_RTCSLOW_1_L_R` reader - Setting to 1 grants PeriBus2 permission to read RTCSlow_1 low address region."]
pub type PRO_AHB_RTCSLOW_1_L_R_R = crate::BitReader;
#[doc = "Field `PRO_AHB_RTCSLOW_1_L_R` writer - Setting to 1 grants PeriBus2 permission to read RTCSlow_1 low address region."]
pub type PRO_AHB_RTCSLOW_1_L_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_AHB_RTCSLOW_1_L_W` reader - Setting to 1 grants PeriBus2 permission to write RTCSlow_1 low address region."]
pub type PRO_AHB_RTCSLOW_1_L_W_R = crate::BitReader;
#[doc = "Field `PRO_AHB_RTCSLOW_1_L_W` writer - Setting to 1 grants PeriBus2 permission to write RTCSlow_1 low address region."]
pub type PRO_AHB_RTCSLOW_1_L_W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_AHB_RTCSLOW_1_H_F` reader - Setting to 1 grants PeriBus2 permission to fetch RTCSlow_1 high address region."]
pub type PRO_AHB_RTCSLOW_1_H_F_R = crate::BitReader;
#[doc = "Field `PRO_AHB_RTCSLOW_1_H_F` writer - Setting to 1 grants PeriBus2 permission to fetch RTCSlow_1 high address region."]
pub type PRO_AHB_RTCSLOW_1_H_F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_AHB_RTCSLOW_1_H_R` reader - Setting to 1 grants PeriBus2 permission to read RTCSlow_1 high address region."]
pub type PRO_AHB_RTCSLOW_1_H_R_R = crate::BitReader;
#[doc = "Field `PRO_AHB_RTCSLOW_1_H_R` writer - Setting to 1 grants PeriBus2 permission to read RTCSlow_1 high address region."]
pub type PRO_AHB_RTCSLOW_1_H_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_AHB_RTCSLOW_1_H_W` reader - Setting to 1 grants PeriBus2 permission to write RTCSlow_1 high address region."]
pub type PRO_AHB_RTCSLOW_1_H_W_R = crate::BitReader;
#[doc = "Field `PRO_AHB_RTCSLOW_1_H_W` writer - Setting to 1 grants PeriBus2 permission to write RTCSlow_1 high address region."]
pub type PRO_AHB_RTCSLOW_1_H_W_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Configure the split address of RTCSlow_1 for PeriBus2 access."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_1_spltaddr(&self) -> PRO_AHB_RTCSLOW_1_SPLTADDR_R {
        PRO_AHB_RTCSLOW_1_SPLTADDR_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Setting to 1 grants PeriBus2 permission to fetch RTCSlow_1 low address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_1_l_f(&self) -> PRO_AHB_RTCSLOW_1_L_F_R {
        PRO_AHB_RTCSLOW_1_L_F_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Setting to 1 grants PeriBus2 permission to read RTCSlow_1 low address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_1_l_r(&self) -> PRO_AHB_RTCSLOW_1_L_R_R {
        PRO_AHB_RTCSLOW_1_L_R_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Setting to 1 grants PeriBus2 permission to write RTCSlow_1 low address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_1_l_w(&self) -> PRO_AHB_RTCSLOW_1_L_W_R {
        PRO_AHB_RTCSLOW_1_L_W_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Setting to 1 grants PeriBus2 permission to fetch RTCSlow_1 high address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_1_h_f(&self) -> PRO_AHB_RTCSLOW_1_H_F_R {
        PRO_AHB_RTCSLOW_1_H_F_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Setting to 1 grants PeriBus2 permission to read RTCSlow_1 high address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_1_h_r(&self) -> PRO_AHB_RTCSLOW_1_H_R_R {
        PRO_AHB_RTCSLOW_1_H_R_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Setting to 1 grants PeriBus2 permission to write RTCSlow_1 high address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_1_h_w(&self) -> PRO_AHB_RTCSLOW_1_H_W_R {
        PRO_AHB_RTCSLOW_1_H_W_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_AHB_2")
            .field(
                "pro_ahb_rtcslow_1_spltaddr",
                &self.pro_ahb_rtcslow_1_spltaddr(),
            )
            .field("pro_ahb_rtcslow_1_l_f", &self.pro_ahb_rtcslow_1_l_f())
            .field("pro_ahb_rtcslow_1_l_r", &self.pro_ahb_rtcslow_1_l_r())
            .field("pro_ahb_rtcslow_1_l_w", &self.pro_ahb_rtcslow_1_l_w())
            .field("pro_ahb_rtcslow_1_h_f", &self.pro_ahb_rtcslow_1_h_f())
            .field("pro_ahb_rtcslow_1_h_r", &self.pro_ahb_rtcslow_1_h_r())
            .field("pro_ahb_rtcslow_1_h_w", &self.pro_ahb_rtcslow_1_h_w())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10 - Configure the split address of RTCSlow_1 for PeriBus2 access."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_1_spltaddr(
        &mut self,
    ) -> PRO_AHB_RTCSLOW_1_SPLTADDR_W<'_, PRO_AHB_2_SPEC> {
        PRO_AHB_RTCSLOW_1_SPLTADDR_W::new(self, 0)
    }
    #[doc = "Bit 11 - Setting to 1 grants PeriBus2 permission to fetch RTCSlow_1 low address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_1_l_f(&mut self) -> PRO_AHB_RTCSLOW_1_L_F_W<'_, PRO_AHB_2_SPEC> {
        PRO_AHB_RTCSLOW_1_L_F_W::new(self, 11)
    }
    #[doc = "Bit 12 - Setting to 1 grants PeriBus2 permission to read RTCSlow_1 low address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_1_l_r(&mut self) -> PRO_AHB_RTCSLOW_1_L_R_W<'_, PRO_AHB_2_SPEC> {
        PRO_AHB_RTCSLOW_1_L_R_W::new(self, 12)
    }
    #[doc = "Bit 13 - Setting to 1 grants PeriBus2 permission to write RTCSlow_1 low address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_1_l_w(&mut self) -> PRO_AHB_RTCSLOW_1_L_W_W<'_, PRO_AHB_2_SPEC> {
        PRO_AHB_RTCSLOW_1_L_W_W::new(self, 13)
    }
    #[doc = "Bit 14 - Setting to 1 grants PeriBus2 permission to fetch RTCSlow_1 high address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_1_h_f(&mut self) -> PRO_AHB_RTCSLOW_1_H_F_W<'_, PRO_AHB_2_SPEC> {
        PRO_AHB_RTCSLOW_1_H_F_W::new(self, 14)
    }
    #[doc = "Bit 15 - Setting to 1 grants PeriBus2 permission to read RTCSlow_1 high address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_1_h_r(&mut self) -> PRO_AHB_RTCSLOW_1_H_R_W<'_, PRO_AHB_2_SPEC> {
        PRO_AHB_RTCSLOW_1_H_R_W::new(self, 15)
    }
    #[doc = "Bit 16 - Setting to 1 grants PeriBus2 permission to write RTCSlow_1 high address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_1_h_w(&mut self) -> PRO_AHB_RTCSLOW_1_H_W_W<'_, PRO_AHB_2_SPEC> {
        PRO_AHB_RTCSLOW_1_H_W_W::new(self, 16)
    }
}
#[doc = "PeriBus2 permission control register 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_ahb_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_ahb_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_AHB_2_SPEC;
impl crate::RegisterSpec for PRO_AHB_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_ahb_2::R`](R) reader structure"]
impl crate::Readable for PRO_AHB_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_ahb_2::W`](W) writer structure"]
impl crate::Writable for PRO_AHB_2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRO_AHB_2 to value 0x0001_f800"]
impl crate::Resettable for PRO_AHB_2_SPEC {
    const RESET_VALUE: u32 = 0x0001_f800;
}
