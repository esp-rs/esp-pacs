#[doc = "Register `CAM_CNTL` reader"]
pub type R = crate::R<CAM_CNTL_SPEC>;
#[doc = "Register `CAM_CNTL` writer"]
pub type W = crate::W<CAM_CNTL_SPEC>;
#[doc = "Field `CAM_EN` reader - write 1 to start recive camera data, write 0 to disable"]
pub type CAM_EN_R = crate::BitReader;
#[doc = "Field `CAM_EN` writer - write 1 to start recive camera data, write 0 to disable"]
pub type CAM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_UPDATE` reader - write 1 to update ISP_CAM_CONF"]
pub type CAM_UPDATE_R = crate::BitReader;
#[doc = "Field `CAM_UPDATE` writer - write 1 to update ISP_CAM_CONF"]
pub type CAM_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_RESET` reader - this bit configures cam clk domain reset, 1: reset cam input logic, 0: release reset"]
pub type CAM_RESET_R = crate::BitReader;
#[doc = "Field `CAM_RESET` writer - this bit configures cam clk domain reset, 1: reset cam input logic, 0: release reset"]
pub type CAM_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_CLK_INV` reader - this bit configures the invertion of cam clk from pad. 0: not invert cam clk, 1: invert cam clk"]
pub type CAM_CLK_INV_R = crate::BitReader;
#[doc = "Field `CAM_CLK_INV` writer - this bit configures the invertion of cam clk from pad. 0: not invert cam clk, 1: invert cam clk"]
pub type CAM_CLK_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - write 1 to start recive camera data, write 0 to disable"]
    #[inline(always)]
    pub fn cam_en(&self) -> CAM_EN_R {
        CAM_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - write 1 to update ISP_CAM_CONF"]
    #[inline(always)]
    pub fn cam_update(&self) -> CAM_UPDATE_R {
        CAM_UPDATE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - this bit configures cam clk domain reset, 1: reset cam input logic, 0: release reset"]
    #[inline(always)]
    pub fn cam_reset(&self) -> CAM_RESET_R {
        CAM_RESET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - this bit configures the invertion of cam clk from pad. 0: not invert cam clk, 1: invert cam clk"]
    #[inline(always)]
    pub fn cam_clk_inv(&self) -> CAM_CLK_INV_R {
        CAM_CLK_INV_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAM_CNTL")
            .field("cam_en", &self.cam_en())
            .field("cam_update", &self.cam_update())
            .field("cam_reset", &self.cam_reset())
            .field("cam_clk_inv", &self.cam_clk_inv())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - write 1 to start recive camera data, write 0 to disable"]
    #[inline(always)]
    pub fn cam_en(&mut self) -> CAM_EN_W<CAM_CNTL_SPEC> {
        CAM_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - write 1 to update ISP_CAM_CONF"]
    #[inline(always)]
    pub fn cam_update(&mut self) -> CAM_UPDATE_W<CAM_CNTL_SPEC> {
        CAM_UPDATE_W::new(self, 1)
    }
    #[doc = "Bit 2 - this bit configures cam clk domain reset, 1: reset cam input logic, 0: release reset"]
    #[inline(always)]
    pub fn cam_reset(&mut self) -> CAM_RESET_W<CAM_CNTL_SPEC> {
        CAM_RESET_W::new(self, 2)
    }
    #[doc = "Bit 3 - this bit configures the invertion of cam clk from pad. 0: not invert cam clk, 1: invert cam clk"]
    #[inline(always)]
    pub fn cam_clk_inv(&mut self) -> CAM_CLK_INV_W<CAM_CNTL_SPEC> {
        CAM_CLK_INV_W::new(self, 3)
    }
}
#[doc = "isp cam source control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cam_cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cam_cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAM_CNTL_SPEC;
impl crate::RegisterSpec for CAM_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cam_cntl::R`](R) reader structure"]
impl crate::Readable for CAM_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cam_cntl::W`](W) writer structure"]
impl crate::Writable for CAM_CNTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAM_CNTL to value 0x04"]
impl crate::Resettable for CAM_CNTL_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
