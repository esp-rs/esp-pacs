#[doc = "Register `PRO_IRAM0_3` reader"]
pub type R = crate::R<PRO_IRAM0_3_SPEC>;
#[doc = "Register `PRO_IRAM0_3` writer"]
pub type W = crate::W<PRO_IRAM0_3_SPEC>;
#[doc = "Field `PRO_IRAM0_RTCFAST_SPLTADDR` reader - Configure the split address of RTC FAST for IBUS access."]
pub type PRO_IRAM0_RTCFAST_SPLTADDR_R = crate::FieldReader<u16>;
#[doc = "Field `PRO_IRAM0_RTCFAST_SPLTADDR` writer - Configure the split address of RTC FAST for IBUS access."]
pub type PRO_IRAM0_RTCFAST_SPLTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PRO_IRAM0_RTCFAST_L_F` reader - Setting to 1 grants IBUS permission to fetch RTC FAST low address region."]
pub type PRO_IRAM0_RTCFAST_L_F_R = crate::BitReader;
#[doc = "Field `PRO_IRAM0_RTCFAST_L_F` writer - Setting to 1 grants IBUS permission to fetch RTC FAST low address region."]
pub type PRO_IRAM0_RTCFAST_L_F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_IRAM0_RTCFAST_L_R` reader - Setting to 1 grants IBUS permission to read RTC FAST low address region."]
pub type PRO_IRAM0_RTCFAST_L_R_R = crate::BitReader;
#[doc = "Field `PRO_IRAM0_RTCFAST_L_R` writer - Setting to 1 grants IBUS permission to read RTC FAST low address region."]
pub type PRO_IRAM0_RTCFAST_L_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_IRAM0_RTCFAST_L_W` reader - Setting to 1 grants IBUS permission to write RTC FAST low address region."]
pub type PRO_IRAM0_RTCFAST_L_W_R = crate::BitReader;
#[doc = "Field `PRO_IRAM0_RTCFAST_L_W` writer - Setting to 1 grants IBUS permission to write RTC FAST low address region."]
pub type PRO_IRAM0_RTCFAST_L_W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_IRAM0_RTCFAST_H_F` reader - Setting to 1 grants IBUS permission to fetch RTC FAST high address region."]
pub type PRO_IRAM0_RTCFAST_H_F_R = crate::BitReader;
#[doc = "Field `PRO_IRAM0_RTCFAST_H_F` writer - Setting to 1 grants IBUS permission to fetch RTC FAST high address region."]
pub type PRO_IRAM0_RTCFAST_H_F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_IRAM0_RTCFAST_H_R` reader - Setting to 1 grants IBUS permission to read RTC FAST high address region."]
pub type PRO_IRAM0_RTCFAST_H_R_R = crate::BitReader;
#[doc = "Field `PRO_IRAM0_RTCFAST_H_R` writer - Setting to 1 grants IBUS permission to read RTC FAST high address region."]
pub type PRO_IRAM0_RTCFAST_H_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_IRAM0_RTCFAST_H_W` reader - Setting to 1 grants IBUS permission to write RTC FAST high address region."]
pub type PRO_IRAM0_RTCFAST_H_W_R = crate::BitReader;
#[doc = "Field `PRO_IRAM0_RTCFAST_H_W` writer - Setting to 1 grants IBUS permission to write RTC FAST high address region."]
pub type PRO_IRAM0_RTCFAST_H_W_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Configure the split address of RTC FAST for IBUS access."]
    #[inline(always)]
    pub fn pro_iram0_rtcfast_spltaddr(&self) -> PRO_IRAM0_RTCFAST_SPLTADDR_R {
        PRO_IRAM0_RTCFAST_SPLTADDR_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Setting to 1 grants IBUS permission to fetch RTC FAST low address region."]
    #[inline(always)]
    pub fn pro_iram0_rtcfast_l_f(&self) -> PRO_IRAM0_RTCFAST_L_F_R {
        PRO_IRAM0_RTCFAST_L_F_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Setting to 1 grants IBUS permission to read RTC FAST low address region."]
    #[inline(always)]
    pub fn pro_iram0_rtcfast_l_r(&self) -> PRO_IRAM0_RTCFAST_L_R_R {
        PRO_IRAM0_RTCFAST_L_R_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Setting to 1 grants IBUS permission to write RTC FAST low address region."]
    #[inline(always)]
    pub fn pro_iram0_rtcfast_l_w(&self) -> PRO_IRAM0_RTCFAST_L_W_R {
        PRO_IRAM0_RTCFAST_L_W_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Setting to 1 grants IBUS permission to fetch RTC FAST high address region."]
    #[inline(always)]
    pub fn pro_iram0_rtcfast_h_f(&self) -> PRO_IRAM0_RTCFAST_H_F_R {
        PRO_IRAM0_RTCFAST_H_F_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Setting to 1 grants IBUS permission to read RTC FAST high address region."]
    #[inline(always)]
    pub fn pro_iram0_rtcfast_h_r(&self) -> PRO_IRAM0_RTCFAST_H_R_R {
        PRO_IRAM0_RTCFAST_H_R_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Setting to 1 grants IBUS permission to write RTC FAST high address region."]
    #[inline(always)]
    pub fn pro_iram0_rtcfast_h_w(&self) -> PRO_IRAM0_RTCFAST_H_W_R {
        PRO_IRAM0_RTCFAST_H_W_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_IRAM0_3")
            .field(
                "pro_iram0_rtcfast_spltaddr",
                &self.pro_iram0_rtcfast_spltaddr(),
            )
            .field("pro_iram0_rtcfast_l_f", &self.pro_iram0_rtcfast_l_f())
            .field("pro_iram0_rtcfast_l_r", &self.pro_iram0_rtcfast_l_r())
            .field("pro_iram0_rtcfast_l_w", &self.pro_iram0_rtcfast_l_w())
            .field("pro_iram0_rtcfast_h_f", &self.pro_iram0_rtcfast_h_f())
            .field("pro_iram0_rtcfast_h_r", &self.pro_iram0_rtcfast_h_r())
            .field("pro_iram0_rtcfast_h_w", &self.pro_iram0_rtcfast_h_w())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10 - Configure the split address of RTC FAST for IBUS access."]
    #[inline(always)]
    pub fn pro_iram0_rtcfast_spltaddr(&mut self) -> PRO_IRAM0_RTCFAST_SPLTADDR_W<PRO_IRAM0_3_SPEC> {
        PRO_IRAM0_RTCFAST_SPLTADDR_W::new(self, 0)
    }
    #[doc = "Bit 11 - Setting to 1 grants IBUS permission to fetch RTC FAST low address region."]
    #[inline(always)]
    pub fn pro_iram0_rtcfast_l_f(&mut self) -> PRO_IRAM0_RTCFAST_L_F_W<PRO_IRAM0_3_SPEC> {
        PRO_IRAM0_RTCFAST_L_F_W::new(self, 11)
    }
    #[doc = "Bit 12 - Setting to 1 grants IBUS permission to read RTC FAST low address region."]
    #[inline(always)]
    pub fn pro_iram0_rtcfast_l_r(&mut self) -> PRO_IRAM0_RTCFAST_L_R_W<PRO_IRAM0_3_SPEC> {
        PRO_IRAM0_RTCFAST_L_R_W::new(self, 12)
    }
    #[doc = "Bit 13 - Setting to 1 grants IBUS permission to write RTC FAST low address region."]
    #[inline(always)]
    pub fn pro_iram0_rtcfast_l_w(&mut self) -> PRO_IRAM0_RTCFAST_L_W_W<PRO_IRAM0_3_SPEC> {
        PRO_IRAM0_RTCFAST_L_W_W::new(self, 13)
    }
    #[doc = "Bit 14 - Setting to 1 grants IBUS permission to fetch RTC FAST high address region."]
    #[inline(always)]
    pub fn pro_iram0_rtcfast_h_f(&mut self) -> PRO_IRAM0_RTCFAST_H_F_W<PRO_IRAM0_3_SPEC> {
        PRO_IRAM0_RTCFAST_H_F_W::new(self, 14)
    }
    #[doc = "Bit 15 - Setting to 1 grants IBUS permission to read RTC FAST high address region."]
    #[inline(always)]
    pub fn pro_iram0_rtcfast_h_r(&mut self) -> PRO_IRAM0_RTCFAST_H_R_W<PRO_IRAM0_3_SPEC> {
        PRO_IRAM0_RTCFAST_H_R_W::new(self, 15)
    }
    #[doc = "Bit 16 - Setting to 1 grants IBUS permission to write RTC FAST high address region."]
    #[inline(always)]
    pub fn pro_iram0_rtcfast_h_w(&mut self) -> PRO_IRAM0_RTCFAST_H_W_W<PRO_IRAM0_3_SPEC> {
        PRO_IRAM0_RTCFAST_H_W_W::new(self, 16)
    }
}
#[doc = "IBUS permission control register 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_iram0_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_iram0_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_IRAM0_3_SPEC;
impl crate::RegisterSpec for PRO_IRAM0_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_iram0_3::R`](R) reader structure"]
impl crate::Readable for PRO_IRAM0_3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_iram0_3::W`](W) writer structure"]
impl crate::Writable for PRO_IRAM0_3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRO_IRAM0_3 to value 0x0001_f800"]
impl crate::Resettable for PRO_IRAM0_3_SPEC {
    const RESET_VALUE: u32 = 0x0001_f800;
}
